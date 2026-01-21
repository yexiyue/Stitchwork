import type { StatItem } from "@/components/tool-ui/stats-display";
import type { BossOverview } from "../types";

// 将后端数据转换为 StatItem 格式
export function getOverviewStats(data: BossOverview): StatItem[] {
  return [
    {
      key: "pending-count",
      label: "待审批",
      value: data.pendingCount,
      format: { kind: "number" },
    },
    {
      key: "processing-order-count",
      label: "进行中订单",
      value: data.processingOrderCount,
      format: { kind: "number" },
    },
    {
      key: "staff-count",
      label: "员工总数",
      value: data.staffCount,
      format: { kind: "number" },
    },
    {
      key: "today-quantity",
      label: "今日产量",
      value: data.todayQuantity,
      format: { kind: "number", compact: true },
    },
    {
      key: "today-amount",
      label: "今日金额",
      value: parseFloat(data.todayAmount),
      format: { kind: "currency", currency: "CNY" },
    },
    {
      key: "month-quantity",
      label: "本月产量",
      value: data.monthQuantity,
      format: { kind: "number", compact: true },
    },
    {
      key: "month-amount",
      label: "本月金额",
      value: parseFloat(data.monthAmount),
      format: { kind: "currency", currency: "CNY" },
    },
  ];
}
