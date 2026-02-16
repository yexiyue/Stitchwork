// 工具返回的文本内容类型
type Output = { type: "text"; text: string }[];

// 解析后的数据类型
interface MyEarningsSummary {
  totalQuantity: number;
  totalAmount: string; // Decimal 转为字符串
  pendingQuantity: number;
  pendingAmount: string;
  dailyStats: Array<{
    date: string;
    totalQuantity: number;
    totalAmount: string;
  }>;
}

// 计件记录响应类型
interface MyRecordsResponse {
  list: Array<{
    id: string;
    processName?: string;
    orderName?: string;
    quantity: number;
    amount: string;
    status: "pending" | "approved" | "rejected" | "settled";
    recordedBy: "bySelf" | "byBoss";
    recordedAt: string;
    piecePrice?: string;
  }>;
  total: number;
}

// 工资单记录
interface MyPayroll {
  id: string;
  userId: string;
  bossId: string;
  amount: string;
  paymentImage?: string;
  note?: string;
  paidAt: string;
}

// 工资单响应类型
interface MyPayrollListResponse {
  list: MyPayroll[];
  total: number;
}

// 可接工序
interface AvailableTask {
  processId: string;
  processName: string;
  piecePrice: string;
  orderId: string;
  orderName: string;
  orderImage?: string;
  remainingQuantity: number;
}

// 可接工序响应类型
interface AvailableTasksResponse {
  list: AvailableTask[];
}

// ============ 老板端类型定义 ============

// 订单状态
type OrderStatus =
  | "pending"
  | "processing"
  | "completed"
  | "delivered"
  | "cancelled";

// 订单
interface Order {
  id: string;
  customerId: string;
  bossId: string;
  productName: string;
  description?: string;
  images?: string[];
  quantity: number;
  unitPrice: string;
  status: OrderStatus;
  receivedAt: string;
  deliveredAt?: string;
}

// 订单列表响应
interface OrderListResponse {
  list: Order[];
  total: number;
}

// 老板端计件记录响应
interface BossPieceRecordListResponse {
  list: Array<{
    id: string;
    processId: string;
    userId: string;
    bossId: string;
    quantity: number;
    amount: string;
    status: "pending" | "approved" | "rejected" | "settled";
    recordedBy: "bySelf" | "byBoss";
    recordedAt: string;
    processName?: string;
    userName?: string;
    orderId?: string;
    orderName?: string;
    orderImage?: string;
    piecePrice?: string;
  }>;
  total: number;
}

// 员工产量统计项
interface WorkerProduction {
  userId: string;
  userName: string;
  totalQuantity: number;
  totalAmount: string;
}

// 员工产量统计列表
interface WorkerProductionList {
  list: WorkerProduction[];
}

// 老板首页概览数据
interface BossOverview {
  pendingCount: number;
  processingOrderCount: number;
  todayQuantity: number;
  todayAmount: string;
  monthQuantity: number;
  monthAmount: string;
  staffCount: number;
}

// 订单进度项
interface OrderProgressItem {
  orderId: string;
  productName: string;
  customerName: string;
  totalQuantity: number;
  completedQuantity: number;
  progress: number;
  status: string;
}

// 订单进度列表
interface OrderProgressList {
  list: OrderProgressItem[];
}

// 待发工资汇总项
interface UnpaidSummaryItem {
  userId: string;
  userName: string;
  totalQuantity: number;
  totalAmount: string;
}

// 待发工资汇总响应
interface UnpaidSummaryResponse {
  list: UnpaidSummaryItem[];
}

export type {
  Output,
  MyEarningsSummary,
  MyRecordsResponse,
  MyPayroll,
  MyPayrollListResponse,
  AvailableTask,
  AvailableTasksResponse,
  // 老板端类型
  OrderStatus,
  Order,
  OrderListResponse,
  BossPieceRecordListResponse,
  WorkerProduction,
  WorkerProductionList,
  BossOverview,
  OrderProgressItem,
  OrderProgressList,
  UnpaidSummaryItem,
  UnpaidSummaryResponse,
};
