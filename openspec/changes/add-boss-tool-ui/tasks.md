# Tasks: 添加老板端 Tool UI 和前端工具

## 1. 类型定义

- [x] 1.1 在 `src/components/tools/types.ts` 添加老板端响应类型
  - OrderListResponse
  - BossPieceRecordListResponse
  - WorkerProductionList
  - BossOverview
  - OrderProgressList
  - UnpaidSummaryResponse

## 2. 订单列表 Tool UI

- [x] 2.1 创建 `src/components/tools/orders/` 目录
- [x] 2.2 实现 `columns.ts` 定义订单表格列
- [x] 2.3 实现 `loading.tsx` 加载状态组件
- [x] 2.4 实现 `index.tsx` 主 Tool UI 组件

## 3. 计件记录列表 Tool UI

- [x] 3.1 创建 `src/components/tools/piece-records/` 目录
- [x] 3.2 实现 `columns.ts` 定义计件记录表格列
- [x] 3.3 实现 `loading.tsx` 加载状态组件
- [x] 3.4 实现 `index.tsx` 主 Tool UI 组件

## 4. 员工产量统计 Tool UI

- [x] 4.1 创建 `src/components/tools/worker-stats/` 目录
- [x] 4.2 实现 `columns.ts` 定义表格列
- [x] 4.3 实现 `index.tsx` 主 Tool UI 组件（统计卡片 + 员工排行榜）

## 5. 首页概览 Tool UI

- [x] 5.1 创建 `src/components/tools/overview/` 目录
- [x] 5.2 实现 `constants.ts` 定义概览统计项
- [x] 5.3 实现 `index.tsx` 主 Tool UI 组件

## 6. 订单进度 Tool UI

- [x] 6.1 创建 `src/components/tools/order-progress/` 目录
- [x] 6.2 实现进度条组件
- [x] 6.3 实现 `index.tsx` 主 Tool UI 组件

## 7. 待发工资汇总 Tool UI

- [x] 7.1 创建 `src/components/tools/unpaid-summary/` 目录
- [x] 7.2 实现 `columns.ts` 定义表格列
- [x] 7.3 实现 `index.tsx` 主 Tool UI 组件（汇总统计 + 员工列表）

## 8. 集成与导出

- [x] 8.1 更新 `src/components/tools/index.tsx` 导出所有新组件
- [x] 8.2 更新 `src/routes/chat.tsx` 注册老板端 Tool UI 组件

## 9. 验证

- [x] 9.1 TypeScript 类型检查通过
- [ ] 9.2 测试各 Tool UI 组件在 AI 助手中正确渲染
