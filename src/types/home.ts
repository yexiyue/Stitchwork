export interface BossOverview {
  pendingCount: number;
  processingOrderCount: number;
  todayQuantity: number;
  todayAmount: string;
  monthQuantity: number;
  monthAmount: string;
  staffCount: number;
}

export interface StaffOverview {
  monthQuantity: number;
  monthAmount: string;
}

export type HomeOverview = BossOverview | StaffOverview;

export function isBossOverview(
  overview: HomeOverview
): overview is BossOverview {
  return "pendingCount" in overview;
}

export type ActivityType = "submit" | "approve" | "reject";

export interface Activity {
  id: string;
  activityType: ActivityType;
  userName: string;
  orderName: string;
  orderImage?: string;
  processName: string;
  quantity: number;
  createdAt: string;
}

export interface ActivityList {
  list: Activity[];
}
