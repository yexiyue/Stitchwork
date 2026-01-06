## ADDED Requirements

### Requirement: Piece Record Settled Status

计件记录 SHALL 支持已结算（Settled）状态。

#### Scenario: 计件被工资单结算

- **GIVEN** 计件记录状态为 Approved
- **WHEN** 该计件被包含在新创建的工资单中
- **THEN** 计件状态自动变更为 Settled

#### Scenario: 已结算计件不可重复结算

- **GIVEN** 计件记录状态为 Settled
- **WHEN** 尝试将该计件纳入新的工资单
- **THEN** 系统拒绝并提示该计件已结算

### Requirement: Filter Unsettled Records

系统 SHALL 支持筛选未结算的已批准计件。

#### Scenario: 查询待结算计件

- **GIVEN** 老板在创建工资单页面
- **WHEN** 选择员工后查询待结算计件
- **THEN** 系统返回该员工所有 Approved 状态的计件记录

#### Scenario: 计件列表显示结算状态

- **GIVEN** 老板在计件记录列表页面
- **WHEN** 查看计件记录
- **THEN** 已结算的计件显示"已结算"标签
