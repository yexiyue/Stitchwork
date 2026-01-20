# Rust 2024 Edition: `use<>` 精确捕获语法

> 参考：[The Rust Reference - Precise Capturing](https://doc.rust-lang.org/reference/types/impl-trait.html#precise-capturing)

## 问题背景

在 Rust 2024 edition 中，`impl Trait` 返回类型默认会**自动捕获**函数签名中的所有泛型参数（包括类型、const 和生命周期参数）。

### 2024 Edition 变化

> Before the 2024 edition, on free functions and on associated functions and methods of inherent impls, generic lifetime parameters that do not appear in the bounds of the abstract return type are not automatically captured.

即：2024 之前，不在返回类型 bounds 中出现的生命周期参数不会被自动捕获；2024 开始，所有参数都会被自动捕获。

### 典型问题场景

```rust
// Rust 2024 edition
pub async fn chat(
    &self,
    prompt: Message,
    history: Vec<Message>,
) -> impl Stream<Item = Result<Event, Infallible>> {
    // 返回的 Stream 隐式捕获了 &self 的生命周期
    let stream = self.agent.stream_chat(prompt, history).await;
    adapt_rig_stream_sse(stream)
}
```

调用时会报错：

```
error: `chat_session` does not live long enough
  --> src/service/chat.rs:47:18
   |
47 |     let stream = chat_session.chat(prompt, history).await;
   |                  ^^^^^^^^^^^^ borrowed value does not live long enough
```

原因：返回的 `impl Stream` 被认为持有 `&self` 的引用，但实际上 stream 内部并不需要它。

## Precise Capturing: `use<>` 语法

`use<..>` bound 可以显式控制 `impl Trait` 捕获哪些泛型参数。

### 语法格式

```rust
fn example<'a, 'b, T>(x: &'a (), y: T) -> impl Sized + use<'a, T> {
    //                                     ~~~~~~~~~~~~~~~~~~~~~~~
    //                                     只捕获 'a 和 T
    (x, y)
}
```

### 语法约束

根据官方文档：

1. **只能有一个 `use<..>` bound**：bounds 列表中只能出现一个
2. **必须包含所有类型和 const 参数**：所有 in-scope 的类型和 const 泛型参数必须被包含
3. **必须包含 bounds 中出现的生命周期**：其他 bounds 中出现的生命周期参数必须被包含
4. **生命周期参数在前**：`use<..>` 中生命周期参数必须在类型和 const 参数之前
5. **不能与 argument-position impl Trait 一起使用**：因为那些有匿名类型参数

### 常见用法

```rust
// 1. 不捕获任何生命周期参数（返回 'static 类型）
fn foo(&self) -> impl Future<Output = ()> + use<> {
    async {}
}

// 2. 只捕获特定生命周期
fn bar<'a>(&self, data: &'a str) -> impl Future<Output = &'a str> + use<'a> {
    async move { data }
}

// 3. 捕获类型参数（必须包含所有类型参数）
fn baz<T: Clone>(&self, val: T) -> impl Future<Output = T> + use<T> {
    async move { val }
}

// 4. 混合捕获（生命周期在前）
fn qux<'a, T>(&self, data: &'a T) -> impl Iterator<Item = &'a T> + use<'a, T> {
    std::iter::once(data)
}
```

## 实际应用

### 修复 Stream 生命周期问题

```rust
// 修复前：Stream 隐式捕获 &self
pub async fn chat(
    &self,
    prompt: Message,
    history: Vec<Message>,
) -> impl Stream<Item = Result<Event, Infallible>> {
    // ...
}

// 修复后：Stream 不捕获任何生命周期
pub async fn chat(
    &self,
    prompt: Message,
    history: Vec<Message>,
) -> impl Stream<Item = Result<Event, Infallible>> + use<> {
    // 返回的 stream 是 'static，可以独立于 &self 存在
    let stream = self.agent.stream_chat(prompt, history).await;
    adapt_rig_stream_sse(stream)
}
```

### 修复 Hook trait 实现

```rust
// 修复前
fn on_tool_call(
    &self,
    tool_name: &str,
    args: &str,
    cancel_sig: CancelSignal,
) -> impl Future<Output = ()> + Send {
    // Future 隐式捕获 &self, tool_name, args
    async {}
}

// 修复后
fn on_tool_call(
    &self,
    tool_name: &str,
    args: &str,
    cancel_sig: CancelSignal,
) -> impl Future<Output = ()> + Send + use<> {
    // Future 不捕获任何引用
    if self.is_frontend_tool(tool_name) {
        cancel_sig.cancel();
    }
    async {}
}
```

## 与 Rust 2021 的对比

| Edition | 默认行为 | 控制方式 |
|---------|---------|---------|
| Rust 2021 | 只捕获 bounds 中出现的生命周期 | 无法精确控制 |
| Rust 2024 | 捕获所有 in-scope 泛型参数 | `use<>` 精确指定 |

## 何时使用 `use<>`

1. **返回的类型实际上是 `'static`**：内部不持有任何借用
2. **避免不必要的生命周期约束**：让返回值可以独立存活
3. **trait 方法实现**：当 trait 定义期望特定的生命周期行为时

## 注意事项

- `use<>` 只能用于 `impl Trait` 返回类型
- 如果返回值确实需要某个生命周期，必须在 `use<>` 中列出
- 遗漏必要的生命周期会导致编译错误
- 所有类型和 const 参数必须被包含（不能省略）

## 参考资料

- [The Rust Reference: Precise Capturing](https://doc.rust-lang.org/reference/types/impl-trait.html#precise-capturing)
- [RFC 3498: Lifetime Capture Rules 2024](https://rust-lang.github.io/rfcs/3498-lifetime-capture-rules-2024.html)
