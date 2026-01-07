import { client } from "./client";
import type { RegisterCode, UserListItem } from "@/types";

export const adminApi = {
  // Register codes
  createRegisterCode: () =>
    client.post<RegisterCode>("/api/admin/register-codes"),
  listRegisterCodes: () =>
    client.get<RegisterCode[]>("/api/admin/register-codes"),
  disableRegisterCode: (id: string) =>
    client.delete<void>(`/api/admin/register-codes/${id}`),

  // Users
  listUsers: () => client.get<UserListItem[]>("/api/admin/users"),
};
