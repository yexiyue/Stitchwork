# Assistant-UI Tool UI 使用指南

本文档介绍如何在 assistant-ui 中自定义 Tool 的 UI 展示，基于 [Tool UI](https://www.tool-ui.com/) 框架的设计理念。

## 概述

Tool UI 是一个用于构建"对话原生 UI"的 React 组件框架。当 AI 助手调用后端工具（Tool）时，工具返回 JSON 数据，Tool UI 将其渲染为内联的、可引用的界面组件。

### 核心理念

- **对话原生组件**: 组件生活在消息内部，针对聊天宽度和滚动优化
- **Schema 驱动渲染**: 每个界面都由可序列化的 schema 驱动，具有稳定的 ID
- **助手锚定**: 助手介绍、解释和关闭每个界面
- **技术栈无关**: 适用于任何 LLM 提供商和编排层

### 组件角色

Tool UI 组件分为以下几种角色：

| 角色 | 描述 | 示例 |
|------|------|------|
| `information` | 展示信息，只读 | 概览卡片、统计数据 |
| `decision` | 需要用户做出选择 | 选项列表、确认对话框 |
| `control` | 用户可以操作的控件 | 表单、筛选器 |
| `state` | 展示进度或状态 | 进度条、加载状态 |
| `composite` | 组合多种角色 | 带操作按钮的数据表格 |

### 生命周期

Tool UI 组件的生命周期：

```
invocation → output-pending → interactive → committing → receipt → errored
   调用          等待输出        可交互        提交中       回执      错误
```

## 注册 Tool UI 的两种方式

### 方式一：MessagePrimitive.Parts (推荐)

通过 `MessagePrimitive.Parts` 的 `components.tools` 属性配置，适用于后端工具：

```tsx
// src/components/thread.tsx
import { ToolFallback } from "@/components/tool-fallback";
import { OverviewToolUI } from "@/components/tool-ui/overview-tool";
import { OrderListToolUI } from "@/components/tool-ui/order-list-tool";

const AssistantMessage: FC = () => {
  return (
    <MessagePrimitive.Root>
      <MessagePrimitive.Parts
        components={{
          Text: MarkdownText,
          tools: {
            by_name: {
              // 按工具名称指定渲染组件
              get_overview: OverviewToolUI,
              query_orders: OrderListToolUI,
            },
            // 未匹配到的工具使用 Fallback
            Fallback: ToolFallback,
          },
        }}
      />
    </MessagePrimitive.Root>
  );
};
```

### 方式二：makeAssistantToolUI

使用 `makeAssistantToolUI` 创建独立的 Tool UI 组件，然后挂载到 Provider 下：

```tsx
// src/components/tool-ui/preview-link-ui.tsx
import { makeAssistantToolUI } from "@assistant-ui/react";

export const PreviewLinkUI = makeAssistantToolUI({
  toolName: "previewLink",  // 必须与后端工具名称匹配
  render: ({ result, status }) => {
    // result 在工具完成前是 undefined
    if (result === undefined) {
      return <div className="animate-pulse">加载中...</div>;
    }

    // 渲染结果
    return <LinkPreview {...result} />;
  },
});

// 在 App 中注册
function App() {
  return (
    <AssistantRuntimeProvider runtime={runtime}>
      <PreviewLinkUI />  {/* 挂载即注册 */}
      <Thread />
    </AssistantRuntimeProvider>
  );
}
```

### 方式三：makeAssistantTool (前端交互工具)

用于需要用户交互的前端工具，工具参数作为 UI 的 props，用户操作后调用 `addResult`：

```tsx
import { makeAssistantTool } from "@assistant-ui/react";

export const SelectFormatTool = makeAssistantTool<
  SerializableOptionList,  // 参数类型
  OptionListSelection      // 结果类型
>({
  toolName: "selectFormat",
  description: "让用户选择输出格式",
  parameters: SerializableOptionListSchema,
  render: ({ args, result, addResult }) => {
    // 已有结果时显示回执状态
    if (result !== undefined) {
      return <OptionList {...args} confirmed={result} />;
    }

    // 等待用户选择
    return (
      <OptionList
        {...args}
        onConfirm={(selection) => addResult(selection)}
      />
    );
  },
});
```

## 后端工具 vs 前端交互工具

理解这两种工具的区别对于正确选择实现方式至关重要。

### 后端工具 (Backend Tool)

使用 `makeAssistantToolUI` 或 `MessagePrimitive.Parts` 配置。

**工作流程：**

```
LLM 调用工具 → 后端执行逻辑 → 返回 JSON 结果 → UI 渲染结果
```

**示例：** `get_overview` 工具
1. LLM 决定调用 `get_overview`
2. 请求发送到 Rust 后端
3. 后端查询数据库，计算统计数据
4. 返回 JSON 结果 `{ pendingCount: 5, todayQuantity: 120, ... }`
5. 前端 `OverviewToolUI` 组件渲染概览卡片

**特点：**
- 工具逻辑在服务端执行
- 结果由后端计算后返回
- UI 只负责展示结果
- 适用于：数据查询、API 调用、计算任务

### 前端交互工具 (Frontend Interactive Tool)

使用 `makeAssistantTool` 创建。

**工作流程：**

```
LLM 调用工具 → UI 渲染（参数作为 props）→ 等待用户交互 → 用户操作作为结果返回给 LLM
```

**示例：** `selectFormat` 工具
1. LLM 决定需要用户选择格式，调用工具：`{ options: ["PDF", "Excel", "CSV"] }`
2. 前端渲染选项列表 UI，显示三个按钮
3. **等待用户交互** - 用户点击选择 "PDF"
4. 调用 `addResult("PDF")` 把用户选择作为工具结果返回
5. LLM 收到结果，继续对话："好的，我将生成 PDF 格式的报表..."

**特点：**
- 工具定义在前端（客户端）
- **用户的操作/选择就是工具的返回结果**
- 实现 "人在回路"（Human-in-the-loop）模式
- 适用于：用户确认、选项选择、表单填写、审批流程

### 对比总结

| 特性 | 后端工具 | 前端交互工具 |
| ---- | -------- | ------------ |
| 工具定义位置 | 服务端 (Rust/Node) | 客户端 (React) |
| 结果来源 | 后端计算 | 用户交互 |
| API | `makeAssistantToolUI` | `makeAssistantTool` |
| 典型用例 | 查询数据、调用 API | 用户选择、确认操作 |
| 是否阻塞 | 否（后端异步执行） | 是（等待用户操作） |

### 前端交互工具的典型场景

| 场景 | 描述 | 示例 |
| ---- | ---- | ---- |
| 选项列表 | 让用户从多个选项中选择 | 选择导出格式、选择收件人 |
| 确认对话框 | 危险操作前让用户确认 | 删除确认、批量操作确认 |
| 表单填写 | 收集用户输入信息 | 填写备注、输入数量 |
| 审批流程 | 批准/拒绝待审批项 | 审批计件记录、审批工资单 |

### 自动继续对话

用户操作完成后，可以配置 LLM 自动继续对话（而不是等待用户下一条消息）：

```tsx
import { useChatRuntime, AssistantChatTransport } from "@assistant-ui/react-ai-sdk";
import { lastAssistantMessageIsCompleteWithToolCalls } from "ai";

const runtime = useChatRuntime({
  transport: new AssistantChatTransport({ api: "/api/chat" }),
  // 当助手消息包含已完成的工具调用时，自动重新提交
  sendAutomaticallyWhen: lastAssistantMessageIsCompleteWithToolCalls,
});
```

## Tool UI 组件接口

每个 Tool UI 组件接收以下 props：

```typescript
interface ToolCallContentPartProps {
  toolName: string;      // 工具名称
  argsText: string;      // 参数的 JSON 字符串
  args: unknown;         // 解析后的参数对象
  result?: unknown;      // 工具返回结果（完成前为 undefined）
  status?: {
    type: "running" | "complete" | "incomplete";
    reason?: "cancelled" | "error" | "length";
    error?: unknown;
  };
}
```

## Response Actions（响应操作）

Response Actions 是轻量级的 CTA 按钮，用于人机协作的决策点。当 AI 完成工作后等待用户确认时使用。

```tsx
import { DataTable } from "@/components/tool-ui/data-table";

<DataTable
  id="flagged-expenses"
  columns={columns}
  data={data}
  responseActions={{
    items: [
      { id: "reject", label: "全部拒绝", variant: "destructive", confirmLabel: "确认" },
      { id: "approve", label: "全部批准", variant: "default", confirmLabel: "确认" },
    ],
    align: "right",
  }}
  onResponseAction={(id) => {
    if (id === "approve") approveAll();
    if (id === "reject") rejectAll();
  }}
/>
```

### Action 属性

| 属性 | 类型 | 描述 |
|------|------|------|
| `id` | string | 操作标识符 |
| `label` | string | 按钮文本 |
| `variant` | "default" \| "secondary" \| "destructive" \| "outline" \| "ghost" | 按钮样式 |
| `confirmLabel` | string? | 确认文本（用于危险操作的二次确认） |

## 项目中的后端 Tools

### 老板端工具 (Boss MCP)

| 工具名称 | 描述 | 参数 |
|---------|------|------|
| `query_orders` | 查询订单列表 | status, search, start_date, end_date, page, page_size |
| `query_piece_records` | 查询计件记录列表 | status, start_date, end_date, page, page_size |
| `get_worker_stats` | 获取员工产量统计 | start_date, end_date |
| `get_overview` | 获取首页概览数据 | 无 |
| `get_order_progress` | 获取订单进度列表 | start_date, end_date |
| `get_unpaid_summary` | 获取待发工资汇总 | user_name |

### 员工端工具 (Staff MCP)

| 工具名称 | 描述 | 参数 |
|---------|------|------|
| `get_my_records` | 查询我的计件记录 | status, start_date, end_date, page, page_size |
| `get_my_earnings` | 查询我的收入统计 | start_date, end_date |
| `get_my_payrolls` | 查询我的工资单 | start_date, end_date, page, page_size |
| `get_available_tasks` | 查看可接工序 | 无 |

## 实现示例

### 1. 首页概览卡片 (get_overview)

展示信息角色（`role: "information"`）的典型示例：

```tsx
// src/components/tool-ui/overview-tool.tsx
import type { ToolCallContentPartComponent } from "@assistant-ui/react";
import { Card } from "antd-mobile";

interface BossOverview {
  pendingCount: number;
  processingOrders: number;
  todayQuantity: number;
  todayAmount: string;
  monthQuantity: number;
  monthAmount: string;
}

export const OverviewToolUI: ToolCallContentPartComponent = ({
  result,
  status,
}) => {
  // 1. 运行中 - 显示加载状态
  if (status?.type === "running") {
    return (
      <div className="grid grid-cols-2 gap-2 my-2">
        {[1, 2, 3, 4].map((i) => (
          <Card key={i} className="animate-pulse bg-gray-100 h-20" />
        ))}
      </div>
    );
  }

  // 2. 失败 - 显示错误
  if (status?.type === "incomplete") {
    return (
      <div className="text-red-500 text-sm py-2">
        获取概览数据失败
      </div>
    );
  }

  // 3. 完成 - 显示结果
  if (!result) return null;

  const data = result as BossOverview;

  return (
    <div className="grid grid-cols-2 gap-2 my-2">
      <Card className="bg-blue-50">
        <div className="text-sm text-gray-500">待审批</div>
        <div className="text-xl font-bold text-blue-600">{data.pendingCount}</div>
      </Card>
      <Card className="bg-green-50">
        <div className="text-sm text-gray-500">进行中订单</div>
        <div className="text-xl font-bold text-green-600">{data.processingOrders}</div>
      </Card>
      <Card className="bg-orange-50">
        <div className="text-sm text-gray-500">今日产量</div>
        <div className="text-xl font-bold text-orange-600">{data.todayQuantity}</div>
        <div className="text-xs text-gray-400">¥{data.todayAmount}</div>
      </Card>
      <Card className="bg-purple-50">
        <div className="text-sm text-gray-500">本月产量</div>
        <div className="text-xl font-bold text-purple-600">{data.monthQuantity}</div>
        <div className="text-xs text-gray-400">¥{data.monthAmount}</div>
      </Card>
    </div>
  );
};
```

### 2. 订单列表 (query_orders)

带分页和状态标签的列表组件：

```tsx
// src/components/tool-ui/order-list-tool.tsx
import type { ToolCallContentPartComponent } from "@assistant-ui/react";
import { List, Tag } from "antd-mobile";

interface Order {
  id: string;
  productName: string;
  status: string;
  quantity: number;
  createdAt: string;
}

interface OrderListResult {
  list: Order[];
  total: number;
}

const statusMap: Record<string, { color: string; text: string }> = {
  pending: { color: "default", text: "待处理" },
  processing: { color: "primary", text: "进行中" },
  completed: { color: "success", text: "已完成" },
  delivered: { color: "success", text: "已出货" },
  cancelled: { color: "danger", text: "已取消" },
};

export const OrderListToolUI: ToolCallContentPartComponent = ({
  result,
  status,
}) => {
  if (status?.type === "running") {
    return <div className="animate-pulse py-4">查询订单中...</div>;
  }

  if (!result) return null;

  const data = result as OrderListResult;

  if (data.list.length === 0) {
    return <div className="text-gray-500 text-center py-4">暂无订单</div>;
  }

  return (
    <div className="my-2 rounded-lg border overflow-hidden">
      <div className="bg-gray-50 px-3 py-2 text-sm text-gray-500">
        共 {data.total} 条订单
      </div>
      <List>
        {data.list.slice(0, 5).map((order) => (
          <List.Item
            key={order.id}
            description={`数量: ${order.quantity}`}
            extra={
              <Tag color={statusMap[order.status]?.color}>
                {statusMap[order.status]?.text || order.status}
              </Tag>
            }
          >
            {order.productName}
          </List.Item>
        ))}
      </List>
      {data.list.length > 5 && (
        <div className="text-center text-sm text-gray-400 py-2">
          还有 {data.total - 5} 条...
        </div>
      )}
    </div>
  );
};
```

### 3. 收入统计 (get_my_earnings)

带图表的统计组件：

```tsx
// src/components/tool-ui/earnings-tool.tsx
import type { ToolCallContentPartComponent } from "@assistant-ui/react";
import { ProgressBar } from "antd-mobile";

interface DailyStat {
  date: string;
  totalQuantity: number;
  totalAmount: string;
}

interface EarningsSummary {
  totalQuantity: number;
  totalAmount: string;
  pendingQuantity: number;
  pendingAmount: string;
  dailyStats: DailyStat[];
}

export const EarningsToolUI: ToolCallContentPartComponent = ({
  result,
  status,
}) => {
  if (status?.type === "running") {
    return <div className="animate-pulse py-4">统计中...</div>;
  }

  if (!result) return null;

  const data = result as EarningsSummary;
  const total = parseFloat(data.totalAmount) + parseFloat(data.pendingAmount);
  const approvedPercent = total > 0
    ? (parseFloat(data.totalAmount) / total) * 100
    : 0;

  return (
    <div className="my-2 p-4 rounded-lg border bg-gradient-to-r from-green-50 to-blue-50">
      <div className="text-center mb-4">
        <div className="text-sm text-gray-500">已结算金额</div>
        <div className="text-3xl font-bold text-green-600">
          ¥{data.totalAmount}
        </div>
        <div className="text-sm text-gray-400">
          共 {data.totalQuantity} 件
        </div>
      </div>

      <div className="mb-2">
        <div className="flex justify-between text-xs text-gray-500 mb-1">
          <span>结算进度</span>
          <span>{approvedPercent.toFixed(1)}%</span>
        </div>
        <ProgressBar percent={approvedPercent} />
      </div>

      <div className="text-center text-sm">
        <span className="text-orange-500">
          待审批: ¥{data.pendingAmount} ({data.pendingQuantity}件)
        </span>
      </div>
    </div>
  );
};
```

### 4. 待发工资汇总 (get_unpaid_summary) - 带操作按钮

展示带 Response Actions 的组件：

```tsx
// src/components/tool-ui/unpaid-summary-tool.tsx
import type { ToolCallContentPartComponent } from "@assistant-ui/react";
import { List, Button } from "antd-mobile";

interface UnpaidSummaryItem {
  userId: string;
  userName: string;
  totalQuantity: number;
  totalAmount: string;
}

interface UnpaidSummaryResult {
  list: UnpaidSummaryItem[];
}

export const UnpaidSummaryToolUI: ToolCallContentPartComponent = ({
  result,
  status,
}) => {
  if (status?.type === "running") {
    return <div className="animate-pulse py-4">统计待发工资中...</div>;
  }

  if (!result) return null;

  const data = result as UnpaidSummaryResult;

  if (data.list.length === 0) {
    return (
      <div className="text-gray-500 text-center py-4">
        暂无待发工资
      </div>
    );
  }

  const totalAmount = data.list.reduce(
    (sum, item) => sum + parseFloat(item.totalAmount),
    0
  );

  return (
    <div className="my-2 rounded-lg border overflow-hidden">
      <div className="bg-orange-50 px-3 py-2 flex justify-between items-center">
        <span className="text-sm text-gray-600">
          待发工资汇总 ({data.list.length}人)
        </span>
        <span className="text-lg font-bold text-orange-600">
          ¥{totalAmount.toFixed(2)}
        </span>
      </div>
      <List>
        {data.list.map((item) => (
          <List.Item
            key={item.userId}
            description={`${item.totalQuantity} 件`}
            extra={
              <span className="text-green-600 font-medium">
                ¥{item.totalAmount}
              </span>
            }
          >
            {item.userName}
          </List.Item>
        ))}
      </List>
      {/* Response Actions */}
      <div className="flex justify-end gap-2 p-3 border-t bg-gray-50">
        <Button size="small" color="default">
          导出明细
        </Button>
        <Button size="small" color="primary">
          批量发放
        </Button>
      </div>
    </div>
  );
};
```

### 5. 可接工序列表 (get_available_tasks)

员工端的任务列表，可考虑添加"接单"交互：

```tsx
// src/components/tool-ui/tasks-tool.tsx
import type { ToolCallContentPartComponent } from "@assistant-ui/react";
import { List, Image } from "antd-mobile";

interface AvailableTask {
  processId: string;
  processName: string;
  piecePrice: string;
  orderId: string;
  orderName: string;
  orderImage?: string;
  remainingQuantity: number;
}

interface TasksResult {
  list: AvailableTask[];
}

export const AvailableTasksToolUI: ToolCallContentPartComponent = ({
  result,
  status,
}) => {
  if (status?.type === "running") {
    return <div className="animate-pulse py-4">查询可接工序中...</div>;
  }

  if (!result) return null;

  const data = result as TasksResult;

  if (data.list.length === 0) {
    return (
      <div className="text-gray-500 text-center py-4">
        暂无可接工序
      </div>
    );
  }

  return (
    <div className="my-2 rounded-lg border overflow-hidden">
      <div className="bg-blue-50 px-3 py-2 text-sm font-medium text-blue-700">
        可接工序 ({data.list.length})
      </div>
      <List>
        {data.list.map((task) => (
          <List.Item
            key={task.processId}
            prefix={
              task.orderImage && (
                <Image
                  src={task.orderImage}
                  width={40}
                  height={40}
                  fit="cover"
                  className="rounded"
                />
              )
            }
            description={
              <div className="flex justify-between">
                <span>{task.orderName}</span>
                <span className="text-orange-500">
                  ¥{task.piecePrice}/件
                </span>
              </div>
            }
            extra={
              <span className="text-green-600 font-medium">
                剩余 {task.remainingQuantity}
              </span>
            }
          >
            {task.processName}
          </List.Item>
        ))}
      </List>
    </div>
  );
};
```

## 在 Thread 中配置 Tool UI

修改 `src/components/thread.tsx`，注册所有自定义 Tool UI：

```tsx
import { ToolFallback } from "@/components/tool-fallback";
import { OverviewToolUI } from "@/components/tool-ui/overview-tool";
import { OrderListToolUI } from "@/components/tool-ui/order-list-tool";
import { EarningsToolUI } from "@/components/tool-ui/earnings-tool";
import { AvailableTasksToolUI } from "@/components/tool-ui/tasks-tool";
import { UnpaidSummaryToolUI } from "@/components/tool-ui/unpaid-summary-tool";

const AssistantMessage: FC = () => {
  return (
    <MessagePrimitive.Root className="...">
      <div className="...">
        <MessagePrimitive.Parts
          components={{
            Text: MarkdownText,
            tools: {
              by_name: {
                // 老板端工具
                get_overview: OverviewToolUI,
                query_orders: OrderListToolUI,
                get_unpaid_summary: UnpaidSummaryToolUI,
                // 员工端工具
                get_my_earnings: EarningsToolUI,
                get_available_tasks: AvailableTasksToolUI,
              },
              Fallback: ToolFallback,
            },
          }}
        />
        <MessageError />
      </div>
    </MessagePrimitive.Root>
  );
};
```

## 处理加载状态的完整模式

Tool UI 组件应该处理三种状态：

```tsx
export const MyToolUI: ToolCallContentPartComponent = ({ result, status }) => {
  // 1. 运行中 - 显示骨架屏或加载动画
  if (status?.type === "running") {
    return (
      <div className="flex items-center gap-2 text-gray-500">
        <LoadingSpinner />
        <span>处理中...</span>
      </div>
    );
  }

  // 2. 失败或取消 - 显示错误信息
  if (status?.type === "incomplete") {
    const isCancelled = status.reason === "cancelled";
    return (
      <div className={isCancelled ? "text-gray-500" : "text-red-500"}>
        {isCancelled ? "已取消" : "执行失败"}
        {status.error && (
          <p className="text-sm mt-1">{String(status.error)}</p>
        )}
      </div>
    );
  }

  // 3. 完成但无结果
  if (!result) return null;

  // 4. 完成 - 渲染结果
  return <div>{/* 渲染结果 */}</div>;
};
```

## 最佳实践

### 设计原则

1. **保持简洁**: Tool UI 应该直观地展示工具结果，避免过于复杂
2. **对话优先**: 组件针对聊天宽度优化，移动端优先，可快速浏览
3. **Schema 驱动**: 工具输出结构化 JSON，组件一致渲染
4. **可引用**: 每个组件有稳定 ID，助手可以引用（"上面的第二行"）

### 实现建议

1. **处理空状态**: 当结果为空时显示友好的提示
2. **限制显示数量**: 列表类数据限制显示前几条，避免占用过多空间
3. **加载状态**: 始终处理 `status.type === "running"` 的情况
4. **错误处理**: 优雅地处理工具执行失败的情况
5. **响应式设计**: 考虑移动端的显示效果
6. **副作用回执**: 重要操作后显示持久化的回执（状态、摘要、时间戳）

### Response Actions 建议

- 默认 2-3 个 CTA，直接关联上方内容
- 对破坏性或高影响操作使用 `confirmLabel` 二次确认
- `confirmTimeout` 保持较短（默认 3000ms）
- 操作按钮响应式布局，窄屏时堆叠显示

## 相关文件

- [src/components/thread.tsx](../src/components/thread.tsx) - Thread 组件，配置 Tool UI
- [src/components/tool-fallback.tsx](../src/components/tool-fallback.tsx) - 默认的 Tool UI 组件
- [crates/server/src/mcp/boss.rs](../crates/server/src/mcp/boss.rs) - 老板端 MCP 工具实现
- [crates/server/src/mcp/staff.rs](../crates/server/src/mcp/staff.rs) - 员工端 MCP 工具实现

## 参考资料

- [Tool UI 官方文档](https://www.tool-ui.com/docs/overview)
- [assistant-ui 文档](https://www.assistant-ui.com/docs)
