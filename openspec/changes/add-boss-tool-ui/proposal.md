# Change: 添加老板端 Tool UI 和前端工具

## Why

当前 AI 助手聊天界面只有员工端的 Tool UI 组件（get_my_records, get_my_earnings, get_my_payrolls, get_available_tasks, create_piece_record）。老板端有 6 个 MCP 工具但缺少对应的前端 UI 组件来美化展示查询结果，导致老板用户看到的是原始 JSON 数据而非格式化的表格和图表。

## What Changes

- 添加老板端 6 个 Tool UI 组件：
  - `query_orders`: 订单列表 UI（表格展示）
  - `query_piece_records`: 计件记录列表 UI（表格展示）
  - `get_worker_stats`: 员工产量统计 UI（统计卡片 + 排行榜）
  - `get_overview`: 首页概览数据 UI（统计卡片）
  - `get_order_progress`: 订单进度列表 UI（进度条展示）
  - `get_unpaid_summary`: 待发工资汇总 UI（统计卡片 + 列表）
- 在 chat.tsx 中注册老板端 Tool UI 组件
- 复用现有的 DataTable、StatsDisplay 等共享组件
- 添加老板端特有的类型定义

## Impact

- Affected specs: assistant-ui (新增)
- Affected code:
  - `src/components/tools/` - 新增老板端工具 UI 组件
  - `src/components/tools/types.ts` - 添加老板端类型定义
  - `src/components/tools/index.tsx` - 导出新组件
  - `src/routes/chat.tsx` - 注册新 Tool UI
