import { makeAssistantToolUI } from "@assistant-ui/react";
import { StatsDisplay } from "@/components/tool-ui/stats-display";
import { DailyTrendChart } from "../daily-trend-chart";
import { SectionLabel, ToolLoading } from "../shared";
import { getMyEarningsStats } from "./constants";
import type { Output, MyEarningsSummary } from "../types";

// 收入统计工具 UI
export const StatsToolUi = makeAssistantToolUI<unknown, Output>({
  toolName: "get_my_earnings",
  render: ({ status, result }) => {
    // 加载状态
    if (status?.type === "running") {
      return <ToolLoading toolName="get_my_earnings" />;
    }

    // 无结果则不渲染
    if (!result || result.length === 0) return null;

    // 从 Output 中提取并解析 JSON 文本
    const data: MyEarningsSummary = JSON.parse(result[0].text);

    // 将后端数据转换为 StatItem 格式
    const stats = getMyEarningsStats(data);

    return (
      <StatsDisplay
        id="my-earnings"
        title="我的收入统计"
        stats={stats}
        footer={
          data.dailyStats.length > 0 && (
            <div className="p-2">
              <SectionLabel>收入趋势图</SectionLabel>
              <DailyTrendChart dailyStats={data.dailyStats} />
            </div>
          )
        }
      />
    );
  },
});
