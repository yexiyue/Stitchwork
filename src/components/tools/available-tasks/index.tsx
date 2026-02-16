import { makeAssistantToolUI } from "@assistant-ui/react";
import { DataTable, type Column } from "@/components/tool-ui/data-table";
import { ToolLoading } from "../shared";
import type { Output, AvailableTasksResponse } from "../types";

// 可接工序表格列定义
function getTaskColumns(): Column[] {
  return [
    {
      key: "processName",
      label: "工序",
      priority: "primary",
    },
    {
      key: "orderName",
      label: "订单",
      priority: "primary",
      truncate: true,
    },
    {
      key: "piecePrice",
      label: "单价",
      align: "right",
      priority: "primary",
      format: {
        kind: "currency",
        currency: "CNY",
        decimals: 2,
      },
    },
    {
      key: "remainingQuantity",
      label: "可接数量",
      align: "right",
      priority: "primary",
      format: { kind: "number" },
    },
  ];
}

// 可接工序 UI
export const AvailableTasksToolUi = makeAssistantToolUI<unknown, Output>({
  toolName: "get_available_tasks",
  render: ({ status, result }) => {
    // 加载状态
    if (status?.type === "running") {
      return <ToolLoading toolName="get_available_tasks" />;
    }

    // 无结果则不渲染
    if (!result || result.length === 0) return null;

    // 解析 JSON 数据
    const data: AvailableTasksResponse = JSON.parse(result[0].text);

    // 将 AvailableTask[] 转换为 Record<string, string>[] 格式
    const tableData = data.list.map((item) => ({
      processId: item.processId,
      processName: item.processName,
      piecePrice: item.piecePrice,
      orderId: item.orderId,
      orderName: item.orderName,
      orderImage: item.orderImage || "",
      remainingQuantity: String(item.remainingQuantity),
    }));

    return (
      <DataTable
        id="available-tasks"
        columns={getTaskColumns()}
        data={tableData}
        rowIdKey="processId"
        emptyMessage="暂无可接工序"
        layout="auto"
        maxHeight="400px"
      />
    );
  },
});
