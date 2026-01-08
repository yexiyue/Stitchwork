import { useAuthStore } from "@/stores/auth";

/**
 * 工坊设置 Hook
 * 提供计件单位和场所名称的访问
 */
export function useWorkshopSettings() {
  const workshop = useAuthStore((s) => s.user?.workshop);

  return {
    /** 计件单位（默认：打） */
    pieceUnit: workshop?.pieceUnit ?? "打",
    /** 场所名称（默认：工坊） */
    businessLabel: workshop?.businessLabel ?? "工坊",
  };
}
