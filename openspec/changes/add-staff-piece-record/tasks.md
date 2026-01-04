# Tasks: add-staff-piece-record

## Backend Tasks

- [x] 1. 扩展 piece_record list 返回结构，添加 `order_image` 字段
  - 关联查询 order 表获取 images 字段
  - 取 images 数组第一个元素作为封面图
- [x] 2. 新增员工可访问的订单/工序 API
  - 更新 order/process getOne 支持 Staff 访问（通过 workshop_id 验证权限）
  - 更新 piece_record getOne 支持 Staff 访问自己的记录
- [x] 3. cargo check 验证编译通过

## Frontend Tasks

- [x] 4. 更新 `PieceRecord` 类型，添加 `orderImage` 和 `piecePrice` 字段
- [x] 5. 实现员工计件列表页 `/my-records/` (原设计路径 `/_staff/records/`，因路由冲突调整)
  - 虚拟列表展示
  - 卡片左侧显示订单图片（56×56）
  - 卡片右侧显示：订单名-工序名、数量×单价=金额、状态、时间
  - 按状态筛选（全部/待审核/已通过/已拒绝）
  - 点击卡片跳转详情页
- [x] 6. 实现员工计件详情页 `/my-records/$id`
  - 订单图片轮播
  - 订单信息（产品名、描述、数量、状态）
  - 工序信息（名称、描述、单价）
  - 计件信息（数量、金额、审核状态、录入方式、时间）
- [x] 7. 实现员工录入计件页 `/my-records/new`
  - 订单选择（级联获取工序）
  - 工序选择
  - 数量输入（步进器）
  - 金额显示（只读，自动计算）
  - 提交后显示"待审核"提示
- [x] 8. 更新员工端底部导航，添加"计件"入口
  - 路径从 `/record` 更新为 `/my-records`

## Validation

- [x] 9. pnpm build 验证前端编译（新增代码无错误，预存问题不影响）
- [ ] 10. 手动测试：列表查看 → 详情查看 → 新增录入 → Boss 审核
