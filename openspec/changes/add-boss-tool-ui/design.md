# Design: 老板端 Tool UI 和前端工具

## Context

项目使用 @assistant-ui/react 实现 AI 助手聊天界面。后端通过 MCP (Model Context Protocol) 定义工具，前端使用 `makeAssistantToolUI` 和 `makeAssistantTool` 注册对应的 UI 组件来美化工具调用结果的展示。

当前员工端已实现完整的 Tool UI：
- `get_my_records` → RecordsToolUi（表格）
- `get_my_earnings` → StatsToolUi（统计卡片 + 趋势图）
- `get_my_payrolls` → PayrollToolUi（表格）
- `get_available_tasks` → AvailableTasksToolUi（任务卡片列表）
- `create_piece_record` → CreateRecordTool（确认表单）

老板端 MCP 工具定义在 `crates/server/src/mcp/boss.rs`，需要对应的前端 UI。

## Goals / Non-Goals

**Goals:**
- 为老板端 6 个 MCP 工具提供美观的 UI 展示
- 复用现有的 DataTable、StatsDisplay 等共享组件
- 保持与员工端 Tool UI 一致的设计风格

**Non-Goals:**
- 不添加新的前端交互工具（如审批、结算操作）
- 不修改后端 MCP 工具逻辑

## Decisions

### 1. 组件结构

遵循员工端已有的目录结构：

```
src/components/tools/
├── orders/              # query_orders
│   ├── index.tsx
│   ├── columns.tsx
│   └── loading.tsx
├── piece-records/       # query_piece_records
│   ├── index.tsx
│   ├── columns.tsx
│   └── loading.tsx
├── worker-stats/        # get_worker_stats
│   ├── index.tsx
│   └── constants.tsx
├── overview/            # get_overview
│   ├── index.tsx
│   └── constants.tsx
├── order-progress/      # get_order_progress
│   └── index.tsx
└── unpaid-summary/      # get_unpaid_summary
    └── index.tsx
```

### 2. 复用共享组件

| 工具 | UI 组件 | 复用组件 |
|------|---------|----------|
| query_orders | 订单表格 | DataTable |
| query_piece_records | 计件记录表格 | DataTable |
| get_worker_stats | 员工排行榜 | StatsDisplay + DataTable |
| get_overview | 概览统计 | StatsDisplay |
| get_order_progress | 进度列表 | 自定义进度条组件 |
| get_unpaid_summary | 待发工资 | StatsDisplay + DataTable |

### 3. 类型映射

后端响应类型 → 前端 TypeScript 类型：

```typescript
// boss.rs → types.ts
OrderListResponse → OrderListResponse
PieceRecordListResponse → BossPieceRecordListResponse
WorkerProductionList → WorkerProductionList
BossOverview → BossOverview
OrderProgressList → OrderProgressList
UnpaidSummaryResponse → UnpaidSummaryResponse
```

### 4. Tool UI 注册

在 `chat.tsx` 中按角色条件注册：

```tsx
<AssistantRuntimeProvider runtime={runtime}>
  {/* 通用组件 */}

  {/* 员工端 Tool UI */}
  <StatsToolUi />
  <RecordsToolUi />
  <PayrollToolUi />
  <AvailableTasksToolUi />
  <CreateRecordTool />

  {/* 老板端 Tool UI */}
  <OrdersToolUi />
  <PieceRecordsToolUi />
  <WorkerStatsToolUi />
  <OverviewToolUi />
  <OrderProgressToolUi />
  <UnpaidSummaryToolUi />
</AssistantRuntimeProvider>
```

## Risks / Trade-offs

- **Risk**: 组件数量增加导致 chat.tsx 变得臃肿
  - **Mitigation**: Tool UI 组件只在对应工具被调用时渲染，不影响性能

- **Risk**: 类型定义与后端不同步
  - **Mitigation**: 使用 JsonSchema 生成的注释作为参考，保持类型一致

## Open Questions

- 是否需要为老板端添加交互工具（如 `approve_record`、`settle_payroll`）？
  - 当前提案仅实现展示型 Tool UI，交互工具可在后续提案中添加
