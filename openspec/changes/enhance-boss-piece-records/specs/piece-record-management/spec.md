# Piece Record Management

## ADDED Requirements

### Requirement: 老板端计件列表显示关联信息

计件列表卡片 SHALL 显示完整的关联信息，包括员工姓名、工序名称、订单名称。

#### Scenario: 查看计件记录详情
- Given 老板在计件管理页面
- When 查看计件记录列表
- Then 每条记录显示：员工姓名、工序名称、订单名称、数量、金额、状态、记录时间

### Requirement: 老板端按员工筛选计件记录

老板 SHALL 可以按员工筛选计件记录，快速查看特定员工的计件情况。

#### Scenario: 按员工筛选
- Given 老板在计件管理页面
- When 点击员工筛选下拉菜单并选择某员工
- Then 列表只显示该员工的计件记录

### Requirement: 老板端按订单筛选计件记录

老板 SHALL 可以按订单筛选计件记录，查看特定订单的计件进度。

#### Scenario: 按订单筛选
- Given 老板在计件管理页面
- When 点击订单筛选下拉菜单并选择某订单
- Then 列表只显示该订单相关工序的计件记录

### Requirement: 老板帮员工创建计件记录

老板 SHALL 可以帮员工创建计件记录，记录的 recordedBy 字段为 "boss"，状态自动为 approved。

#### Scenario: 老板创建计件记录
- Given 老板在计件管理页面
- When 点击"新增"按钮
- Then 弹出表单，包含：员工选择、订单选择、工序选择（级联）、数量输入
- When 填写完成并提交
- Then 创建计件记录，recordedBy = "boss"，status = "approved"

### Requirement: 工序选择依赖订单

创建计件时，工序选择列表 SHALL 根据所选订单动态加载。

#### Scenario: 级联选择工序
- Given 老板在新增计件弹窗
- When 选择一个订单
- Then 工序选择列表显示该订单下的所有工序
- When 未选择订单
- Then 工序选择列表为空或禁用
