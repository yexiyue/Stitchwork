## ADDED Requirements

### Requirement: SSE Notification Endpoint

系统 SHALL 提供 SSE 端点用于实时推送通知。

#### Scenario: 用户订阅通知

- **GIVEN** 用户已登录并持有有效 JWT token
- **WHEN** 用户连接 `GET /api/sse/events?token=<jwt>`
- **THEN** 服务器返回 `text/event-stream` 响应
- **AND** 连接保持打开状态等待通知

#### Scenario: 无效 token 拒绝连接

- **GIVEN** 请求携带无效或过期的 token
- **WHEN** 用户尝试连接 SSE 端点
- **THEN** 服务器返回 401 Unauthorized

#### Scenario: 心跳保活

- **GIVEN** SSE 连接已建立
- **WHEN** 30 秒内无消息
- **THEN** 服务器发送心跳事件保持连接

### Requirement: Record Submission Notification

系统 SHALL 在计件提交时通知老板。

#### Scenario: 员工提交计件

- **GIVEN** 员工提交计件记录
- **WHEN** 记录创建成功
- **THEN** 老板收到通知 "{员工名}提交了{工序名}计件 {数量}件"

### Requirement: Record Approval Notification

系统 SHALL 在计件审批后通知员工。

#### Scenario: 计件审批通过

- **GIVEN** 老板审批计件记录
- **WHEN** 状态变更为 Approved
- **THEN** 员工收到通知 "计件已通过：{工序名} {数量}件 ¥{金额}"

#### Scenario: 计件审批拒绝

- **GIVEN** 老板审批计件记录
- **WHEN** 状态变更为 Rejected
- **THEN** 员工收到通知 "计件被拒绝：{工序名} {数量}件"

#### Scenario: 批量审批通知

- **GIVEN** 老板批量审批多条计件记录
- **WHEN** 批量审批完成
- **THEN** 每个相关员工收到各自的通知

### Requirement: Payroll Notification

系统 SHALL 在工资发放时通知员工。

#### Scenario: 工资发放

- **GIVEN** 老板创建工资单
- **WHEN** 工资单创建成功
- **THEN** 员工收到通知 "收到工资：¥{金额}"

### Requirement: Local Notification

移动端 SHALL 显示系统本地通知。

#### Scenario: App 在前台

- **GIVEN** App 在前台运行
- **WHEN** 收到 SSE 消息
- **THEN** 显示 Toast 提示
- **AND** 显示系统本地通知

#### Scenario: 通知权限请求

- **GIVEN** App 首次启动
- **WHEN** 用户登录成功
- **THEN** 请求系统通知权限
