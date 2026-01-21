import { makeAssistantToolUI } from "@assistant-ui/react";
import { DataTable } from "@/components/tool-ui/data-table";
import { StatsDisplay, type StatItem } from "@/components/tool-ui/stats-display";
import { SectionLabel, ToolLoading } from "../shared";
import { getWorkerStatsColumns } from "./columns";
import type { Output, WorkerProductionList } from "../types";

// 计算汇总统计
function getSummaryStats(data: WorkerProductionList): StatItem[] {
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
      label: "总产量",
      value: totalQuantity,
      format: { kind: "number", compact: true },
    },
    {
      key: "total-amount",
      label: "总金额",
      value: totalAmount,
      format: { kind: "currency", currency: "CNY" },
    },
  ];
}

// 员工产量统计 UI
export const WorkerStatsToolUi = makeAssistantToolUI<unknown, Output>({
  toolName: "get_worker_stats",
  render: ({ status, result }) => {
    // 加载状态
    if (status?.type === "running") {
      return <ToolLoading toolName="get_worker_stats" />;
    }

    // 无结果则不渲染
    if (!result || result.length === 0) return null;

    // 解析 JSON 数据
    const data: WorkerProductionList = JSON.parse(result[0].text);

    // 转换为 DataTable 兼容格式
    const tableData = data.list.map((item) => ({
      userId: item.userId,
      userName: item.userName,
      totalQuantity: item.totalQuantity,
      totalAmount: item.totalAmount,
    }));

    return (
      <StatsDisplay
        id="worker-stats"
        title="员工产量统计"
        stats={getSummaryStats(data)}
        footer={
          tableData.length > 0 && (
            <div className="p-2">
              <SectionLabel>员工排行榜</SectionLabel>
              <DataTable
                id="worker-stats-list"
                columns={getWorkerStatsColumns()}
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
