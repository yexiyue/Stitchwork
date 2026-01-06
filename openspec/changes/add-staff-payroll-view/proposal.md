# Change: 员工工资单查看页面

## Why

员工目前只能查看自己的计件记录，无法查看工资发放历史。后端 API 已支持员工按 `user_id` 筛选自己的工资单，但前端缺少对应页面。员工需要一个地方查看：

- 工资发放历史列表
- 每笔工资的详情（金额、备注、支付凭证）
- 关联的计件记录明细

## What Changes

### 前端

- 新增 `_staff/my-payrolls/index.tsx` - 工资单列表页
- 新增 `_staff/my-payrolls/$id.tsx` - 工资单详情页
- 在 `profile.tsx` 员工"我的"区块添加"工资记录"入口

### 后端

无需修改，现有 API 已支持：
- `GET /api/payrolls` - Staff 角色自动按 `user_id` 过滤
- `GET /api/payrolls/:id` - Staff 角色验证所有权

## Impact

- Affected specs: `staff-payroll` (新增)
- Affected code:
  - `src/routes/_auth/_staff/my-payrolls/index.tsx` - 新增
  - `src/routes/_auth/_staff/my-payrolls/$id.tsx` - 新增
