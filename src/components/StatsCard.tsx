import { STATS_COLORS, type StatsColor } from "@/constants";

interface StatsCardProps {
  /** 数值 */
  value: number | string;
  /** 标签 */
  label: string;
  /** 颜色主题 */
  color?: StatsColor;
  /** 前缀（如 ¥） */
  prefix?: string;
  /** 是否使用较小的字体 */
  small?: boolean;
}

/**
 * 统计数字卡片
 */
export function StatsCard({
  value,
  label,
  color = "blue",
  prefix = "",
  small = false,
}: StatsCardProps) {
  const colors = STATS_COLORS[color];
  const valueSize = small ? "text-lg" : "text-2xl";

  return (
    <div className={`${colors.bg} rounded-lg p-3 text-center`}>
      <div className={`${valueSize} font-bold ${colors.text}`}>
        {prefix}{value}
      </div>
      <div className="text-xs text-gray-500">{label}</div>
    </div>
  );
}
