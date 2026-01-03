# frontend-api Specification

## Purpose
TBD - created by archiving change add-frontend-api-module. Update Purpose after archive.
## Requirements
### Requirement: API Client

系统 SHALL 提供统一的 HTTP 请求客户端，封装 fetch API。

#### Scenario: 请求发送
- **WHEN** 调用 API 函数发起请求
- **THEN** 自动添加 Authorization header（如有 token）
- **THEN** 自动处理 JSON 序列化/反序列化

#### Scenario: 错误处理
- **WHEN** 服务端返回非 0 code
- **THEN** 抛出包含错误信息的异常

### Requirement: Auth API

系统 SHALL 提供认证相关的 API 函数。

#### Scenario: 登录
- **WHEN** 调用 `authApi.login({ username, password })`
- **THEN** 发送 POST /api/login 请求并返回 token

#### Scenario: 注册
- **WHEN** 调用 `authApi.register({ username, password })`
- **THEN** 发送 POST /api/register 请求

### Requirement: Customer API

系统 SHALL 提供客户管理的 CRUD API 函数。

#### Scenario: 客户列表
- **WHEN** 调用 `customerApi.list(params)`
- **THEN** 发送 GET /api/customers 请求并返回分页数据

#### Scenario: 创建客户
- **WHEN** 调用 `customerApi.create(dto)`
- **THEN** 发送 POST /api/customers 请求

### Requirement: Order API

系统 SHALL 提供订单管理的 CRUD API 函数。

#### Scenario: 订单列表
- **WHEN** 调用 `orderApi.list(params)`
- **THEN** 发送 GET /api/orders 请求并返回分页数据

#### Scenario: 更新订单状态
- **WHEN** 调用 `orderApi.updateStatus(id, status)`
- **THEN** 发送 PATCH /api/orders/{id}/status 请求

### Requirement: Process API

系统 SHALL 提供工序管理的 CRUD API 函数。

#### Scenario: 工序列表
- **WHEN** 调用 `processApi.list(params)`
- **THEN** 发送 GET /api/processes 请求并返回分页数据

### Requirement: PieceRecord API

系统 SHALL 提供计件记录的 CRUD API 函数。

#### Scenario: 计件记录列表
- **WHEN** 调用 `pieceRecordApi.list(params)`
- **THEN** 发送 GET /api/piece-records 请求并返回分页数据

### Requirement: Payroll API

系统 SHALL 提供工资管理的 CRUD API 函数。

#### Scenario: 工资列表
- **WHEN** 调用 `payrollApi.list(params)`
- **THEN** 发送 GET /api/payrolls 请求并返回分页数据

### Requirement: Stats API

系统 SHALL 提供统计相关的 API 函数。

#### Scenario: 订单统计
- **WHEN** 调用 `statsApi.orderStats(orderId)`
- **THEN** 发送 GET /api/orders/{id}/stats 请求

#### Scenario: 客户汇总
- **WHEN** 调用 `statsApi.customerSummary()`
- **THEN** 发送 GET /api/stats/customers 请求

#### Scenario: 工人产量
- **WHEN** 调用 `statsApi.workerProduction(params)`
- **THEN** 发送 GET /api/stats/workers 请求

### Requirement: Share API

系统 SHALL 提供分享管理的 API 函数。

#### Scenario: 分享列表
- **WHEN** 调用 `shareApi.list()`
- **THEN** 发送 GET /api/shares 请求

#### Scenario: 公开分享页
- **WHEN** 调用 `shareApi.getPublic(token)`
- **THEN** 发送 GET /api/public/share/{token} 请求

