# Tasks: add-admin-dashboard

## Backend

- [ ] 1. 添加 AdminStats DTO 定义统计数据结构
- [ ] 2. 实现 admin_stats service 函数，聚合各项统计数据
- [ ] 3. 添加 GET /api/admin/stats 路由
- [ ] 4. 修改 list_users 支持分页参数，返回 { list, total }
- [ ] 5. 修改 list_register_codes 支持分页参数，返回 { list, total }
- [ ] 6. cargo check 验证后端编译

## Frontend API

- [ ] 7. 更新 adminApi.listUsers 支持分页参数
- [ ] 8. 更新 adminApi.listRegisterCodes 支持分页参数
- [ ] 9. 添加 adminApi.getStats 接口
- [ ] 10. 添加 AdminStats 类型定义

## Frontend Pages

- [ ] 11. 重构超管首页，添加统计卡片和图表
- [ ] 12. 用户列表页改用 VirtualList + useInfiniteList
- [ ] 13. 注册码列表页改用 VirtualList + useInfiniteList
- [ ] 14. pnpm tsc --noEmit 验证前端编译
