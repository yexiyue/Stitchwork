# Assistant UI Tool Components

## ADDED Requirements

### Requirement: Boss Orders Tool UI

系统 SHALL 为老板端 `query_orders` MCP 工具提供 Tool UI 组件，以表格形式展示订单列表。

#### Scenario: 订单查询结果展示

- **WHEN** AI 助手调用 `query_orders` 工具返回订单列表
- **THEN** 系统使用 DataTable 组件展示订单信息
- **AND** 表格包含产品名称、客户名称、数量、状态、创建日期等列
- **AND** 加载中显示骨架屏加载状态

### Requirement: Boss Piece Records Tool UI

系统 SHALL 为老板端 `query_piece_records` MCP 工具提供 Tool UI 组件，以表格形式展示计件记录列表。

#### Scenario: 计件记录查询结果展示

- **WHEN** AI 助手调用 `query_piece_records` 工具返回计件记录
- **THEN** 系统使用 DataTable 组件展示计件记录
- **AND** 表格包含员工姓名、工序名称、订单名称、数量、金额、状态、记录时间等列
- **AND** 状态列使用不同颜色标签区分 pending/approved/rejected/settled

### Requirement: Boss Worker Stats Tool UI

系统 SHALL 为老板端 `get_worker_stats` MCP 工具提供 Tool UI 组件，展示员工产量统计。

#### Scenario: 员工产量统计展示

- **WHEN** AI 助手调用 `get_worker_stats` 工具返回员工产量数据
- **THEN** 系统展示汇总统计卡片（总数量、总金额）
- **AND** 展示员工产量排行榜表格
- **AND** 加载中显示统计卡片骨架屏

### Requirement: Boss Overview Tool UI

系统 SHALL 为老板端 `get_overview` MCP 工具提供 Tool UI 组件，展示首页概览数据。

#### Scenario: 首页概览数据展示

- **WHEN** AI 助手调用 `get_overview` 工具返回概览数据
- **THEN** 系统使用 StatsDisplay 组件展示统计卡片
- **AND** 包含待审批数、进行中订单数、今日产量、今日金额、本月产量、本月金额

### Requirement: Boss Order Progress Tool UI

系统 SHALL 为老板端 `get_order_progress` MCP 工具提供 Tool UI 组件，展示订单进度列表。

#### Scenario: 订单进度列表展示

- **WHEN** AI 助手调用 `get_order_progress` 工具返回订单进度数据
- **THEN** 系统展示订单进度卡片列表
- **AND** 每个卡片包含订单名称、完成数量/总数量、进度百分比
- **AND** 使用进度条可视化展示完成进度

### Requirement: Boss Unpaid Summary Tool UI

系统 SHALL 为老板端 `get_unpaid_summary` MCP 工具提供 Tool UI 组件，展示待发工资汇总。

#### Scenario: 待发工资汇总展示

- **WHEN** AI 助手调用 `get_unpaid_summary` 工具返回待发工资数据
- **THEN** 系统展示汇总统计（总待发金额、总待发数量、员工人数）
- **AND** 展示各员工待发工资明细列表
- **AND** 列表包含员工姓名、待发数量、待发金额
