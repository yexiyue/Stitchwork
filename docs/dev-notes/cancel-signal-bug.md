# CancelSignal Bugs 分析

> 版本：rig-core 0.28.0
> 文件位置：`rig/rig-core/src/agent/prompt_request/mod.rs` 第 161-200 行

---

## 问题概述

`CancelSignal` 结构体存在两个 bug，会导致 Hook 机制无法正常工作：

1. **`cancel_with_reason()` 不设置 cancelled 标志** - 调用后 `is_cancelled()` 仍返回 false
2. **`Clone::clone()` 丢失 reason** - 由于 `OnceLock` 的 clone 语义，克隆会丢失已设置的 reason

---

## 当前实现

```rust
pub struct CancelSignal {
    sig: Arc<AtomicBool>,
    reason: OnceLock<String>,
}

impl CancelSignal {
    pub fn cancel(&self) {
        self.sig.store(true, Ordering::SeqCst);
    }

    pub fn cancel_with_reason(&self, reason: &str) {
        // ❌ Bug 1: 只设置 reason，不设置 sig
        let _ = self.reason.set(reason.to_string());
    }

    fn is_cancelled(&self) -> bool {
        self.sig.load(Ordering::SeqCst)
    }

    fn cancel_reason(&self) -> Option<&str> {
        self.reason.get().map(|x| x.as_str())
    }
}

impl Clone for CancelSignal {
    fn clone(&self) -> Self {
        Self {
            sig: self.sig.clone(),
            // ❌ Bug 2: OnceLock::clone() 返回空的 OnceLock
            reason: self.reason.clone(),
        }
    }
}
```

---

## Bug 1: `cancel_with_reason()` 不设置 cancelled 标志

### 问题描述

`cancel_with_reason()` 方法只设置了 `reason`，但没有调用 `cancel()` 或直接设置 `sig` 为 true。

### 复现代码

```rust
let cancel_sig = CancelSignal::new();

// 用户期望：调用 cancel_with_reason 会同时取消
cancel_sig.cancel_with_reason("frontend_tool");

// 实际结果：is_cancelled() 仍然返回 false！
assert!(!cancel_sig.is_cancelled()); // ✅ 通过，说明 cancel 没生效
assert_eq!(cancel_sig.cancel_reason(), Some("frontend_tool")); // ✅ reason 被设置了
```

### 影响

在 `StreamingPromptHook::on_tool_call` 中使用 `cancel_with_reason()` 后：

```rust
impl<M> StreamingPromptHook<M> for FrontendToolHook {
    async fn on_tool_call(&self, tool_name: &str, ..., cancel_sig: CancelSignal) {
        if self.frontend_tools.contains(tool_name) {
            // ❌ 这不会真正取消循环！
            cancel_sig.cancel_with_reason("frontend_tool");
        }
    }
}
```

agent 循环不会中断，因为 `is_cancelled()` 返回 false：

```rust
// streaming.rs 中的检查
if cancel_sig.is_cancelled() {  // ❌ 永远是 false
    return Err(PromptCancelled);
}
// 工具会继续执行...
```

### 修复方案

```rust
pub fn cancel_with_reason(&self, reason: &str) {
    let _ = self.reason.set(reason.to_string());
    self.cancel();  // ✅ 添加这一行
}
```

---

## Bug 2: `Clone::clone()` 后 reason 不共享

### 问题描述

`OnceLock<T>` 的 `clone()` 实现是正确的——如果原 OnceLock 已有值，clone 后的也会有值：

```rust
impl<T: Clone> Clone for OnceLock<T> {
    fn clone(&self) -> OnceLock<T> {
        let cell = Self::new();
        if let Some(value) = self.get() {
            match cell.set(value.clone()) {  // 如果有值，复制过去
                Ok(()) => (),
                Err(_) => unreachable!(),
            }
        }
        cell
    }
}
```

**真正的问题**：clone 后是**两个独立的 OnceLock 实例**，不是共享同一份。后续对其中一个设置值，另一个看不到。

### 当前的 Clone 实现

```rust
impl Clone for CancelSignal {
    fn clone(&self) -> Self {
        Self {
            sig: self.sig.clone(),       // ✅ Arc clone → 共享同一个 AtomicBool
            reason: self.reason.clone(), // ❌ OnceLock clone → 两个独立实例
        }
    }
}
```

- `sig: Arc<AtomicBool>` clone 后仍然**共享同一个** AtomicBool
- `reason: OnceLock<String>` clone 后变成**两个独立的** OnceLock

### 复现代码

```rust
let cancel_sig = CancelSignal::new();  // reason 是空的
let cloned = cancel_sig.clone();       // clone 时 reason 也是空的，两个独立的空 OnceLock

// 在 clone 的副本上设置 reason（hook 里就是这样）
cloned.cancel_with_reason("frontend_tool");
cloned.cancel();

// sig 是共享的，两边都能看到
assert!(cancel_sig.is_cancelled());  // ✅ 通过
assert!(cloned.is_cancelled());      // ✅ 通过

// reason 不是共享的！
assert_eq!(cloned.cancel_reason(), Some("frontend_tool"));  // ✅ clone 的有值
assert_eq!(cancel_sig.cancel_reason(), None);  // ❌ 原始的是 None！
```

