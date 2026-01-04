import { client } from "./client";
import type {
  OrderStats,
  CustomerSummaryList,
  WorkerProductionList,
  WorkerStatsParams,
  DailyStatsList,
  GroupStatsList,
} from "@/types";

export const statsApi = {
  orderStats: (orderId: string) =>
    client.get<OrderStats>(`/api/orders/${orderId}/stats`),
  customerSummary: () =>
    client.get<CustomerSummaryList>("/api/stats/customers"),
  workerProduction: (params?: WorkerStatsParams) =>
    client.get<WorkerProductionList>("/api/stats/workers", params),
  dailyStats: (params?: WorkerStatsParams) =>
    client.get<DailyStatsList>("/api/stats/daily", params),
  statsByOrder: (params?: WorkerStatsParams) =>
    client.get<GroupStatsList>("/api/stats/by-order", params),
  statsByProcess: (params?: WorkerStatsParams) =>
    client.get<GroupStatsList>("/api/stats/by-process", params),
};
