## Context

系统需要支持多 Boss 用户，每个 Boss 只能访问自己的数据。Staff 用户只能访问自己参与的数据。

## Goals

- Boss 数据隔离：每个 Boss 只能看到自己创建的客户及关联数据
- Staff 数据访问：Staff 可以看到自己参与的订单和工序
- 查询性能：避免多层 JOIN，使用冗余字段优化查询

## Non-Goals

- 不修改现有的认证机制
- 不修改 Payroll 模块（已有用户过滤）

## Decisions

### 1. 冗余字段设计

在 `order`, `process`, `piece_record` 表添加 `boss_id` 字段：

| 表 | 现有关联 | 新增字段 |
|---|---------|---------|
| `order` | `customer_id` | `boss_id` |
| `process` | `order_id` | `boss_id` |
| `piece_record` | `process_id`, `user_id` | `boss_id` |

**理由**: 避免多层 JOIN 查询，直接通过 `boss_id` 过滤

### 2. 数据访问规则

```
Boss 角色:
├── Customer     → WHERE user_id = boss_id
├── Order        → WHERE boss_id = boss_id
├── Process      → WHERE boss_id = boss_id
└── Piece Record → WHERE boss_id = boss_id

Staff 角色:
├── Order        → WHERE id IN (参与的订单)
├── Process      → WHERE id IN (参与的工序)
└── Piece Record → WHERE user_id = staff_id
```

### 3. Staff 参与数据查询

Staff 通过 `piece_record` 表确定参与关系：

```sql
-- Staff 参与的工序 ID
SELECT DISTINCT process_id FROM piece_record WHERE user_id = ?

-- Staff 参与的订单 ID
SELECT DISTINCT p.order_id FROM process p
JOIN piece_record pr ON pr.process_id = p.id
WHERE pr.user_id = ?
```

## Risks / Trade-offs

| 风险 | 缓解措施 |
|-----|---------|
| 数据冗余增加存储 | `boss_id` 仅 16 字节 UUID，影响可忽略 |
| 数据一致性 | 在 Service 层创建时自动填充，无需手动维护 |
| 历史数据迁移 | 迁移脚本通过关联关系回填 `boss_id` |

## Migration Plan

1. 添加 `boss_id` 字段（允许 NULL）
2. 通过关联关系回填历史数据
3. 设置 `boss_id` 为 NOT NULL
4. 更新 Service 层代码
