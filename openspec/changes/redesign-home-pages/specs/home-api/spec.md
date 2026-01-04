# home-api Specification

## Purpose

提供首页数据概览和最近动态的后端 API。

## ADDED Requirements

### Requirement: Home Overview API

后端 SHALL 提供首页概览数据 API。

#### Scenario: 老板获取概览数据

- Given 请求 `GET /api/home/overview`
- When 老板用户调用
- Then 返回待审核计件数量
- And 返回进行中订单数量
- And 返回今日产量和金额
- And 返回本月产量和金额
- And 返回员工数量

#### Scenario: 员工获取概览数据

- Given 请求 `GET /api/home/overview`
- When 员工用户调用
- Then 返回个人本月产量
- And 返回个人本月金额

### Requirement: Activities API

后端 SHALL 提供最近动态 API。

#### Scenario: 老板获取最近动态

- Given 请求 `GET /api/home/activities`
- When 老板用户调用
- Then 返回最近的计件提交事件
- And 返回最近的审核事件
- And 按时间倒序排列

#### Scenario: 员工获取最近记录

- Given 请求 `GET /api/home/activities`
- When 员工用户调用
- Then 返回个人最近的计件记录
- And 按时间倒序排列

## API Response Format

### Overview (Boss)

```json
{
  "code": 0,
  "data": {
    "pendingCount": 5,
    "processingOrderCount": 12,
    "todayQuantity": 328,
    "todayAmount": "1640.00",
    "monthQuantity": 5280,
    "monthAmount": "26400.00",
    "staffCount": 8
  }
}
```

### Overview (Staff)

```json
{
  "code": 0,
  "data": {
    "monthQuantity": 420,
    "monthAmount": "2100.00"
  }
}
```

### Activities

```json
{
  "code": 0,
  "data": {
    "list": [
      {
        "id": "uuid",
        "type": "submit|approve|reject",
        "userName": "张三",
        "orderName": "订单A",
        "processName": "裁剪",
        "quantity": 50,
        "createdAt": "2026-01-04T10:30:00Z"
      }
    ]
  }
}
```

## Cross-references

- boss-home-ui: 老板端首页使用此 API
- staff-home-ui: 员工端首页使用此 API
