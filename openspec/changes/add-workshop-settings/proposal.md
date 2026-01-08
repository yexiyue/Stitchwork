# Proposal: add-workshop-settings

## Summary

允许老板在工坊设置中自定义计件单位和经营场所名称，替代当前硬编码的「打」和「工坊」文案。

## Problem

当前系统存在以下限制：
1. 计件单位固定为「打」，无法满足不同行业需求（如「件」「双」「条」等）
2. 经营场所统一称为「工坊」，不适合所有业态（如「工厂」「店铺」等）
3. UI 中多处硬编码这些术语，无法根据实际业务场景灵活调整

## Solution

1. **扩展 Workshop 实体**：添加 `piece_unit`（计件单位）和 `business_label`（场所名称）字段
2. **更新工坊设置页**：允许老板编辑这两个设置项
3. **前端动态替换**：创建设置 Hook，将 UI 中的硬编码文案替换为动态值

## Scope

### Backend
- Workshop entity 新增 `piece_unit`、`business_label` 字段（带默认值）
- 更新 Workshop API 支持新字段的读写

### Frontend
- 工坊设置页新增「计件单位」和「场所名称」表单项
- 创建 `useWorkshopSettings` Hook 提供设置访问
- 更新约 30 处 UI 文案引用

## Out of Scope

- 多语言国际化
- 单位换算（如 1打=12件）
- 历史数据迁移（新字段使用默认值）

## Dependencies

无外部依赖。

## Risks

- **文案遗漏**：可能遗漏部分硬编码位置，需全面搜索验证
- **缓存一致性**：用户修改设置后需确保前端立即生效
