import type { Column } from "@/components/tool-ui/data-table";

// 工资单表格列定义
export function getPayrollColumns(): Column[] {
  return [
    {
      key: "paidAt",
      label: "发放时间",
      align: "center",
      priority: "primary",
      format: { kind: "date", dateFormat: "short" },
    },
    {
      key: "amount",
      label: "金额",
      align: "right",
      priority: "primary",
      format: {
        kind: "currency",
        currency: "CNY",
        decimals: 2,
      },
    },
    {
      key: "note",
      label: "备注",
      truncate: true,
    },
    {
      key: "paymentImage",
      label: "支付凭证",
      align: "center",
      priority: "secondary",
      format: {
        kind: "link",
        hrefKey: "paymentImage",
      },
    },
  ];
}
