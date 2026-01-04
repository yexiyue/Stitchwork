# boss-records-detail-ui Specification

## Purpose

提供老板端计件详情页，支持查看完整信息和管理操作。

## ADDED Requirements

### Requirement: Boss Piece Record Detail Page

老板端 SHALL 提供计件详情页 `/records/$id`

#### Scenario: 显示订单信息
- Given 老板访问计件详情页
- When 页面加载
- Then 显示订单图片轮播（如有多张）
- And 显示订单产品名、描述、数量、状态

#### Scenario: 显示工序信息
- Given 老板访问计件详情页
- When 页面加载
- Then 显示工序名称、描述、计件单价

#### Scenario: 显示计件信息
- Given 老板访问计件详情页
- When 页面加载
- Then 显示计件数量、金额、审核状态
- And 显示员工姓名、录入方式、录入时间

### Requirement: Boss Piece Record Actions

老板端详情页 SHALL 支持审核、编辑和删除操作。

#### Scenario: 审核待审核记录
- Given 计件记录状态为 pending
- When 老板点击"通过"或"拒绝"按钮
- Then 更新记录状态
- And 返回列表页并刷新

#### Scenario: 编辑计件数量
- Given 老板在详情页
- When 点击编辑按钮
- Then 可修改计件数量
- And 金额自动重新计算

#### Scenario: 删除计件记录
- Given 老板在详情页
- When 点击删除按钮
- Then 弹出确认对话框
- And 确认后删除记录并返回列表

## Cross-references

- boss-records-list-ui: 点击卡片进入详情
