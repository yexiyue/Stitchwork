# group-stats-api Specification

## Purpose

提供按订单/工序分组的统计数据 API，支持堆叠柱状图展示。

## ADDED Requirements

### Requirement: Group Stats API

后端 SHALL 提供分组统计 API。

#### Scenario: 按订单分组统计

- Given 请求 `GET /api/stats/by-order`
- When 提供 startDate 和 endDate 参数
- Then 返回每个订单的计件统计
- And 包含：订单ID、订单名称、总数量、总金额

#### Scenario: 按工序分组统计

- Given 请求 `GET /api/stats/by-process`
- When 提供 startDate 和 endDate 参数
- Then 返回每个工序的计件统计
- And 包含：工序ID、工序名称、总数量、总金额

#### Scenario: 员工权限过滤

- Given 员工用户请求
- When 调用分组统计 API
- Then 仅返回该员工个人的分组数据

#### Scenario: 空数据处理

- Given 请求的日期范围内无数据
- When 查询统计
- Then 返回空数组

## API Response Format

```json
{
  "code": 0,
  "data": {
    "list": [
      { "id": 1, "name": "订单A", "totalQuantity": 100, "totalAmount": "500.00" },
      { "id": 2, "name": "订单B", "totalQuantity": 80, "totalAmount": "400.00" }
    ]
  }
}
```

## Cross-references

- staff-charts-ui: 堆叠柱状图使用此 API
