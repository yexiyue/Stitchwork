export interface ProcessProgress {
  processId: string;
  name: string;
  completedQuantity: number;
}

export interface OrderStats {
  orderId: string;
  totalQuantity: number;
  completedQuantity: number;
  progress: number;
  processes: ProcessProgress[];
}

export interface CustomerSummary {
  customerId: string;
  customerName: string;
  totalOrders: number;
  pendingOrders: number;
  processingOrders: number;
  completedOrders: number;
}

export interface CustomerSummaryList {
  list: CustomerSummary[];
}

export interface WorkerProduction {
  userId: string;
  userName: string;
  totalQuantity: number;
  totalAmount: string;
}

export interface WorkerProductionList {
  list: WorkerProduction[];
}

export interface WorkerStatsParams {
  startDate?: string;
  endDate?: string;
}

// Daily stats for trend chart
export interface DailyStat {
  date: string;
  totalQuantity: number;
  totalAmount: string;
}

export interface DailyStatsList {
  list: DailyStat[];
}

// Group stats for stacked bar chart
export interface GroupStat {
  id: string;
  name: string;
  totalQuantity: number;
  totalAmount: string;
}

export interface GroupStatsList {
  list: GroupStat[];
}

// ============ Order Stats ============

export interface OrderStatsParams {
  startDate?: string;
  endDate?: string;
}

export interface OrderOverview {
  totalOrders: number;
  pendingOrders: number;
  processingOrders: number;
  completedOrders: number;
  deliveredOrders: number;
  cancelledOrders: number;
  totalQuantity: number;
  totalAmount: string;
}

export interface MonthlyOrderStat {
  month: string;
  orderCount: number;
  totalQuantity: number;
  totalAmount: string;
}

export interface MonthlyOrderStatsList {
  list: MonthlyOrderStat[];
}

export interface CustomerContribution {
  customerId: string;
  customerName: string;
  orderCount: number;
  totalQuantity: number;
  totalAmount: string;
}

export interface CustomerContributionList {
  list: CustomerContribution[];
}

export interface OrderProgressItem {
  orderId: string;
  productName: string;
  customerName: string;
  totalQuantity: number;
  completedQuantity: number;
  progress: number;
  status: string;
}

export interface OrderProgressList {
  list: OrderProgressItem[];
}

export interface DailyOrderStat {
  date: string;
  orderCount: number;
  totalQuantity: number;
  totalAmount: string;
}

export interface DailyOrderStatsList {
  list: DailyOrderStat[];
}
