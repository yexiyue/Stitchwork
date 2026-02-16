# 实时通知系统实现

基于 SSE (Server-Sent Events) 实现的实时消息推送系统，用于计件审批、工资发放、用户注册等业务场景的即时通知。

## 架构概览

```mermaid
flowchart LR
    subgraph Backend["Backend (Axum)"]
        Notifier["Notifier<br/>(broadcast)"]
    end

    subgraph Tauri["Tauri Rust"]
        SSE_T["SSE Client<br/>(Authorization header)"]
        Notify["Local Notification<br/>(system tray)"]
    end

    subgraph Browser["Browser"]
        SSE_B["EventSource<br/>(?token=xxx)"]
    end

    subgraph Frontend["Frontend (React)"]
        Hook["useNotify Hook"]
        Toast["Toast + Query<br/>Invalidation"]
    end

    Backend -->|SSE| SSE_T
    Backend -->|SSE| SSE_B
    SSE_T --> Notify
    SSE_T -->|"emit()"| Hook
    SSE_B --> Hook
    Hook --> Toast
```

**双环境支持：**

| 环境 | SSE 客户端 | 认证方式 | 通知方式 |
|------|-----------|---------|---------|
| Tauri | Rust `reqwest_eventsource` | `Authorization: Bearer <token>` | 系统通知 + Toast |
| 浏览器 | 原生 `EventSource` | `?token=<jwt>` (query param) | Toast |

**为什么 Tauri 用 Rust SSE 客户端？**

- App 后台时 WebView 可能暂停，Rust 层持续运行
- 可直接调用系统通知 API（`tauri-plugin-notification`）
- 支持 Authorization header（原生 EventSource 不支持）
- 自动重连逻辑更可控

## 通知类型

```rust
pub enum Notification {
    // 计件相关
    RecordSubmitted { user_name, process_name, quantity }  // 员工提交 → 老板
    RecordApproved { process_name, quantity, amount }      // 老板通过 → 员工
    RecordRejected { process_name, quantity }              // 老板拒绝 → 员工
    PayrollReceived { amount }                             // 发工资 → 员工

    // 用户注册
    UserRegistered { username, phone }                     // 新用户注册 → 超管
    StaffJoined { username, phone }                        // 员工加入 → 老板
}
```

## 后端实现

### 1. Notifier 服务 (`server/src/service/notification/mod.rs`)

使用 `DashMap` + `tokio::broadcast` 管理多用户订阅：

```rust
pub struct Notifier {
    channels: DashMap<Uuid, broadcast::Sender<Notification>>,
}

impl Notifier {
    /// 用户订阅 - SSE 连接时调用
    pub fn subscribe(&self, user_id: Uuid) -> broadcast::Receiver<Notification> {
        self.channels
            .entry(user_id)
            .or_insert_with(|| broadcast::channel(16).0)
            .subscribe()
    }

    /// 发送通知给单个用户
    pub fn send(&self, user_id: Uuid, notification: Notification) {
        if let Some(sender) = self.channels.get(&user_id) {
            let _ = sender.send(notification);
        }
    }

    /// 发送通知给多个用户
    pub fn send_many(&self, user_ids: &[Uuid], notification: Notification) {
        for user_id in user_ids {
            self.send(*user_id, notification.clone());
        }
    }
}
```

**关键点：**
- 每个用户一个 broadcast channel，容量 16
- 懒初始化：首次订阅时创建 channel
- `send()` 忽略无订阅者的情况（用户不在线）
- `send_many()` 用于通知多个用户（如所有超管）

### 2. SSE 端点 (`server/src/service/notification/controller.rs`)

```rust
/// GET /api/sse/events
/// 认证方式（优先级从高到低）：
/// 1. Authorization: Bearer <token>
/// 2. ?token=<jwt>
async fn sse_events(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Query(query): Query<SseQuery>,
) -> Result<Sse<impl Stream<Item = Result<Event, Infallible>>>, AppError> {
    // 优先从 Authorization header 获取 token
    let token = headers
        .get("authorization")
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer "))
        .map(|s| s.to_string())
        .or(query.token)
        .ok_or(AppError::Unauthorized)?;

    let claims = verify_token(&token)?;
    let mut rx = state.notifier.subscribe(claims.sub);

    let stream = async_stream::stream! {
        loop {
            match rx.recv().await {
                Ok(notification) => {
                    yield Ok(Event::default().data(json));
                }
                Err(RecvError::Lagged(_)) => continue,  // 消息积压，跳过
                Err(RecvError::Closed) => break,
            }
        }
    };

    // 30 秒心跳保活
    Ok(Sse::new(stream).keep_alive(
        KeepAlive::new().interval(Duration::from_secs(30)).text("ping")
    ))
}
```

**关键点：**
- 支持 Authorization header（Tauri）和 query parameter（浏览器）两种认证
- 30 秒心跳防止连接超时
- `Lagged` 错误时跳过积压消息继续接收

### 3. 触发通知

在业务 Service 中调用 `notifier.send()`：

```rust
// auth/service.rs - 老板注册
pub async fn register(db, notifier, req) -> Result<Uuid> {
    let user = create_user(db, req).await?;

    // 通知所有超管
    let super_admins = user::Entity::find()
        .filter(user::Column::IsSuperAdmin.eq(true))
        .all(db).await?;
    let admin_ids: Vec<Uuid> = super_admins.iter().map(|u| u.id).collect();
    notifier.send_many(&admin_ids, Notification::UserRegistered {
        username: user.username.clone(),
        phone: user.phone.clone(),
    });

    Ok(user.id)
}

// auth/service.rs - 员工注册
pub async fn register_staff(db, invite_codes, notifier, req) -> Result<LoginResponse> {
    let user = create_staff(db, req).await?;

    // 通知工坊老板
    if let Some(ref workshop) = ws {
        notifier.send(workshop.owner_id, Notification::StaffJoined {
            username: user.username.clone(),
            phone: user.phone.clone(),
        });
    }

    Ok(response)
}

// piece_record/controller.rs - 员工提交计件
pub async fn create(...) {
    let record = service::create(db, dto, claims.sub).await?;

    // 通知老板
    state.notifier.send(record.boss_id, Notification::RecordSubmitted {
        user_name: claims.display_name.clone(),
        process_name: process.name.clone(),
        quantity: record.quantity,
    });
}
```

