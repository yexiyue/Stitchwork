import type { OrderStatus, PieceRecordStatus } from "@/types";

// 订单状态映射
export const ORDER_STATUS_MAP: Record<OrderStatus, { label: string; color: string }> = {
  pending: { label: "待处理", color: "#faad14" },
  processing: { label: "进行中", color: "#1890ff" },
  completed: { label: "已完成", color: "#52c41a" },
  delivered: { label: "已交付", color: "#722ed1" },
  cancelled: { label: "已取消", color: "#999" },
};

// 订单状态选项（用于筛选）
export const ORDER_STATUS_OPTIONS = [
  { key: "", title: "全部状态" },
  { key: "pending", title: "待处理" },
  { key: "processing", title: "进行中" },
  { key: "completed", title: "已完成" },
  { key: "delivered", title: "已交付" },
  { key: "cancelled", title: "已取消" },
] as const;

// 计件记录状态映射
export const RECORD_STATUS_MAP: Record<PieceRecordStatus, { label: string; color: string }> = {
  pending: { label: "待审核", color: "#faad14" },
  approved: { label: "已通过", color: "#52c41a" },
  rejected: { label: "已拒绝", color: "#ff4d4f" },
  settled: { label: "已结算", color: "#722ed1" },
};

// 计件记录状态选项（用于筛选）
export const RECORD_STATUS_OPTIONS = [
  { key: "", title: "全部状态" },
  { key: "pending", title: "待审核" },
  { key: "approved", title: "已通过" },
  { key: "rejected", title: "已拒绝" },
  { key: "settled", title: "已结算" },
] as const;

// 计件记录状态选项（不含全部，用于员工筛选）
export const RECORD_STATUS_OPTIONS_NO_ALL = [
  { key: "pending", title: "待审核" },
  { key: "approved", title: "已通过" },
  { key: "rejected", title: "已拒绝" },
  { key: "settled", title: "已结算" },
] as const;
