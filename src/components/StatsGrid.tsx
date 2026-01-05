import type { ReactNode } from "react";

interface StatsGridProps {
  children: ReactNode;
  columns?: 2 | 3 | 4;
}

const GRID_COLS = {
  2: "grid-cols-2",
  3: "grid-cols-3",
  4: "grid-cols-4",
} as const;

/**
 * 统计卡片网格容器
 */
export function StatsGrid({ children, columns = 3 }: StatsGridProps) {
  return (
    <div className={`grid ${GRID_COLS[columns]} gap-3`}>
      {children}
    </div>
  );
}
