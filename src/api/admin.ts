import { client } from "./client";
import type {
  UserListItem,
  AdminStats,
  ListData,
  QueryParams,
} from "@/types";

export const adminApi = {
  // Stats
  getStats: () => client.get<AdminStats>("/api/admin/stats"),

  // Users
  listUsers: (params?: QueryParams) =>
    client.get<ListData<UserListItem>>("/api/admin/users", { params }),
};
