# data-access Specification

## MODIFIED Requirements

### Requirement: Staff Data Access

系统 SHALL 允许 Staff 用户访问工坊的订单和工序数据，但隐藏敏感信息。

#### Scenario: Staff 查询订单列表
- **WHEN** Staff 用户请求订单列表
- **THEN** 系统返回该 Staff 所属工坊的所有订单
- **AND** 隐藏客户详细信息（只显示订单名称、产品名、状态）

#### Scenario: Staff 查询工序列表
- **WHEN** Staff 用户请求工序列表
- **THEN** 系统返回该 Staff 所属工坊的所有工序
- **AND** 显示：工序名称、单价、总数量、已完成数量

#### Scenario: Staff 查询计件记录列表
- **WHEN** Staff 用户请求计件记录列表
- **THEN** 系统只返回该 Staff 自己的计件记录（`piece_record.user_id = staff_id`）
