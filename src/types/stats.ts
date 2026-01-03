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
