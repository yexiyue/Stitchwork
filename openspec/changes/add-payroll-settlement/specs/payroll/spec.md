## ADDED Requirements

### Requirement: Payroll Settlement

系统 SHALL 支持老板创建工资单结算多个计件记录。

#### Scenario: 创建工资单成功

- **GIVEN** 老板已登录
- **AND** 存在多个已批准（Approved）状态的计件记录
- **WHEN** 老板选择计件记录并创建工资单
- **THEN** 系统创建工资单并关联所选计件
- **AND** 所选计件状态变更为已结算（Settled）

#### Scenario: 创建工资单校验失败

- **GIVEN** 老板尝试创建工资单
- **WHEN** 所选计件中存在非 Approved 状态的记录
- **THEN** 系统拒绝创建并返回错误信息

### Requirement: Payroll Payment Proof

系统 SHALL 支持上传支付凭证图片。

#### Scenario: 上传支付凭证

- **GIVEN** 老板正在创建或编辑工资单
- **WHEN** 老板上传支付凭证图片（如微信转账截图）
- **THEN** 系统保存图片并关联到工资单

#### Scenario: 查看支付凭证

- **GIVEN** 工资单已有支付凭证
- **WHEN** 用户查看工资单详情
- **THEN** 系统显示支付凭证图片

### Requirement: Payroll Detail View

系统 SHALL 支持查看工资单的结算明细。

#### Scenario: 查看结算明细

- **GIVEN** 工资单已创建并关联计件记录
- **WHEN** 用户查看工资单详情
- **THEN** 系统显示关联的计件记录列表（包含订单、工序、数量、金额）

### Requirement: Payroll List

系统 SHALL 提供工资单列表管理页面。

#### Scenario: 按员工筛选工资单

- **GIVEN** 老板在工资单列表页面
- **WHEN** 老板按员工筛选
- **THEN** 系统显示该员工的所有工资单

#### Scenario: 工资单列表排序

- **GIVEN** 老板在工资单列表页面
- **WHEN** 页面加载
- **THEN** 系统按发放时间倒序显示工资单
