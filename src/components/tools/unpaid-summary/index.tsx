import { makeAssistantToolUI } from "@assistant-ui/react";
import { DataTable } from "@/components/tool-ui/data-table";
import { StatsDisplay, type StatItem } from "@/components/tool-ui/stats-display";
import { SectionLabel, ToolLoading } from "../shared";
import { getUnpaidSummaryColumns } from "./columns";
import { Wallet } from "lucide-react";
import type { Output, UnpaidSummaryResponse } from "../types";

// 计算汇总统计
function getSummaryStats(data: UnpaidSummaryResponse): StatItem[] {
  const totalQuantity = data.list.reduce(
    (sum, item) => sum + item.totalQuantity,
    0
  );
  const totalAmount = data.list.reduce(
    (sum, item) => sum + parseFloat(item.totalAmount),
    0
  );

  return [
    {
      key: "worker-count",
      label: "员工人数",
      value: data.list.length,
      format: { kind: "number" },
    },
    {
      key: "total-quantity",
      label: "待发总数量",
      value: totalQuantity,
      format: { kind: "number", compact: true },
    },
    {
      key: "total-amount",
      label: "待发总金额",
      value: totalAmount,
      format: { kind: "currency", currency: "CNY" },
    },
  ];
}

// 空状态组件
function EmptyState() {
  return (
    <div className="flex flex-col items-center justify-center py-8 text-center">
      <div className="w-12 h-12 mb-3 rounded-full bg-muted/50 flex items-center justify-center">
        <Wallet className="w-6 h-6 text-muted-foreground/60" />
      </div>
      <p className="text-sm text-muted-foreground">暂无待发工资记录</p>
    </div>
  );
}

// 待发工资汇总 UI
export const UnpaidSummaryToolUi = makeAssistantToolUI<unknown, Output>({
  toolName: "get_unpaid_summary",
  render: ({ status, result }) => {
    // 加载状态
    if (status?.type === "running") {
      return <ToolLoading toolName="get_unpaid_summary" />;
    }

    // 无结果则不渲染
    if (!result || result.length === 0) return null;

    // 解析 JSON 数据
    const data: UnpaidSummaryResponse = JSON.parse(result[0].text);

    if (data.list.length === 0) {
      return <EmptyState />;
    }

    // 转换为 DataTable 兼容格式
    const tableData = data.list.map((item) => ({
      userId: item.userId,
      userName: item.userName,
      totalQuantity: item.totalQuantity,
      totalAmount: item.totalAmount,
    }));

    return (
      <StatsDisplay
        id="unpaid-summary"
        title="待发工资汇总"
        stats={getSummaryStats(data)}
        footer={
          tableData.length > 0 && (
            <div className="p-2">
              <SectionLabel>员工明细</SectionLabel>
              <DataTable
                id="unpaid-summary-list"
                columns={getUnpaidSummaryColumns()}
                data={tableData}
                rowIdKey="userId"
                emptyMessage="暂无数据"
                layout="auto"
                maxHeight="300px"
              />
            </div>
          )
        }
      />
    );
  },
});
