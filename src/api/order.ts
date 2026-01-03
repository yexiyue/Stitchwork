import { client } from "./client";
import type {
  ListData,
  QueryParams,
  Order,
  CreateOrderDto,
  UpdateOrderDto,
  UpdateOrderStatusDto,
  OrderQueryParams,
} from "@/types";

export const orderApi = {
  list: (params?: QueryParams & OrderQueryParams) =>
    client.get<ListData<Order>>("/api/orders", params),
  getOne: (id: string) =>
    client.get<Order>(`/api/orders/${id}`),
  create: (data: CreateOrderDto) =>
    client.post<Order>("/api/orders", data),
  update: (id: string, data: UpdateOrderDto) =>
    client.put<Order>(`/api/orders/${id}`, data),
  delete: (id: string) =>
    client.delete<void>(`/api/orders/${id}`),
  updateStatus: (id: string, data: UpdateOrderStatusDto) =>
    client.patch<Order>(`/api/orders/${id}/status`, data),
};
