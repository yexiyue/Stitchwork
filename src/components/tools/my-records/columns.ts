import type { Column } from "@/components/tool-ui/data-table";

// 计件记录表格列定义
export function getRecordsColumns(): Column[] {
  return [
    {
      key: "status",
      label: "状态",
      align: "center",
      priority: "primary",
      format: {
        kind: "status",
        statusMap: {
          pending: { tone: "warning", label: "待审批" },
          approved: { tone: "success", label: "已通过" },
          rejected: { tone: "danger", label: "已拒绝" },
          settled: { tone: "info", label: "已结算" },
        },
      },
    },
    {
      key: "processName",
      label: "工序",
      priority: "primary",
    },
    {
      key: "orderName",
      label: "订单",
      hideOnMobile: true,
      truncate: true,
    },
    {
      key: "quantity",
      label: "数量",
      align: "right",
      priority: "primary",
      format: { kind: "number" },
    },
    {
      key: "piecePrice",
      label: "单价",
      align: "right",
      hideOnMobile: true,
      format: {
        kind: "currency",
        currency: "CNY",
        decimals: 2,
      },
    },
    {
      key: "amount",
      label: "金额",
      align: "right",
      priority: "secondary",
      format: {
        kind: "currency",
        currency: "CNY",
        decimals: 2,
      },
    },
    {
      key: "recordedAt",
      label: "录入时间",
      align: "center",
      priority: "secondary",
      format: { kind: "date", dateFormat: "short" },
    },
  ];
}
