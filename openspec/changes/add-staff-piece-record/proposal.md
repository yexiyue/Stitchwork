# Proposal: add-staff-piece-record

## Summary

实现员工端计件功能：计件列表页（带订单图片）、计件详情页、员工自行录入计件。

## Background

当前系统：
- Boss 端计件管理功能完整（列表、新增、审核）
- 员工端 `record.tsx` 仅为占位符（显示"暂无可记件的工序"）
- 后端 API 已支持员工查看自己的计件记录（`user_id` 过滤）
- 后端 API 已支持员工创建计件（状态为 `pending`，需 Boss 审核）

缺失功能：
- 后端 list API 未返回订单图片字段
- 员工端无计件列表 UI
- 员工端无计件详情页
- 员工端无录入计件 UI

## Scope

### In Scope

1. **后端扩展**：list API 返回 `orderImage` 字段（订单第一张图片）
2. **员工计件列表页**：显示自己的所有计件记录，卡片带订单图片
3. **员工计件详情页**：查看订单信息、工序信息、计件详情
4. **员工录入计件页**：选择订单/工序，输入数量，提交待审核

### Out of Scope

- 员工修改/删除计件记录（仅 Boss 有权限）
- 员工审核计件（仅 Boss 有权限）
- 统计摘要功能（后续迭代）

## Approach

1. **后端**：扩展 piece_record list 返回结构，加入关联订单的第一张图片
2. **前端类型**：`PieceRecord` 添加 `orderImage` 字段
3. **员工列表页**：复用 `VirtualList` 组件，卡片左侧显示图片
4. **员工详情页**：路由 `/_staff/records/$id`，展示完整信息
5. **员工录入页**：路由 `/_staff/records/new`，级联选择订单→工序

## Dependencies

- 现有 piece_record API（已支持员工权限）
- 现有 order/process API（获取下拉选项）
- VirtualList、ImageViewer 等 UI 组件
