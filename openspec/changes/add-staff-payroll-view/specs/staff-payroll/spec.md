# Staff Payroll View

## ADDED Requirements

### Requirement: Staff Payroll List

员工 SHALL 能够查看自己的工资发放历史列表。

#### Scenario: 查看工资单列表

- **GIVEN** 员工已登录
- **WHEN** 员工访问"我的工资"页面
- **THEN** 系统显示该员工的所有工资单
- **AND** 按发放时间倒序排列
- **AND** 每条记录显示金额、发放日期、备注摘要

#### Scenario: 空状态

- **GIVEN** 员工已登录
- **WHEN** 员工没有任何工资发放记录
- **THEN** 页面显示空状态提示

### Requirement: Staff Payroll Detail

员工 SHALL 能够查看工资单详情，包括关联的计件记录。

#### Scenario: 查看工资单详情

- **GIVEN** 员工已登录
- **WHEN** 员工点击某条工资单
- **THEN** 系统显示该工资单的完整信息
- **AND** 包括：金额、发放日期、备注、支付凭证图片
- **AND** 显示关联的计件记录列表

#### Scenario: 权限控制

- **GIVEN** 员工已登录
- **WHEN** 员工尝试访问其他员工的工资单详情
- **THEN** 系统返回 403 或 404 错误
