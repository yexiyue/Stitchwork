## 1. 后端：数据模型变更

- [x] 1.1 在 `piece_record.rs` 的 `PieceRecordStatus` 枚举中添加 `Settled` 状态
- [x] 1.2 在 `payroll.rs` 添加 `payment_image: Option<String>` 字段
- [x] 1.3 创建 `payroll_record.rs` 关联表实体（payroll_id, piece_record_id）
- [x] 1.4 在 `entity/mod.rs` 中导出新实体
- [x] 1.5 运行 `cargo check` 验证编译通过

## 2. 后端：API 实现

- [x] 2.1 更新 `payroll/dto.rs`：CreatePayrollDto 添加 recordIds 和 paymentImage 字段
- [x] 2.2 更新 `payroll/service.rs`：创建工资单时关联计件并更新状态为 Settled
- [x] 2.3 更新 `payroll/service.rs`：查询详情时返回关联的计件记录
- [x] 2.4 更新 `record/controller.rs`：支持 status=approved 筛选待结算计件
- [x] 2.5 运行 `cargo check` 验证编译通过

## 3. 前端：类型定义

- [x] 3.1 更新 `types/piece-record.ts`：PieceRecordStatus 添加 settled 值
- [x] 3.2 更新 `types/payroll.ts`：Payroll 类型添加 paymentImage 和 paidAt 字段
- [x] 3.3 更新 `types/payroll.ts`：CreatePayrollDto 添加 recordIds 和 paymentImage 字段

## 4. 前端：API 更新

- [x] 4.1 更新 `api/payroll.ts`：create 方法支持新字段
- [x] 4.2 QueryParams 已支持 status 筛选参数

## 5. 前端：工资单页面

- [x] 5.1 创建 `routes/_auth/_boss/payroll/index.tsx`：工资单列表页
- [x] 5.2 创建 `routes/_auth/_boss/payroll/new.tsx`：创建工资单页面
  - 选择员工
  - 勾选待结算计件
  - 自动计算总金额
  - 上传支付凭证
- [x] 5.3 创建 `routes/_auth/_boss/payroll/$id.tsx`：工资单详情页
  - 显示基本信息
  - 显示支付凭证图片
- [x] 5.4 在 `routes/_auth/profile.tsx` 添加工资单管理入口

## 6. 前端：计件列表增强

- [x] 6.1 更新计件记录状态常量：RECORD_STATUS_MAP 添加 settled 状态
- [x] 6.2 更新 RECORD_STATUS_OPTIONS 支持按结算状态筛选
- [x] 6.3 更新 _staff/my-records/$id.tsx 的 STATUS_MAP 添加 settled

## 7. 验证

- [x] 7.1 运行 `pnpm build` 验证前端构建
- [x] 7.2 运行 `cargo check` 验证后端编译
- [ ] 7.3 手动测试完整流程：创建计件 → 审批 → 创建工资单 → 查看详情