### 影响

在 agent 循环中，`CancelSignal` 被 clone 传递给 hook：

```rust
// streaming.rs
if let Some(ref hook) = self.hook {
    // clone 传入 hook，hook 里设置 reason 是设置到 clone 的副本上
    hook.on_tool_call(..., cancel_sig.clone()).await;

    if cancel_sig.is_cancelled() {
        // 这里读的是原始 cancel_sig，它的 reason 是 None！
        return Err(PromptError::prompt_cancelled(
            chat_history,
            cancel_sig.cancel_reason().unwrap_or("<no reason given>"),
            //         ^^^^^^^^^^^^^^^^^ 永远返回 None，因为设置的是 clone 的那份
        ));
    }
}
```

结果：`cancel_reason()` 永远返回 `"<no reason given>"`，即使 hook 里设置了 reason。

### 修复方案

将 `reason` 也改为 `Arc` 包装：

```rust
pub struct CancelSignal {
    sig: Arc<AtomicBool>,
    reason: Arc<OnceLock<String>>,  // ✅ 改用 Arc 包装
}

impl Clone for CancelSignal {
    fn clone(&self) -> Self {
        Self {
            sig: self.sig.clone(),
            reason: self.reason.clone(),  // 现在是 Arc clone，保持共享
        }
    }
}

impl CancelSignal {
    fn new() -> Self {
        Self {
            sig: Arc::new(AtomicBool::new(false)),
            reason: Arc::new(OnceLock::new()),  // 改为 Arc 包装
        }
    }

    pub fn cancel_with_reason(&self, reason: &str) {
        let _ = self.reason.set(reason.to_string());
        self.cancel();  // 同时修复 Bug 1
    }

    // 其他方法不需要改动
}
```

---

## 完整修复 Diff

```diff
 pub struct CancelSignal {
     sig: Arc<AtomicBool>,
-    reason: OnceLock<String>,
+    reason: Arc<OnceLock<String>>,
 }

 impl CancelSignal {
     fn new() -> Self {
         Self {
             sig: Arc::new(AtomicBool::new(false)),
-            reason: OnceLock::new(),
+            reason: Arc::new(OnceLock::new()),
         }
     }

     pub fn cancel(&self) {
         self.sig.store(true, Ordering::SeqCst);
     }

     pub fn cancel_with_reason(&self, reason: &str) {
-        // SAFETY: This can only be set once. We immediately return once the prompt hook is finished if the internal AtomicBool is set to true
-        // It is technically on the user to return early when using this in a prompt hook, but this is relatively obvious
         let _ = self.reason.set(reason.to_string());
+        self.cancel();
     }

     fn is_cancelled(&self) -> bool {
         self.sig.load(Ordering::SeqCst)
     }

     fn cancel_reason(&self) -> Option<&str> {
         self.reason.get().map(|x| x.as_str())
     }
 }

-impl Clone for CancelSignal {
-    fn clone(&self) -> Self {
-        Self {
-            sig: self.sig.clone(),
-            reason: self.reason.clone(),
-        }
-    }
-}
+// 现在可以直接 derive Clone，因为 Arc<T> 的 clone 是共享语义
+// 或者保持手动实现也可以
```

---

## 测试用例建议

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cancel_with_reason_sets_cancelled_flag() {
        let sig = CancelSignal::new();

        sig.cancel_with_reason("test reason");

        assert!(sig.is_cancelled(), "cancel_with_reason should set cancelled flag");
        assert_eq!(sig.cancel_reason(), Some("test reason"));
    }

    #[test]
    fn test_clone_shares_reason() {
        let sig = CancelSignal::new();
        let cloned = sig.clone();

        sig.cancel_with_reason("shared reason");

        // Both should see the same reason
        assert_eq!(sig.cancel_reason(), Some("shared reason"));
        assert_eq!(cloned.cancel_reason(), Some("shared reason"),
            "cloned signal should share the same reason");
    }

    #[test]
    fn test_clone_shares_cancelled_state() {
        let sig = CancelSignal::new();
        let cloned = sig.clone();

        sig.cancel();

        assert!(sig.is_cancelled());
        assert!(cloned.is_cancelled(), "cloned signal should share cancelled state");
    }
}
```

---

## 发现背景

在实现 **前端工具（Frontend Tools）** 功能时发现此问题。流程如下：

1. 后端通过 `StreamingPromptHook::on_tool_call` 拦截前端工具
2. Hook 调用 `cancel_sig.cancel_with_reason("frontend_tool")` 中断循环
3. 期望 agent 循环检测到取消并返回 `PromptCancelled`
4. **实际**：`is_cancelled()` 返回 false，循环继续执行后端工具

---

## 参考

- [rig 源码 - prompt_request/mod.rs](https://github.com/0xPlaygrounds/rig/blob/main/rig/rig-core/src/agent/prompt_request/mod.rs)
- [std::sync::OnceLock 文档](https://doc.rust-lang.org/std/sync/struct.OnceLock.html)
- [AI SDK Frontend Tools](https://sdk.vercel.ai/docs/ai-sdk-ui/chatbot-with-tool-calling#frontend-tools)
