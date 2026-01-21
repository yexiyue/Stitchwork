import type { Column } from "@/components/tool-ui/data-table";

// 待发工资列表表格列定义
export function getUnpaidSummaryColumns(): Column[] {
  return [
    {
      key: "userName",
      label: "员工",
      priority: "primary",
    },
    {
      key: "totalQuantity",
      label: "待发数量",
      align: "right",
      priority: "primary",
      format: { kind: "number" },
    },
    {
      key: "totalAmount",
      label: "待发金额",
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
