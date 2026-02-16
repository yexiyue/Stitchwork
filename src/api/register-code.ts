import { client } from "./client";
import type { RegisterCode, ListData, QueryParams } from "@/types";

export const registerCodeApi = {
  create: () => client.post<RegisterCode>("/api/register-codes"),
  list: (params?: QueryParams) =>
    client.get<ListData<RegisterCode>>("/api/register-codes", { params }),
  delete: (id: string) => client.delete<void>(`/api/register-codes/${id}`),
};
