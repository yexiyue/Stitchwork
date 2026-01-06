## Context

工资结算是服装加工流程的最后一环。老板需要：
1. 选择多个已批准的计件记录
2. 创建工资单进行结算
3. 上传支付凭证（微信/支付宝转账截图）
4. 追溯历史工资单的结算明细

## Goals / Non-Goals

**Goals:**
- 工资单与计件记录建立明确关联
- 计件记录有清晰的结算状态
- 支持上传支付凭证图片
- 自由选择计件进行结算

**Non-Goals:**
- 自动结算（仍需老板手动操作）
- 工资预算/预估功能
- 批量结算多个员工

## Decisions

### 1. 计件状态设计

**决定**: 新增 `Settled` 状态到 `PieceRecordStatus` 枚举

```rust
pub enum PieceRecordStatus {
    Pending,   // 待审批
    Approved,  // 已批准，待结算
    Rejected,  // 已拒绝
    Settled,   // 已结算
}
```

**理由**: 状态枚举更直观，查询时用 `status = 'approved'` 即可获取待结算记录。

**备选方案**: 使用 `payroll_id: Option<Uuid>` 判断是否结算。缺点是需要 JOIN 查询，且语义不够明确。

### 2. 工资单-计件关联设计

**决定**: 新增 `payroll_record` 关联表

```rust
// payroll_record.rs
pub struct Model {
    pub id: Uuid,
    pub payroll_id: Uuid,
    pub piece_record_id: Uuid,
}
```

**理由**:
- 一个工资单可结算多个计件（多对多）
- 便于查询工资单包含哪些计件
- 保持 piece_record 表简洁

### 3. 支付凭证存储

**决定**: 在 payroll 表添加 `payment_image: Option<String>` 字段

**理由**: 复用现有的 S3 图片上传功能，存储图片 URL。

### 4. 结算金额计算

**决定**: 工资单金额由前端计算后传入，后端仅校验

```typescript
// 前端计算
const selectedRecords = records.filter(r => selected.includes(r.id));
const totalAmount = selectedRecords.reduce((sum, r) => sum + r.amount, 0);
```

**理由**: 避免前后端金额计算不一致的问题。后端可选校验总额是否匹配。

## API Design

### 创建工资单

```
POST /api/payrolls
{
  "userId": "uuid",           // 收款员工
  "amount": 1500.00,          // 总金额
  "recordIds": ["uuid", ...], // 关联的计件记录 ID
  "paymentImage": "url",      // 可选：支付凭证图片
  "note": "备注"              // 可选
}
```

**流程**:
1. 验证 recordIds 对应的计件均为 Approved 状态
2. 验证计件均属于当前老板
3. 创建 payroll 记录
4. 创建 payroll_record 关联记录
5. 将相关 piece_record 状态更新为 Settled
6. 返回完整的工资单信息

### 查询待结算计件

```
GET /api/records?status=approved&userId=xxx
```

返回指定员工所有已批准未结算的计件记录。

### 查询工资单详情

```
GET /api/payrolls/:id
```

返回工资单信息及关联的计件记录列表。

## Risks / Trade-offs

**风险**: 结算后无法撤销
- **缓解**: 暂不支持撤销功能，后续可通过删除工资单实现（同时回滚计件状态）

**风险**: 金额计算精度
- **缓解**: 使用 Decimal 类型，前后端均使用字符串传输

## Migration Plan

1. 后端添加 Settled 状态和 payroll_record 表（Entity First 自动同步）
2. 更新 API 支持新的创建逻辑
3. 前端新增工资单管理页面
4. 无需数据迁移（新功能，不影响历史数据）

## Decisions (Clarified)

- **不支持删除工资单**: 工资单一旦创建即为最终记录，避免账目混乱
- **不需要审批流程**: 老板直接发放工资，无需额外审批环节
