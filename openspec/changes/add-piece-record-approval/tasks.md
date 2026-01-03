## 1. Database Schema

- [x] 1.1 添加 `PieceRecordStatus` 枚举（pending/approved/rejected）
- [x] 1.2 添加 `RecordedBy` 枚举（self/boss）
- [x] 1.3 修改 `piece_record` entity 添加 `status` 和 `recorded_by` 字段
- [x] ~~1.4 创建数据库迁移脚本~~ (不需要)

## 2. Backend API - 记件审批

- [x] 2.1 修改创建记件记录逻辑，根据角色设置 status 和 recorded_by
- [x] 2.2 添加 `POST /piece-records/:id/approve` 审批通过接口
- [x] 2.3 添加 `POST /piece-records/:id/reject` 审批驳回接口
- [x] 2.4 修改查询接口支持按 status 过滤
- [x] 2.5 修改统计接口只计算 approved 状态

## 3. Backend API - 员工数据访问

- [x] 3.1 修改订单查询接口，Staff 可查看工坊所有订单（隐藏客户详情）
- [x] 3.2 修改工序查询接口，Staff 可查看工坊所有工序
- [x] 3.3 添加订单/工序的 Staff 视图 DTO（过滤敏感字段）

## 4. Frontend API

- [x] 4.1 更新 piece-record API 类型定义
- [x] 4.2 添加 approve/reject API 调用方法

## 5. Frontend Layout

- [x] 5.1 在 `_auth.tsx` 实现 TabBar 布局组件
- [x] 5.2 根据用户角色渲染不同的 TabBar 项
- [x] 5.3 创建老板端页面：首页、订单、工序、我的
- [x] 5.4 创建员工端页面：首页、工序、记件、我的
- [x] 5.5 实现"我的"页面（客户/员工管理入口）
- [x] 5.6 实现待审核列表页面
