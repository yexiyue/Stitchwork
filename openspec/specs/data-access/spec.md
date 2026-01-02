# data-access Specification

## Purpose
TBD - created by archiving change add-user-data-isolation. Update Purpose after archive.
## Requirements
### Requirement: Boss Data Isolation

系统 SHALL 确保 Boss 用户只能访问自己创建的数据。

#### Scenario: Boss 查询客户列表
- **WHEN** Boss 用户请求客户列表
- **THEN** 系统只返回该 Boss 创建的客户（`customer.user_id = boss_id`）

#### Scenario: Boss 查询订单列表
- **WHEN** Boss 用户请求订单列表
- **THEN** 系统只返回该 Boss 的订单（`order.boss_id = boss_id`）

#### Scenario: Boss 查询工序列表
- **WHEN** Boss 用户请求工序列表
- **THEN** 系统只返回该 Boss 的工序（`process.boss_id = boss_id`）

#### Scenario: Boss 查询计件记录列表
- **WHEN** Boss 用户请求计件记录列表
- **THEN** 系统只返回该 Boss 的计件记录（`piece_record.boss_id = boss_id`）

### Requirement: Staff Data Access

系统 SHALL 允许 Staff 用户访问自己参与的数据。

#### Scenario: Staff 查询订单列表
- **WHEN** Staff 用户请求订单列表
- **THEN** 系统只返回该 Staff 参与过的订单（通过 piece_record 关联）

#### Scenario: Staff 查询工序列表
- **WHEN** Staff 用户请求工序列表
- **THEN** 系统只返回该 Staff 参与过的工序（`piece_record.user_id = staff_id`）

#### Scenario: Staff 查询计件记录列表
- **WHEN** Staff 用户请求计件记录列表
- **THEN** 系统只返回该 Staff 的计件记录（`piece_record.user_id = staff_id`）

### Requirement: Boss ID Redundancy

系统 SHALL 在 `order`, `process`, `piece_record` 表中维护 `boss_id` 冗余字段。

#### Scenario: 创建订单时自动填充 boss_id
- **WHEN** 创建新订单
- **THEN** 系统自动从关联的 customer 获取 `user_id` 并填充到 `order.boss_id`

#### Scenario: 创建工序时自动填充 boss_id
- **WHEN** 创建新工序
- **THEN** 系统自动从关联的 order 获取 `boss_id` 并填充到 `process.boss_id`

#### Scenario: 创建计件记录时自动填充 boss_id
- **WHEN** 创建新计件记录
- **THEN** 系统自动从关联的 process 获取 `boss_id` 并填充到 `piece_record.boss_id`

### Requirement: Update/Delete Ownership Validation

系统 SHALL 在更新或删除数据时校验数据归属权。

#### Scenario: Boss 更新自己的客户
- **WHEN** Boss 用户更新客户信息
- **AND** 该客户属于该 Boss（`customer.user_id = boss_id`）
- **THEN** 更新成功

#### Scenario: Boss 更新他人的客户
- **WHEN** Boss 用户更新客户信息
- **AND** 该客户不属于该 Boss
- **THEN** 返回 403 Forbidden 错误

#### Scenario: Boss 删除自己的订单
- **WHEN** Boss 用户删除订单
- **AND** 该订单属于该 Boss（`order.boss_id = boss_id`）
- **THEN** 删除成功

#### Scenario: Boss 删除他人的订单
- **WHEN** Boss 用户删除订单
- **AND** 该订单不属于该 Boss
- **THEN** 返回 403 Forbidden 错误

