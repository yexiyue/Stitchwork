import { makeAssistantToolUI } from "@assistant-ui/react";
import { DataTable } from "@/components/tool-ui/data-table";
import { ToolLoading } from "../shared";
import { getOrdersColumns } from "./columns";
import type { Output, OrderListResponse } from "../types";

// 订单列表查询 UI
export const OrdersToolUi = makeAssistantToolUI<unknown, Output>({
  toolName: "query_orders",
  render: ({ status, result }) => {
    // 加载状态
    if (status?.type === "running") {
      return <ToolLoading toolName="query_orders" />;
    }

    // 无结果则不渲染
    if (!result || result.length === 0) return null;

    // 解析 JSON 数据
    const data: OrderListResponse = JSON.parse(result[0].text);

    // 转换为 DataTable 兼容格式
    const tableData = data.list.map((item) => ({
      id: item.id,
      productName: item.productName,
      quantity: item.quantity,
      unitPrice: item.unitPrice,
      status: item.status,
      receivedAt: item.receivedAt,
    }));

    return (
      <DataTable
        id="orders"
        columns={getOrdersColumns()}
        data={tableData}
        rowIdKey="id"
        emptyMessage="暂无订单"
        layout="auto"
        maxHeight="400px"
      />
    );
  },
});
