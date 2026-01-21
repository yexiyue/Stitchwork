import type { Column } from "@/components/tool-ui/data-table";

// 员工产量排行榜表格列定义
export function getWorkerStatsColumns(): Column[] {
  return [
    {
      key: "userName",
      label: "员工",
      priority: "primary",
    },
    {
      key: "totalQuantity",
      label: "总产量",
      align: "right",
      priority: "primary",
      format: { kind: "number" },
    },
    {
      key: "totalAmount",
      label: "总金额",
      align: "right",
      priority: "primary",
      format: {
        kind: "currency",
        currency: "CNY",
        decimals: 2,
      },
    },
  ];
}
