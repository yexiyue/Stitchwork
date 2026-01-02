# Change: Add User Data Isolation for List APIs

## Why

当前系统缺少多租户数据隔离，所有 Boss 用户可以看到系统中所有数据。需要确保：
1. Boss 只能看到自己创建的客户及相关订单、工序、计件记录
2. Staff 只能看到自己参与的订单、工序和计件记录

## What Changes

- **数据库**: 在 `order`, `process`, `piece_record` 表添加 `boss_id` 冗余字段
- **Entity**: 更新 Rust entity 定义，添加 `boss_id` 字段
- **Service**: 创建记录时自动填充 `boss_id`
- **Controller**: 列表查询根据用户角色添加过滤条件

## Impact

- Affected code:
  - `server/src/entity/order.rs`
  - `server/src/entity/process.rs`
  - `server/src/entity/piece_record.rs`
  - `server/src/service/customer/controller.rs`
  - `server/src/service/customer/service.rs`
  - `server/src/service/order/controller.rs`
  - `server/src/service/order/service.rs`
  - `server/src/service/process/controller.rs`
  - `server/src/service/process/service.rs`
  - `server/src/service/piece_record/controller.rs`
  - `server/src/service/piece_record/service.rs`
- Database migration required
