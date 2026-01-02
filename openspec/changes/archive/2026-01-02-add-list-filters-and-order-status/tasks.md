# Tasks

## 1. Order Status Enhancement

- [x] 1.1 在 `order/dto.rs` 定义 OrderStatus 枚举和状态流转校验
- [x] 1.2 修改 `order/service.rs` update_status 添加状态流转校验

## 2. Order List Filtering

- [x] 2.1 在 `order/dto.rs` 添加 OrderQueryParams 结构体（含 customer_id, status）
- [x] 2.2 修改 `order/service.rs` list 方法支持新过滤参数
- [x] 2.3 修改 `order/controller.rs` 使用新的查询参数

## 3. Process List Filtering

- [x] 3.1 在 `process/dto.rs` 添加 ProcessQueryParams 结构体（含 order_id）
- [x] 3.2 修改 `process/service.rs` list 方法支持 order_id 过滤
- [x] 3.3 修改 `process/controller.rs` 使用新的查询参数

## 4. Auto Status Update

- [x] 4.1 修改 `piece_record/service.rs` create 方法，自动将 pending 订单更新为 processing

## 5. Statistics Module

- [x] 5.1 创建 `stats` 模块结构
- [x] 5.2 实现订单进度统计接口 `GET /orders/{id}/stats`
- [x] 5.3 实现客户订单汇总接口 `GET /stats/customers`
- [x] 5.4 实现工人产量统计接口 `GET /stats/workers`
