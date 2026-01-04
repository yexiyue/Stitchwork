# daily-stats-api Specification

## Purpose

提供按日统计的时间序列数据 API，支持趋势图展示。

## ADDED Requirements

### Requirement: Daily Stats API

后端 SHALL 提供按日统计 API。

#### Scenario: 获取每日统计
- Given 请求 `GET /api/stats/daily`
- When 提供 startDate 和 endDate 参数
- Then 返回日期范围内每日的计件统计
- And 包含：日期、总数量、总金额

#### Scenario: 空数据处理
- Given 请求的日期范围内无数据
- When 查询统计
- Then 返回空数组

#### Scenario: 老板权限
- Given 老板用户请求
- When 调用 API
- Then 返回该老板下所有员工的汇总数据

#### Scenario: 员工权限
- Given 员工用户请求
- When 调用 API
- Then 仅返回该员工个人的统计数据

## API Response Format

```json
{
  "code": 0,
  "data": {
    "list": [
      { "date": "2026-01-01", "totalQuantity": 100, "totalAmount": "500.00" },
      { "date": "2026-01-02", "totalQuantity": 150, "totalAmount": "750.00" }
    ]
  }
}
```

## Cross-references

- stats-charts-ui: 趋势图使用此 API
