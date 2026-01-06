# Change: 完善工资单结算流程

## Why

当前工资单（payroll）功能未完善：
- 工资单没有关联具体的计件记录，无法追溯结算明细
- 计件记录没有"已结算"状态，无法区分哪些已发工资
- 没有支付凭证功能，无法上传微信/支付宝转账截图

## What Changes

### 数据模型

- **piece_record**: 新增 `Settled` 状态，表示该计件已被工资单结算
- **payroll**: 新增 `payment_image` 字段存储支付凭证图片 URL
- **payroll_record**: 新增关联表，关联工资单与多个计件记录（多对多）

### API

- **POST /api/payrolls**: 支持传入 `recordIds` 数组，创建工资单并关联计件
- **GET /api/payrolls/:id**: 返回关联的计件记录列表
- **GET /api/records**: 支持 `settled=false` 筛选未结算的已批准计件

### 前端

- 新增工资单管理页面（列表、创建、详情）
- 创建工资单时可勾选多个已批准计件
- 支持上传支付凭证图片
- 计件列表显示结算状态

## Impact

- Affected specs: `payroll` (新增), `piece-record` (新增/修改)
- Affected code:
  - `server/src/entity/payroll.rs` - 添加 payment_image 字段
  - `server/src/entity/piece_record.rs` - 添加 Settled 状态
  - `server/src/entity/payroll_record.rs` - 新增关联表
  - `server/src/service/payroll/` - 更新 DTO 和逻辑
  - `src/routes/_auth/_boss/payroll/` - 新增前端页面
  - `src/api/payroll.ts` - 更新 API 类型
