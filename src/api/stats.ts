import { client } from "./client";
import type {
  OrderStats,
  CustomerSummaryList,
  WorkerProductionList,
  WorkerStatsParams,
  DailyStatsList,
  GroupStatsList,
  OrderStatsParams,
  OrderOverview,
  MonthlyOrderStatsList,
  CustomerContributionList,
  OrderProgressList,
  DailyOrderStatsList,
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
  // Order stats
  orderOverview: (params?: OrderStatsParams) =>
    client.get<OrderOverview>("/api/stats/orders/overview", params),
  monthlyOrderStats: (params?: OrderStatsParams) =>
    client.get<MonthlyOrderStatsList>("/api/stats/orders/monthly", params),
  customerContribution: (params?: OrderStatsParams) =>
    client.get<CustomerContributionList>("/api/stats/orders/by-customer", params),
  orderProgress: (params?: OrderStatsParams) =>
    client.get<OrderProgressList>("/api/stats/orders/progress", params),
  dailyOrderStats: (params?: OrderStatsParams) =>
    client.get<DailyOrderStatsList>("/api/stats/orders/daily", params),
};
