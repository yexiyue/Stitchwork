## Context

服装加工流程需要清晰的订单状态流转，按客户/订单维度查看数据的能力，以及统计功能来了解业务进度。

## Goals

- 订单可按客户和状态筛选
- 工序可按订单筛选
- 订单状态有明确的流转规则
- 计件记录自动触发订单状态变更
- 提供统计接口了解业务进度

## Non-Goals

- 不修改工序的数据结构（工序不需要状态）
- 不做复杂的报表功能

## Decisions

### 1. 订单状态流转

```
pending → processing → completed → delivered
   ↓          ↓
cancelled  cancelled
```

| 状态 | 含义 | 允许的下一状态 |
|-----|------|--------------|
| `pending` | 待处理，刚接单 | `processing`, `cancelled` |
| `processing` | 加工中 | `completed`, `cancelled` |
| `completed` | 加工完成，待交付 | `delivered` |
| `delivered` | 已交付 | (终态) |
| `cancelled` | 已取消 | (终态) |

### 2. 自动状态更新

创建计件记录时：
- 如果订单状态为 `pending`，自动更新为 `processing`

手动更新状态：
- `completed` 和 `delivered` 需要手动触发

### 3. 过滤参数扩展

订单列表新增：
- `customer_id: Option<Uuid>` - 按客户筛选
- `status: Option<String>` - 按状态筛选

工序列表新增：
- `order_id: Option<Uuid>` - 按订单筛选

### 4. 统计接口设计

#### 4.1 订单进度统计 `GET /orders/{id}/stats`

```json
{
  "orderId": "uuid",
  "totalQuantity": 100,        // 订单总数量
  "completedQuantity": 75,     // 已完成数量（所有工序计件总和）
  "progress": 0.75,            // 完成进度
  "processes": [
    {
      "processId": "uuid",
      "name": "裁剪",
      "completedQuantity": 80
    }
  ]
}
```

#### 4.2 客户订单汇总 `GET /stats/customers`

```json
{
  "list": [
    {
      "customerId": "uuid",
      "customerName": "张三",
      "totalOrders": 10,
      "pendingOrders": 2,
      "processingOrders": 5,
      "completedOrders": 3
    }
  ]
}
```

#### 4.3 工人产量统计 `GET /stats/workers`

```json
{
  "list": [
    {
      "userId": "uuid",
      "userName": "李四",
      "totalQuantity": 500,
      "totalAmount": 2500.00,
      "period": "2024-01"
    }
  ]
}
```

### 5. 实现方式

- 使用独立的 QueryParams 结构体
- 统计接口放在新的 `stats` 模块
- 自动状态更新在 piece_record create 中触发
