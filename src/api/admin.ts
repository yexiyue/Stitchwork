import { client } from "./client";
import type {
  RegisterCode,
  UserListItem,
  AdminStats,
  ListData,
  QueryParams,
} from "@/types";

export const adminApi = {
  // Stats
  getStats: () => client.get<AdminStats>("/api/admin/stats"),

  // Register codes
  createRegisterCode: () =>
    client.post<RegisterCode>("/api/admin/register-codes"),
  listRegisterCodes: (params?: QueryParams) =>
    client.get<ListData<RegisterCode>>("/api/admin/register-codes", { params }),
  disableRegisterCode: (id: string) =>
    client.delete<void>(`/api/admin/register-codes/${id}`),

  // Users
  listUsers: (params?: QueryParams) =>
    client.get<ListData<UserListItem>>("/api/admin/users", { params }),
};
