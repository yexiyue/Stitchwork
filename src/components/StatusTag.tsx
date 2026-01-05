import { Tag } from "antd-mobile";
import { ORDER_STATUS_MAP, RECORD_STATUS_MAP } from "@/constants";
import type { OrderStatus, PieceRecordStatus } from "@/types";

interface OrderStatusTagProps {
  status: OrderStatus;
  type: "order";
}

interface RecordStatusTagProps {
  status: PieceRecordStatus;
  type: "record";
}

type StatusTagProps = OrderStatusTagProps | RecordStatusTagProps;

/**
 * 状态标签组件
 */
export function StatusTag(props: StatusTagProps) {
  const { status, type } = props;

  const statusMap = type === "order" ? ORDER_STATUS_MAP : RECORD_STATUS_MAP;
  const statusInfo = statusMap[status as keyof typeof statusMap];

  if (!statusInfo) {
    return null;
  }

  return (
    <Tag
      color={statusInfo.color}
      fill="outline"
      style={{ "--border-radius": "4px" }}
    >
      {statusInfo.label}
    </Tag>
  );
}
