# Tasks: add-admin-dashboard

## Backend

- [x] 1. 添加 AdminStats DTO 定义统计数据结构
- [x] 2. 实现 admin_stats service 函数，聚合各项统计数据
- [x] 3. 添加 GET /api/admin/stats 路由
- [x] 4. 修改 list_users 支持分页参数，返回 { list, total }
- [x] 5. 修改 list_register_codes 支持分页参数，返回 { list, total }
- [x] 6. cargo check 验证后端编译

## Frontend API

- [x] 7. 更新 adminApi.listUsers 支持分页参数
- [x] 8. 更新 adminApi.listRegisterCodes 支持分页参数
- [x] 9. 添加 adminApi.getStats 接口
- [x] 10. 添加 AdminStats 类型定义

## Frontend Pages

- [x] 11. 重构超管首页，添加统计卡片和图表
- [x] 12. 用户列表页改用 VirtualList + useInfiniteList
- [x] 13. 注册码列表页改用 VirtualList + useInfiniteList
- [x] 14. pnpm tsc --noEmit 验证前端编译
