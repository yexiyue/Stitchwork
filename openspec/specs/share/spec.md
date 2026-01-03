# share Specification

## Purpose
TBD - created by archiving change add-entity-description-and-images. Update Purpose after archive.
## Requirements
### Requirement: Create Share

系统 SHALL 支持 Boss 创建招工分享。

#### Scenario: 创建招工分享

- **WHEN** Boss 请求 `POST /shares`
- **AND** 提供 title, order_ids, process_ids
- **THEN** 生成唯一 share_token 并返回分享信息

### Requirement: Public Share Access

系统 SHALL 支持公开访问招工分享。

#### Scenario: 访问招工页面

- **WHEN** 请求 `GET /public/share/{token}`
- **AND** token 有效且分享处于激活状态
- **THEN** 返回作坊信息、选中的订单和工序列表

#### Scenario: 无效 token 访问

- **WHEN** 请求 `GET /public/share/{token}`
- **AND** token 无效或分享已停用
- **THEN** 返回 404 Not Found

### Requirement: Manage Share

系统 SHALL 支持 Boss 管理招工分享。

#### Scenario: 停用分享

- **WHEN** Boss 请求 `PATCH /shares/{id}`
- **AND** 设置 is_active 为 false
- **THEN** 分享链接不再可访问

