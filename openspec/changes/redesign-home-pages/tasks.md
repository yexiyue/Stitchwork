# Tasks: redesign-home-pages

## Backend

- [ ] 1. 添加首页概览 API
  - `GET /api/home/overview`
  - 老板：待审核数、进行中订单、今日/本月产量、员工数
  - 员工：本月产量和金额

- [ ] 2. 添加最近动态 API
  - `GET /api/home/activities`
  - 老板：最近计件提交/审核事件
  - 员工：最近个人计件记录

- [ ] 3. cargo check 验证后端编译

## Frontend - Common

- [ ] 4. 添加首页 API 客户端
  - homeApi.overview()
  - homeApi.activities()

- [ ] 5. 创建迷你图表组件
  - 复用 ECharts
  - 支持简化的折线图展示

## Frontend - Boss Home

- [ ] 6. 拆分老板端首页
  - 创建 `src/routes/_auth/_boss/index.tsx`
  - 保留工坊信息卡片

- [ ] 7. 添加数据概览区域
  - 待审核数（实时徽章）
  - 进行中订单数
  - 今日产量

- [ ] 8. 添加本周趋势迷你图
  - 调用 daily-stats API
  - 显示近7天产量

- [ ] 9. 添加快捷操作入口
  - 新建订单
  - 客户管理
  - 快捷记件

- [ ] 10. 添加最近动态列表
  - 显示最近5条动态
  - 支持点击查看详情

## Frontend - Staff Home

- [ ] 11. 拆分员工端首页
  - 创建 `src/routes/_auth/_staff/index.tsx`
  - 保留工坊信息卡片

- [ ] 12. 添加本月统计卡片
  - 完成数量、预估金额
  - 点击跳转统计详情

- [ ] 13. 添加本周趋势迷你图
  - 调用 daily-stats API
  - 显示近7天个人产量

- [ ] 14. 添加快捷操作入口
  - 录入计件按钮

- [ ] 15. 添加最近记录列表
  - 显示最近5条计件记录

## Cleanup

- [ ] 16. 移除旧的共用首页
  - 删除或重定向 `_auth/index.tsx`

- [ ] 17. 更新路由配置
  - 确保老板/员工分别进入各自首页

## Validation

- [ ] 18. pnpm build 验证编译
- [ ] 19. 手动测试首页功能
