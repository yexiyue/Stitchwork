import { useQuery, useMutation, useQueryClient } from "@tanstack/react-query";
import { orderApi } from "@/api";
import type {
  CreateOrderDto,
  UpdateOrderDto,
  QueryParams,
  OrderQueryParams,
} from "@/types";

const KEY = "orders";

export const useOrders = (params?: QueryParams & OrderQueryParams) =>
  useQuery({
    queryKey: params ? [KEY, params] : [KEY],
    queryFn: () => orderApi.list(params),
  });

export const useOrder = (id: string) =>
  useQuery({
    queryKey: [KEY, id],
    queryFn: () => orderApi.getOne(id),
    enabled: !!id,
  });

export const useCreateOrder = () => {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: (data: CreateOrderDto) => orderApi.create(data),
    onSuccess: () => qc.invalidateQueries({ queryKey: [KEY] }),
  });
};

export const useUpdateOrder = () => {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: ({ id, data }: { id: string; data: UpdateOrderDto }) =>
      orderApi.update(id, data),
    onSuccess: () => qc.invalidateQueries({ queryKey: [KEY] }),
  });
};

export const useDeleteOrder = () => {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: (id: string) => orderApi.delete(id),
    onSuccess: () => qc.invalidateQueries({ queryKey: [KEY] }),
  });
};

// 特殊方法
export const useUpdateOrderStatus = () => {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: ({ id, status }: { id: string; status: string }) =>
      orderApi.updateStatus(id, { status }),
    onSuccess: () => qc.invalidateQueries({ queryKey: [KEY] }),
  });
};
