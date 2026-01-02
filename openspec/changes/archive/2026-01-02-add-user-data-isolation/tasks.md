# Tasks

## 1. Entity Updates

- [x] 1.1 更新 `order.rs` entity，添加 `boss_id` 字段
- [x] 1.2 更新 `process.rs` entity，添加 `boss_id` 字段
- [x] 1.3 更新 `piece_record.rs` entity，添加 `boss_id` 字段

## 2. Service Layer - Boss ID Auto-fill

- [x] 2.1 修改 `order/service.rs` 创建逻辑，自动填充 `boss_id`
- [x] 2.2 修改 `process/service.rs` 创建逻辑，自动填充 `boss_id`
- [x] 2.3 修改 `piece_record/service.rs` 创建逻辑，自动填充 `boss_id`

## 3. List API Filtering

- [x] 3.1 修改 `customer/service.rs` list，Boss 按 `user_id` 过滤
- [x] 3.2 修改 `order/service.rs` list，Boss 按 `boss_id` 过滤，Staff 按参与过滤
- [x] 3.3 修改 `process/service.rs` list，Boss 按 `boss_id` 过滤，Staff 按参与过滤
- [x] 3.4 修改 `piece_record/service.rs` list，Boss 按 `boss_id` 过滤，Staff 按 `user_id` 过滤

## 4. Update/Delete Ownership Validation

- [x] 4.1 修改 `customer/service.rs` update/delete，校验 `user_id` 归属
- [x] 4.2 修改 `order/service.rs` update/delete，校验 `boss_id` 归属
- [x] 4.3 修改 `process/service.rs` update/delete，校验 `boss_id` 归属
- [x] 4.4 修改 `piece_record/service.rs` update/delete，校验 `boss_id` 归属

## 5. Controller Updates

- [x] 5.1 更新各 controller，传递用户信息到 service 层
