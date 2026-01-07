import type { Role } from "./auth";

export interface RegisterCode {
  id: string;
  code: string;
  isActive: boolean;
  usedBy?: string;
  usedAt?: string;
  createdAt: string;
  usedByUsername?: string;
}

export interface UserListItem {
  id: string;
  username: string;
  role: Role;
  displayName?: string;
  phone: string;
  avatar?: string;
  isSuperAdmin: boolean;
  createdAt: string;
}
