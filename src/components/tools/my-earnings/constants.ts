import type { StatItem } from "@/components/tool-ui/stats-display";
import type { MyEarningsSummary } from "../types";

// 将后端数据转换为 StatItem 格式
export function getMyEarningsStats(data: MyEarningsSummary): StatItem[] {
  return [
    {
      key: "total-quantity",
      label: "总产量",
      value: data.totalQuantity,
      format: { kind: "number", compact: true },
    },
    {
      key: "total-amount",
      label: "已结算金额",
      value: parseFloat(data.totalAmount),
      format: { kind: "currency", currency: "CNY" },
    },
    {
      key: "pending-quantity",
      label: "待审批数量",
      value: data.pendingQuantity,
      format: { kind: "number", compact: true },
    },
    {
      key: "pending-amount",
      label: "待审批金额",
      value: parseFloat(data.pendingAmount),
      format: { kind: "currency", currency: "CNY" },
    },
  ];
}
