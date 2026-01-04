# Tasks: enhance-boss-records-ui

## Backend Tasks

- [x] 1. 添加批量审核 API
  - `POST /piece-records/batch-approve` - 批量通过
  - `POST /piece-records/batch-reject` - 批量拒绝
  - 接收 `{ ids: string[] }` 请求体
- [x] 2. cargo check 验证编译通过

## Frontend Tasks

### 列表增强

- [x] 3. 列表卡片显示订单图片
  - 左侧 56×56 订单图片
  - 无图片时显示占位图
- [x] 4. 卡片点击跳转到详情页

### 详情页

- [x] 5. 创建计件详情页 `/records/$id`
  - 订单图片轮播、信息展示
  - 添加员工姓名显示
- [x] 6. 详情页添加审核操作
  - pending 状态显示"通过"/"拒绝"按钮
- [x] 7. 详情页添加编辑功能
  - 修改数量，自动重算金额
- [x] 8. 详情页添加删除功能
  - 确认对话框后删除

### 批量审核

- [x] 9. 添加批量审核 API 调用
  - `pieceRecordApi.batchApprove`
  - `pieceRecordApi.batchReject`
- [x] 10. 列表页添加批量模式入口
- [x] 11. 实现批量选择交互
  - 复选框、全选、已选计数
- [x] 12. 实现批量操作栏
  - 批量通过/拒绝按钮

### 统计功能

- [x] 13. Stats API 已存在（statsApi.workerProduction）
- [x] 14. 列表页添加统计入口按钮
- [x] 15. 创建统计页面 `/records/stats`
  - 员工产量卡片列表
  - 日期范围选择器

## Validation

- [x] 16. cargo check + tsc 验证编译
  - 注：orders 页面有预存在的类型错误（非本次修改引入）
- [ ] 17. 手动测试全流程
