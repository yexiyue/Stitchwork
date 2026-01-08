import { client } from "./client";
import type {
  ListData,
  QueryParams,
  Payroll,
  CreatePayrollDto,
  UpdatePayrollDto,
  PayrollDetail,
} from "@/types";

export const payrollApi = {
  list: (params?: QueryParams) =>
    client.get<ListData<Payroll>>("/api/payrolls", params),
  getOne: (id: string) => client.get<PayrollDetail>(`/api/payrolls/${id}`),
  create: (data: CreatePayrollDto) =>
    client.post<Payroll>("/api/payrolls", data),
  update: (id: string, data: UpdatePayrollDto) =>
    client.put<Payroll>(`/api/payrolls/${id}`, data),
  delete: (id: string) => client.delete<void>(`/api/payrolls/${id}`),
};
