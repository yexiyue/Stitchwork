## 1. 后端：Notifier 服务

- [x] 1.1 添加依赖 `dashmap`, `async-stream` 到 server/Cargo.toml
- [x] 1.2 创建 `server/src/service/notification/mod.rs` 模块
- [x] 1.3 实现 `Notifier` 结构体（DashMap + broadcast channel）
- [x] 1.4 定义 `Notification` 枚举（RecordSubmitted, RecordApproved, RecordRejected, PayrollReceived）
- [x] 1.5 在 `AppState` 中添加 `notifier: Arc<Notifier>`

## 2. 后端：SSE 端点

- [x] 2.1 创建 `server/src/service/notification/controller.rs`
- [x] 2.2 实现 `GET /api/sse/events?token=<jwt>` SSE 端点
- [x] 2.3 添加 token 解析和验证逻辑
- [x] 2.4 实现心跳保活（每 30 秒发送 ping）
- [x] 2.5 在 router 中注册 SSE 路由
- [x] 2.6 运行 `cargo check` 验证编译

## 3. 后端：集成通知触发

- [x] 3.1 更新 `piece_record/controller.rs::create()` - 发送 RecordSubmitted 给老板
- [x] 3.2 更新 `piece_record/controller.rs::approve()` - 发送 RecordApproved 给员工
- [x] 3.3 更新 `piece_record/controller.rs::reject()` - 发送 RecordRejected 给员工
- [x] 3.4 更新 `piece_record/controller.rs::batch_approve()` - 发送批量通知
- [x] 3.5 更新 `piece_record/controller.rs::batch_reject()` - 发送批量通知
- [x] 3.6 更新 `payroll/controller.rs::create()` - 发送 PayrollReceived 给员工
- [x] 3.7 运行 `cargo check` 验证编译

## 4. Tauri：SSE 客户端 (Rust 原生层)

- [x] 4.1 添加依赖 `reqwest-eventsource` 到 src-tauri/Cargo.toml
- [x] 4.2 创建 `src-tauri/src/sse.rs` SSE 客户端模块
- [x] 4.3 实现 `start_sse()` 异步函数（连接、接收、重连）
- [x] 4.4 实现 `connect_sse` Tauri command
- [x] 4.5 实现 `disconnect_sse` Tauri command（取消任务）
- [x] 4.6 收到消息时调用 `tauri_plugin_notification` 发送本地通知
- [x] 4.7 收到消息时 `emit("notification", payload)` 通知前端
- [x] 4.8 在 `lib.rs` 注册 commands
- [x] 4.9 运行 `cargo check` 验证编译

## 5. 前端：通知监听

- [x] 5.1 安装 `@tauri-apps/plugin-notification` npm 包 (已存在)
- [x] 5.2 创建 `src/hooks/useNotify.ts`
- [x] 5.3 实现登录后调用 `invoke('connect_sse')`
- [x] 5.4 实现 `listen('notification')` 监听事件
- [x] 5.5 收到通知时显示 Toast
- [x] 5.6 收到通知时刷新相关 query（piece-records, home-overview）
- [x] 5.7 登出时调用 `invoke('disconnect_sse')`

## 6. 前端：集成

- [x] 6.1 在 `_auth.tsx` 布局中调用 `useNotify` hook
- [x] 6.2 运行 `pnpm tsc --noEmit` 验证类型

## 7. 验证

- [x] 7.1 运行 `pnpm build` 验证前端构建
- [x] 7.2 运行 `cargo check` 验证后端和 Tauri 编译
- [ ] 7.3 手动测试：员工提交计件 → 老板收到通知
- [ ] 7.4 手动测试：老板审批 → 员工收到通知
- [ ] 7.5 手动测试：老板发工资 → 员工收到通知
- [ ] 7.6 手动测试：App 后台时通知是否正常
