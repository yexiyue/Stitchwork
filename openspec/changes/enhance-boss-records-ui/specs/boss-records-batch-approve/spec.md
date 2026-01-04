# boss-records-batch-approve Specification

## Purpose

提供批量审核功能，提高老板处理多条待审核记录的效率。

## ADDED Requirements

### Requirement: Batch Approve Mode

老板端计件列表 SHALL 支持批量审核模式。

#### Scenario: 进入批量模式
- Given 老板在计件列表页
- When 点击"批量审核"按钮
- Then 进入批量选择模式
- And 每个待审核卡片显示复选框
- And 底部显示批量操作栏

#### Scenario: 选择记录
- Given 老板在批量模式
- When 点击待审核记录的复选框
- Then 该记录被选中/取消选中
- And 底部显示已选数量

#### Scenario: 全选待审核记录
- Given 老板在批量模式
- When 点击"全选"按钮
- Then 所有可见的待审核记录被选中

#### Scenario: 批量通过
- Given 老板选中了多条待审核记录
- When 点击"批量通过"按钮
- Then 所有选中记录状态更新为 approved
- And 显示成功提示

#### Scenario: 批量拒绝
- Given 老板选中了多条待审核记录
- When 点击"批量拒绝"按钮
- Then 所有选中记录状态更新为 rejected
- And 显示成功提示

#### Scenario: 退出批量模式
- Given 老板在批量模式
- When 点击"取消"或完成批量操作后
- Then 退出批量模式
- And 恢复正常列表显示

### Requirement: Batch Approve API

后端 SHALL 提供批量审核 API。

#### Scenario: 批量通过
- Given 请求 `POST /piece-records/batch-approve`
- When 提供 ids 数组
- Then 将所有指定的 pending 记录状态更新为 approved

#### Scenario: 批量拒绝
- Given 请求 `POST /piece-records/batch-reject`
- When 提供 ids 数组
- Then 将所有指定的 pending 记录状态更新为 rejected

## Cross-references

- frontend-api: 需要添加批量审核 API
