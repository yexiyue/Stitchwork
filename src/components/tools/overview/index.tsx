import { makeAssistantToolUI } from "@assistant-ui/react";
import {
  StatsDisplay,
  StatsDisplayProgress,
} from "@/components/tool-ui/stats-display";
import { getOverviewStats } from "./constants";
import type { Output, BossOverview } from "../types";

// 首页概览数据 UI
export const OverviewToolUi = makeAssistantToolUI<unknown, Output>({
  toolName: "get_overview",
  render: ({ status, result }) => {
    // 加载状态
    if (status?.type === "running") {
      return <StatsDisplayProgress />;
    }

    // 无结果则不渲染
    if (!result || result.length === 0) return null;

    // 解析 JSON 数据
    const data: BossOverview = JSON.parse(result[0].text);

    return (
      <StatsDisplay
        id="boss-overview"
        title="工坊概览"
        stats={getOverviewStats(data)}
      />
    );
  },
});
