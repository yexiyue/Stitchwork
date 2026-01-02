# statistics Specification

## Purpose
TBD - created by archiving change add-list-filters-and-order-status. Update Purpose after archive.
## Requirements
### Requirement: Order Progress Statistics

系统 SHALL 提供订单进度统计接口。

#### Scenario: 获取订单进度统计

- **WHEN** 请求 `GET /orders/{id}/stats`
- **THEN** 返回订单总数量、已完成数量、完成进度和各工序完成情况

### Requirement: Customer Order Summary

系统 SHALL 提供客户订单汇总统计接口。

#### Scenario: 获取客户订单汇总

- **WHEN** 请求 `GET /stats/customers`
- **THEN** 返回每个客户的订单总数和各状态订单数量

### Requirement: Worker Production Statistics

系统 SHALL 提供工人产量统计接口。

#### Scenario: 获取工人产量统计

- **WHEN** 请求 `GET /stats/workers`
- **AND** 可选提供时间范围参数
- **THEN** 返回每个工人的计件总数量和总金额

