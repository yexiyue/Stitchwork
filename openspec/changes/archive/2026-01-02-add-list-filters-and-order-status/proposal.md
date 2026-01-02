# Change: Add List Filters, Order Status Flow and Statistics

## Why

1. 订单列表缺少按客户筛选功能，工序列表缺少按订单筛选功能
2. 订单状态流转不明确，当前只有 pending 和 delivered 两个状态
3. 缺少统计功能：订单完成进度、工人产量、客户订单汇总等

## What Changes

- **订单列表**: 添加 `customer_id` 和 `status` 过滤参数
- **工序列表**: 添加 `order_id` 过滤参数
- **订单状态**: 定义明确的状态枚举和流转规则
  - `pending` → `processing` → `completed` → `delivered`
- **自动状态更新**: 创建计件记录时自动将订单状态从 pending 更新为 processing
- **统计接口**:
  - 订单进度统计（已完成数量/总数量）
  - 客户订单汇总
  - 工人产量统计

## Impact

- Affected code:
  - `server/src/entity/order.rs` - 状态枚举
  - `server/src/service/order/service.rs` - 过滤和状态校验
  - `server/src/service/process/service.rs` - order_id 过滤
  - `server/src/service/piece_record/service.rs` - 创建时自动更新订单状态
  - `server/src/service/stats/` - 新增统计模块
