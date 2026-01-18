import { Chart } from "@/components/charts";
import type { EChartsOption } from "echarts";
import type { MyEarningsSummary } from "../types";

interface DailyTrendChartProps {
  dailyStats: MyEarningsSummary["dailyStats"];
}

// 每日趋势图组件
export function DailyTrendChart({ dailyStats }: DailyTrendChartProps) {
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
