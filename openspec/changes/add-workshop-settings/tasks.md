# Tasks: add-workshop-settings

## Backend

- [ ] 1. Workshop entity 添加 `piece_unit`、`business_label` 字段（默认值 "打"、"工坊"）
- [ ] 2. 更新 Workshop DTO 支持新字段
- [ ] 3. 更新 create/update workshop service 处理新字段
- [ ] 4. cargo check 验证后端编译

## Frontend Types & API

- [ ] 5. 更新 Workshop 类型定义添加 `pieceUnit`、`businessLabel`
- [ ] 6. 创建 `useWorkshopSettings` Hook

## Frontend Settings Page

- [ ] 7. 工坊设置页添加「计件单位」输入框
- [ ] 8. 工坊设置页添加「场所名称」输入框

## Frontend UI 文案替换

### 员工端
- [ ] 9. `_staff/-home.tsx` 替换 "件" 为动态单位
- [ ] 10. `_staff/my-records/index.tsx` 替换 "件"
- [ ] 11. `_staff/my-records/$id.tsx` 替换 "件"
- [ ] 12. `_staff/my-records/stats.tsx` 替换 "件"

### 老板端
- [ ] 13. `_boss/-home.tsx` 替换 "件"
- [ ] 14. `_boss/records/index.tsx` 替换 "件"
- [ ] 15. `_boss/records/$id.tsx` 替换 "件"
- [ ] 16. `_boss/records/stats.tsx` 替换 "件"
- [ ] 17. `_boss/records/new.tsx` 替换相关文案
- [ ] 18. `_boss/shares/$id.tsx` 替换 "件"
- [ ] 19. `_boss/shares/new.tsx` 替换 "件"
- [ ] 20. `_boss/orders/$id/index.tsx` 替换 "件"
- [ ] 21. `_boss/orders/stats.tsx` 替换 "件"
- [ ] 22. `_boss/payroll/new.tsx` 替换相关文案
- [ ] 23. `_boss/workshop.tsx` 替换 "工坊" 为动态名称

### 公共页面
- [ ] 24. `share/$token.tsx` 替换 "件" 和 "工坊地址"

## Validation

- [ ] 25. pnpm tsc --noEmit 验证前端编译
