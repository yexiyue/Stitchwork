# Proposal: Add Realtime Notifications

## Summary

使用 SSE (Server-Sent Events) 实现实时通知推送，配合 Tauri Notification 插件在移动端显示本地通知。

## Motivation

当前系统缺少实时通知机制，用户需要手动刷新才能看到状态变化：
- 员工提交计件后，老板不知道有新的待审核记录
- 老板审批后，员工不知道计件已通过/拒绝
- 工资发放后，员工没有及时收到通知

## Scope

### 通知场景

| 事件 | 发送者 | 接收者 | 通知内容 |
|------|--------|--------|----------|
| 计件提交 | 员工 | 老板 | "{员工名}提交了{工序名}计件 {数量}件" |
| 计件审批通过 | 老板 | 员工 | "计件已通过：{工序名} {数量}件 ¥{金额}" |
| 计件审批拒绝 | 老板 | 员工 | "计件被拒绝：{工序名} {数量}件" |
| 工资发放 | 老板 | 员工 | "收到工资：¥{金额}" |

### 技术方案

- **后端**: Axum SSE 端点 + tokio broadcast channel
- **前端**: EventSource API + Tauri Notification 插件
- **认证**: JWT Token 验证

## Non-Goals

- 离线消息存储（App 关闭时的消息不保留）
- 推送通知（FCM/APNs）
- 消息已读状态追踪

## Dependencies

- `tauri-plugin-notification` (已在 Cargo.toml 中)
- `tokio` broadcast channel
- `axum` SSE support

## Risks

- SSE 连接在移动端可能被系统休眠中断
- 需要处理连接断开后的重连逻辑
