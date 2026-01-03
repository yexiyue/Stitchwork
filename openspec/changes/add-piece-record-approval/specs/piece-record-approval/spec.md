# piece-record-approval Specification

## ADDED Requirements

### Requirement: Piece Record Status

系统 SHALL 支持记件记录的审批状态管理。

#### Scenario: 员工创建记件记录
- **WHEN** Staff 用户创建记件记录
- **THEN** 记录状态为 `pending`，`recorded_by` 为 `self`

#### Scenario: 老板创建记件记录
- **WHEN** Boss 用户为员工创建记件记录
- **THEN** 记录状态为 `approved`，`recorded_by` 为 `boss`

#### Scenario: 查询待审核记录
- **WHEN** Boss 用户请求待审核记件记录列表
- **THEN** 系统返回 `status = pending` 的记录

### Requirement: Piece Record Approval

系统 SHALL 允许 Boss 审批员工提交的记件记录。

#### Scenario: 审批通过
- **WHEN** Boss 用户审批通过记件记录
- **AND** 该记录属于该 Boss（`piece_record.boss_id = boss_id`）
- **AND** 该记录状态为 `pending`
- **THEN** 记录状态更新为 `approved`

#### Scenario: 审批驳回
- **WHEN** Boss 用户驳回记件记录
- **AND** 该记录属于该 Boss
- **AND** 该记录状态为 `pending`
- **THEN** 记录状态更新为 `rejected`

#### Scenario: 非待审核状态无法审批
- **WHEN** Boss 用户尝试审批非 `pending` 状态的记录
- **THEN** 返回 400 Bad Request 错误

### Requirement: Approved Records Only for Statistics

系统 SHALL 只将 `approved` 状态的记件记录计入统计和工资计算。

#### Scenario: 工资统计只计算已通过记录
- **WHEN** 系统计算员工工资
- **THEN** 只统计 `status = approved` 的记件记录金额
