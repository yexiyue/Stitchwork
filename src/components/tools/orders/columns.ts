import type { Column } from "@/components/tool-ui/data-table";

// 订单表格列定义
export function getOrdersColumns(): Column[] {
  return [
    {
      key: "status",
      label: "状态",
      align: "center",
      priority: "primary",
      format: {
        kind: "status",
        statusMap: {
          pending: { tone: "warning", label: "待处理" },
          processing: { tone: "info", label: "进行中" },
          completed: { tone: "success", label: "已完成" },
          delivered: { tone: "success", label: "已出货" },
          cancelled: { tone: "danger", label: "已取消" },
        },
      },
    },
    {
      key: "productName",
      label: "产品名称",
      priority: "primary",
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
      key: "unitPrice",
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
      key: "receivedAt",
      label: "接单时间",
      align: "center",
      priority: "secondary",
      format: { kind: "date", dateFormat: "short" },
    },
  ];
}
