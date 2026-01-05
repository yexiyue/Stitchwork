// 卡片样式常量
export const CARD_STYLES = {
  // 列表项卡片
  listItem: "bg-white p-3 mb-2 mx-2 rounded-lg shadow-sm flex gap-3",
  // 基础卡片
  base: "bg-white rounded-lg shadow-sm",
  // 统计卡片容器
  statsSection: "bg-white mt-2 p-4",
  // 页面容器
  pageContainer: "flex flex-col h-full overflow-hidden",
  // 统计页面容器
  statsPageContainer: "flex flex-col h-full bg-gray-50",
} as const;

// 统计卡片颜色
export const STATS_COLORS = {
  blue: {
    bg: "bg-blue-50",
    text: "text-blue-600",
  },
  green: {
    bg: "bg-green-50",
    text: "text-green-600",
  },
  purple: {
    bg: "bg-purple-50",
    text: "text-purple-600",
  },
  orange: {
    bg: "bg-orange-50",
    text: "text-orange-600",
  },
} as const;

export type StatsColor = keyof typeof STATS_COLORS;
