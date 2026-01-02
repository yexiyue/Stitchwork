# order-management Specification

## Purpose
TBD - created by archiving change add-list-filters-and-order-status. Update Purpose after archive.
## Requirements
### Requirement: Order Status Flow

系统 SHALL 定义明确的订单状态流转规则。

#### Scenario: 订单状态流转 pending → processing

- **WHEN** 订单状态为 `pending`
- **AND** 请求更新状态为 `processing`
- **THEN** 状态更新成功

#### Scenario: 订单状态流转 processing → completed

- **WHEN** 订单状态为 `processing`
- **AND** 请求更新状态为 `completed`
- **THEN** 状态更新成功

#### Scenario: 订单状态流转 completed → delivered

- **WHEN** 订单状态为 `completed`
- **AND** 请求更新状态为 `delivered`
- **THEN** 状态更新成功，并设置 `delivered_at`

#### Scenario: 非法状态流转被拒绝

- **WHEN** 请求的状态流转不符合规则（如 pending → delivered）
- **THEN** 返回 400 Bad Request 错误

### Requirement: Order List Filter by Customer

系统 SHALL 支持按客户筛选订单列表。

#### Scenario: 按客户筛选订单

- **WHEN** 请求订单列表时提供 `customer_id` 参数
- **THEN** 只返回该客户的订单

### Requirement: Order List Filter by Status

系统 SHALL 支持按状态筛选订单列表。

#### Scenario: 按状态筛选订单

- **WHEN** 请求订单列表时提供 `status` 参数
- **THEN** 只返回该状态的订单

### Requirement: Process List Filter by Order

系统 SHALL 支持按订单筛选工序列表。

#### Scenario: 按订单筛选工序

- **WHEN** 请求工序列表时提供 `order_id` 参数
- **THEN** 只返回该订单的工序

### Requirement: Auto Status Update on Piece Record

系统 SHALL 在创建计件记录时自动更新订单状态。

#### Scenario: 首次计件自动更新订单状态

- **WHEN** 创建计件记录
- **AND** 关联订单状态为 `pending`
- **THEN** 自动将订单状态更新为 `processing`

#### Scenario: 已在加工中的订单不变

- **WHEN** 创建计件记录
- **AND** 关联订单状态不是 `pending`
- **THEN** 订单状态保持不变

