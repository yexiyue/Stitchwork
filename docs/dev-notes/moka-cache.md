# Moka 缓存使用指南

Moka 是 Rust 高性能并发缓存库，支持 TTL 自动过期、并发安全、异步操作。

## 依赖

```toml
moka = { version = "0.12", features = ["future"] }
```

## 基本用法

### 创建缓存

```rust
use moka::future::Cache;
use std::time::Duration;

// 带 TTL 的缓存
let cache: Cache<String, i32> = Cache::builder()
    .time_to_idle(Duration::from_secs(30 * 60))  // 30 分钟无访问过期
    .time_to_live(Duration::from_secs(60 * 60))  // 1 小时强制过期
    .max_capacity(1000)                           // 最大容量
    .build();
```

### 基本操作

```rust
// 插入
cache.insert("key".to_string(), 42).await;

// 获取
if let Some(value) = cache.get(&"key".to_string()).await {
    println!("{}", value);
}

// 删除
cache.remove(&"key".to_string()).await;
```

### get_with - 获取或插入

```rust
// 如果 key 不存在，执行闭包初始化
let value = cache.get_with("key".to_string(), async {
    expensive_computation().await
}).await;
```

### try_get_with - 支持错误处理

```rust
use anyhow::Result;

let result: Result<i32, Arc<anyhow::Error>> = cache
    .try_get_with("key".to_string(), async {
        fallible_computation().await
    })
    .await;
```

## 项目实践：SessionManager

### 实现

```rust
// chat/session_manager.rs
use std::sync::Arc;
use std::time::Duration;
use anyhow::Result;
use moka::future::Cache;
use uuid::Uuid;

#[derive(Clone)]
pub struct SessionManager {
    sessions: Cache<Uuid, Arc<ChatSession>>,
}

impl SessionManager {
    pub fn new() -> Self {
        Self {
            sessions: Cache::builder()
                .time_to_idle(Duration::from_secs(30 * 60)) // 30 分钟无访问自动过期
                .build(),
        }
    }

    pub async fn get_or_try_insert<F, Fut>(&self, id: Uuid, init: F) -> Result<Arc<ChatSession>>
    where
        F: FnOnce() -> Fut,
        Fut: std::future::Future<Output = Result<ChatSession>>,
    {
        self.sessions
            .try_get_with(id, async { Ok(Arc::new(init().await?)) })
            .await
            .map_err(|e: Arc<anyhow::Error>| anyhow::anyhow!("{}", e))
    }
}
```

### 使用

```rust
// service/chat.rs
let chat_session = app_state
    .session_manager
    .get_or_try_insert(req.id, || {
        ChatSession::new(&app_state.db, &app_state.rig_client, claims)
    })
    .await?;
```

## TTL 策略

| 策略 | 说明 |
|------|------|
| `time_to_idle` | 最后访问后多久过期（推荐用于 session） |
| `time_to_live` | 插入后多久强制过期 |

## sync vs future

| 版本 | 特点 |
|------|------|
| `moka::sync::Cache` | 同步 API，`get`/`insert` 不需要 `.await` |
| `moka::future::Cache` | 异步 API，适合 tokio 环境，支持 async 初始化闭包 |

## 并发安全

`try_get_with` 自动处理并发：多个请求同时访问不存在的 key 时，只有一个执行初始化，其他等待复用结果。

## 参考

- [moka 文档](https://docs.rs/moka/latest/moka/)
- [moka GitHub](https://github.com/moka-rs/moka)
