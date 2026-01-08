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

export interface AdminStats {
  // 用户统计
  totalUsers: number;
  bossCount: number;
  staffCount: number;
  todayNewUsers: number;
  weekNewUsers: number;
  monthNewUsers: number;

  // 工坊统计
  totalWorkshops: number;
  activeWorkshops: number;

  // 注册码统计
  totalCodes: number;
  usedCodes: number;
  availableCodes: number;
  disabledCodes: number;

  // 平台活跃度
  todayOrders: number;
  monthOrders: number;
  todayRecords: number;
  monthRecords: number;
}
