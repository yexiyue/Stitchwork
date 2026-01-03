# Change: 添加记件记录审批功能与前端布局

## Why

1. 当前记件记录创建后直接生效，无法区分员工自主录入和老板代录的场景
2. 前端缺少整体布局结构（TabBar），无法根据角色展示不同页面

## What Changes

### 后端
- `piece_record` 表添加 `status` 字段（pending/approved/rejected）
- `piece_record` 表添加 `recorded_by` 枚举字段（self/boss）区分录入人
- 添加审批 API（approve/reject）
- 修改查询逻辑支持按状态过滤
- 工资统计只计算 approved 状态的记录

### 前端
- 实现 TabBar 布局组件
- 老板端 TabBar：首页、订单、工序、我的
- 员工端 TabBar：首页、记件、我的
- 老板端"我的"页面包含：客户管理、员工管理入口
- 添加待审核列表页面（老板端）

## Impact

- Affected specs: data-access, frontend-api, statistics, auth-ui
- Affected code:
  - `server/src/entity/piece_record.rs` - 添加字段
  - `server/src/service/piece_record.rs` - 添加审批逻辑
  - `server/migration/` - 数据库迁移
  - `src/routes/_auth.tsx` - 添加 TabBar 布局
  - `src/routes/_auth/` - 添加各 Tab 页面
