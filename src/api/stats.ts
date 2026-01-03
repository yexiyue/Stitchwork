import { client } from "./client";
import type {
  OrderStats,
  CustomerSummaryList,
  WorkerProductionList,
  WorkerStatsParams,
} from "@/types";

export const statsApi = {
  orderStats: (orderId: string) =>
    client.get<OrderStats>(`/api/orders/${orderId}/stats`),
  customerSummary: () =>
    client.get<CustomerSummaryList>("/api/stats/customers"),
  workerProduction: (params?: WorkerStatsParams) =>
    client.get<WorkerProductionList>("/api/stats/workers", params),
};
