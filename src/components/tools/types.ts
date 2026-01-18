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

export type {
  Output,
  MyEarningsSummary,
  MyRecordsResponse,
  MyPayroll,
  MyPayrollListResponse,
  AvailableTask,
  AvailableTasksResponse,
};
