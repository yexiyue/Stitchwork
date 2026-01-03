import { useQuery } from "@tanstack/react-query";
import { statsApi } from "@/api";
import type { WorkerStatsParams } from "@/types";

export const useOrderStats = (orderId: string) =>
  useQuery({
    queryKey: ["order-stats", orderId],
    queryFn: () => statsApi.orderStats(orderId),
    enabled: !!orderId,
  });

export const useCustomerSummary = () =>
  useQuery({
    queryKey: ["customer-summary"],
    queryFn: () => statsApi.customerSummary(),
  });

export const useWorkerProduction = (params?: WorkerStatsParams) =>
  useQuery({
    queryKey: params ? ["worker-production", params] : ["worker-production"],
    queryFn: () => statsApi.workerProduction(params),
  });
