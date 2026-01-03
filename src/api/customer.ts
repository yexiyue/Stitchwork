import { client } from "./client";
import type { ListData, QueryParams, Customer, CreateCustomerDto, UpdateCustomerDto } from "@/types";

export const customerApi = {
  list: (params?: QueryParams) =>
    client.get<ListData<Customer>>("/api/customers", params),
  getOne: (id: string) =>
    client.get<Customer>(`/api/customers/${id}`),
  create: (data: CreateCustomerDto) =>
    client.post<Customer>("/api/customers", data),
  update: (id: string, data: UpdateCustomerDto) =>
    client.put<Customer>(`/api/customers/${id}`, data),
  delete: (id: string) =>
    client.delete<void>(`/api/customers/${id}`),
};