## Tauri 客户端

### SSE 客户端 (`src-tauri/src/sse.rs`)

```rust
async fn start_sse(app_handle, api_url, token, cancel_rx) {
    let url = format!("{}/api/sse/events", api_url);
    let client = reqwest::Client::new();

    loop {
        let request = client
            .get(&url)
            .header("Authorization", format!("Bearer {}", token));
        let mut es = EventSource::new(request).unwrap();

        loop {
            tokio::select! {
                // 监听取消信号
                _ = cancel_rx.changed() => { return; }

                // 接收 SSE 事件
                event = es.next() => {
                    match event {
                        Some(Ok(Event::Message(msg))) => {
                            // 1. 发送系统通知
                            app_handle.notification()
                                .builder()
                                .title(&payload.title)
                                .body(&payload.body)
                                .show();

                            // 2. 通知前端
                            app_handle.emit("notification", &payload);
                        }
                        Some(Err(_)) => break,  // 错误，退出内层循环
                        None => break,          // 流结束
                    }
                }
            }
        }

        // 5 秒后重连
        tokio::time::sleep(Duration::from_secs(5)).await;
    }
}
```

**关键点：**
- 使用 Authorization header 认证（更安全，token 不暴露在 URL）
- 自动重连（5 秒间隔）
- `tokio::select!` 同时监听取消信号和 SSE 事件
- 双重通知：系统托盘 + 前端事件

### Tauri Commands

```rust
#[tauri::command]
pub async fn connect_sse(app_handle, state, api_url, token) -> Result<(), String>

#[tauri::command]
pub async fn disconnect_sse(state) -> Result<(), String>
```

## 前端集成

### useNotify Hook (`src/hooks/useNotify.ts`)

支持 Tauri 和浏览器两种环境：

```typescript
export function useNotify() {
  const token = useAuthStore((s) => s.token);
  const queryClient = useQueryClient();

  // Tauri 环境
  useEffect(() => {
    if (!token || !isTauri()) return;

    const setup = async () => {
      await invoke("connect_sse", { apiUrl, token, channelId: CHANNEL_ID });

      unlisten = await listen<NotificationPayload>("notification", (event) => {
        handleNotification(event.payload, queryClient);
      });
    };

    setup();
    return () => {
      invoke("disconnect_sse");
      unlisten?.();
    };
  }, [token]);

  // 浏览器环境
  useEffect(() => {
    if (!token || isTauri()) return;

    // 浏览器 EventSource 不支持自定义 headers，使用 query parameter
    const url = `${apiUrl}/api/sse/events?token=${encodeURIComponent(token)}`;
    let eventSource = new EventSource(url);

    eventSource.onmessage = (event) => {
      const payload = JSON.parse(event.data);
      handleNotification(payload, queryClient);
    };

    eventSource.onerror = () => {
      eventSource.close();
      // 5 秒后重连
      setTimeout(connect, 5000);
    };

    return () => eventSource.close();
  }, [token]);
}

function handleNotification(payload, queryClient) {
  Toast.show({ content: payload.title });

  switch (payload.type) {
    case "record_submitted":
      queryClient.invalidateQueries({ queryKey: ["piece-records"] });
      break;
    case "user_registered":
      queryClient.invalidateQueries({ queryKey: ["admin", "users"] });
      break;
    case "staff_joined":
      queryClient.invalidateQueries({ queryKey: ["staff"] });
      break;
    // ...
  }
}
```

### 集成位置

在 `_auth.tsx` 布局组件中调用：

```tsx
function AuthLayout() {
  useNotify();  // 登录后自动启用
  return <Outlet />;
}
```

## 依赖

### 后端 (server/Cargo.toml)
```toml
dashmap = "6"
async-stream = "0.3"
```

### Tauri (src-tauri/Cargo.toml)
```toml
reqwest-eventsource = "0.6"
tokio = { version = "1", features = ["sync", "macros", "time"] }
log = "0.4"
```

### 前端
```bash
pnpm add @tauri-apps/plugin-notification
```

## 数据流

```mermaid
flowchart TD
    A["用户注册/员工提交"] --> B["service function"]
    B --> C["notifier.send()"]
    C --> D["broadcast channel"]
    D --> E["SSE stream"]

    E --> F{"环境?"}
    F -->|Tauri| G["Rust SSE client"]
    F -->|Browser| H["EventSource"]

    G --> I["system notification"]
    G --> J["emit('notification')"]
    H --> K["useNotify hook"]
    J --> K

    K --> L["Toast.show()"]
    K --> M["invalidateQueries()"]
```

## 注意事项

1. **Token 安全**：
   - Tauri 使用 Authorization header，更安全
   - 浏览器使用 query parameter（EventSource 限制），日志中注意脱敏

2. **重连策略**：当前固定 5 秒，可优化为指数退避

3. **消息丢失**：broadcast channel 容量 16，超出会 lag，当前策略是跳过

4. **Android 后台**：需要前台服务保持 App 活跃，否则可能被杀

5. **浏览器兼容**：原生 EventSource 不支持自定义 headers，只能用 query parameter
