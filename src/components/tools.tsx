import { makeAssistantToolUI } from "@assistant-ui/react";
import {
  StatsDisplay,
  StatsDisplayProgress,
  type StatItem,
} from "./tool-ui/stats-display";
import { Chart } from "./charts";
import type { EChartsOption } from "echarts";

// 工具返回的文本内容类型
type Output = { type: "text"; text: string }[];

// 解析后的数据类型
interface MyEarningsSummary {
  totalQuantity: number;
  totalAmount: string; // Decimal 转为字符串
  pendingQuantity: number;
  pendingAmount: string;
  dailyStats: Array<{
    date: string;
    totalQuantity: number;
    totalAmount: string;
  }>;
}

// 每日趋势图组件
function DailyTrendChart({
  dailyStats,
}: {
  dailyStats: MyEarningsSummary["dailyStats"];
}) {
  const chartOption: EChartsOption = {
    tooltip: {
      trigger: "axis",
      formatter: (params: unknown) => {
        const p = (params as { name: string; value: number }[])[0];
        return `${p.name}: ¥${p.value}`;
      },
    },
    grid: { left: 10, right: 10, bottom: 30, top: 10, containLabel: true },
    xAxis: {
      type: "category",
      data: dailyStats.map((d) => d.date.slice(5)), // 只显示 MM-DD
      axisLabel: { fontSize: 10 },
    },
    yAxis: { type: "value" },
    dataZoom: [{ type: "inside", start: 0, end: 100 }],
    series: [
      {
        type: "line",
        data: dailyStats.map((s) => parseFloat(s.totalAmount)),
        smooth: true,
        areaStyle: { opacity: 0.3 },
        lineStyle: { color: "#3b82f6", width: 2 },
        itemStyle: { color: "#3b82f6" },
      },
    ],
  };

  return <Chart option={chartOption} height={180} />;
}

export const StatsToolUi = makeAssistantToolUI<unknown, Output>({
  toolName: "get_my_earnings",
  render: ({ status, result }) => {
    // 加载状态
    if (status?.type === "running") {
      return <StatsDisplayProgress />;
    }

    // 无结果则不渲染
    if (!result || result.length === 0) return null;

    // 从 Output 中提取并解析 JSON 文本
    const data: MyEarningsSummary = JSON.parse(result[0].text);

    // 将后端数据转换为 StatItem 格式
    const stats: StatItem[] = [
      {
        key: "total-quantity",
        label: "总产量",
        value: data.totalQuantity,
        format: { kind: "number", compact: true },
      },
      {
        key: "total-amount",
        label: "已结算金额",
        value: parseFloat(data.totalAmount),
        format: { kind: "currency", currency: "CNY" },
      },
      {
        key: "pending-quantity",
        label: "待审批数量",
        value: data.pendingQuantity,
        format: { kind: "number", compact: true },
      },
      {
        key: "pending-amount",
        label: "待审批金额",
        value: parseFloat(data.pendingAmount),
        format: { kind: "currency", currency: "CNY" },
      },
    ];

    return (
      <StatsDisplay
        id="my-earnings"
        title="我的收入统计"
        stats={stats}
        footer={
          data.dailyStats.length > 0 && (
            <div className="p-2">
              <span className="ml-2 text-muted-foreground relative text-[10px] sm:text-xs font-normal tracking-wider uppercase opacity-90 animate-in fade-in slide-in-from-bottom-1 duration-500 ease-[cubic-bezier(0.16,1,0.3,1)] fill-mode-both">
                收入趋势图
              </span>
              <DailyTrendChart dailyStats={data.dailyStats} />
            </div>
          )
        }
      />
    );
  },
});
