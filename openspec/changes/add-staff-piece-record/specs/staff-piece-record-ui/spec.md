# staff-piece-record-ui Specification

## Purpose

提供员工端计件功能 UI：列表页、详情页、录入页。

## ADDED Requirements

### Requirement: Staff Piece Record List

员工端 SHALL 提供计件列表页 `/_staff/records/`

#### Scenario: 显示计件列表
- Given 员工已登录
- When 访问计件列表页
- Then 显示该员工的所有计件记录
- And 按录入时间倒序排列

#### Scenario: 卡片显示订单图片
- Given 计件记录有关联的订单图片
- When 渲染列表卡片
- Then 卡片左侧显示订单图片缩略图（56×56px）
- And 卡片右侧显示：订单名-工序名、数量金额、状态、时间

#### Scenario: 按状态筛选
- Given 员工在列表页
- When 选择状态筛选器
- Then 列表仅显示对应状态的记录

#### Scenario: 点击卡片进入详情
- Given 员工在列表页
- When 点击某条计件卡片
- Then 跳转到该计件的详情页

### Requirement: Staff Piece Record Detail

员工端 SHALL 提供计件详情页 `/_staff/records/$id`

#### Scenario: 显示订单信息
- Given 员工访问计件详情页
- When 页面加载
- Then 显示订单图片轮播（如有多张）
- And 显示订单产品名、描述、数量、状态

#### Scenario: 显示工序信息
- Given 员工访问计件详情页
- When 页面加载
- Then 显示工序名称、描述、计件单价

#### Scenario: 显示计件信息
- Given 员工访问计件详情页
- When 页面加载
- Then 显示计件数量、金额、审核状态
- And 显示录入方式（自己/老板）、录入时间

### Requirement: Staff Create Piece Record

员工端 SHALL 提供计件录入页 `/_staff/records/new`

#### Scenario: 选择订单和工序
- Given 员工在录入页
- When 选择订单
- Then 工序下拉框加载该订单的工序列表
- And 选择工序后显示计件单价

#### Scenario: 输入数量计算金额
- Given 员工已选择工序
- When 输入计件数量
- Then 自动计算并显示金额（单价 × 数量）

#### Scenario: 提交待审核
- Given 员工填写完整信息
- When 点击提交
- Then 创建计件记录（状态为 pending）
- And 显示提示"已提交，待老板审核"
- And 返回列表页

## Cross-references

- staff-piece-record-api: orderImage 字段
- auth-ui: Route Guard（员工角色）
