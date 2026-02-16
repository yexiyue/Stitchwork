import { Chart } from "@/components/charts";
import type { EChartsOption } from "echarts";
import type { MyEarningsSummary } from "../types";

interface DailyTrendChartProps {
  dailyStats: MyEarningsSummary["dailyStats"];
}

// 每日趋势图组件
export function DailyTrendChart({ dailyStats }: DailyTrendChartProps) {
  // 计算最大值用于 Y 轴范围
  const values = dailyStats.map((s) => parseFloat(s.totalAmount));
  const maxValue = Math.max(...values);
  const minValue = Math.min(...values);
  const hasVariation = maxValue !== minValue;

  const chartOption: EChartsOption = {
    tooltip: {
      trigger: "axis",
      backgroundColor: "rgba(255, 255, 255, 0.95)",
      borderColor: "rgba(0, 0, 0, 0.1)",
      borderWidth: 1,
      padding: [8, 12],
      textStyle: {
        color: "#334155",
        fontSize: 12,
      },
      formatter: (params: unknown) => {
        const p = (params as { name: string; value: number }[])[0];
        return `<div style="font-weight: 500">${p.name}</div><div style="color: #10b981; font-size: 14px; font-weight: 600; margin-top: 4px">¥${p.value.toFixed(2)}</div>`;
      },
      extraCssText: "box-shadow: 0 4px 12px rgba(0, 0, 0, 0.08); border-radius: 8px;",
    },
    grid: {
      left: 8,
      right: 8,
      bottom: 28,
      top: 16,
      containLabel: true,
    },
    xAxis: {
      type: "category",
      data: dailyStats.map((d) => d.date.slice(5)), // 只显示 MM-DD
      axisLabel: {
        fontSize: 10,
        color: "#94a3b8",
        margin: 8,
      },
      axisLine: {
        show: false,
      },
      axisTick: {
        show: false,
      },
    },
    yAxis: {
      type: "value",
      show: false,
      min: hasVariation ? Math.floor(minValue * 0.9) : 0,
      max: Math.ceil(maxValue * 1.1),
    },
    dataZoom: [
      {
        type: "inside",
        start: 0,
        end: 100,
        zoomOnMouseWheel: false,
      },
    ],
    series: [
      {
        type: "line",
        data: values,
        smooth: 0.4,
        symbol: "circle",
        symbolSize: 6,
        showSymbol: dailyStats.length <= 10,
        areaStyle: {
          color: {
            type: "linear",
            x: 0,
            y: 0,
            x2: 0,
            y2: 1,
            colorStops: [
              { offset: 0, color: "rgba(16, 185, 129, 0.25)" },
              { offset: 1, color: "rgba(16, 185, 129, 0.02)" },
            ],
          },
        },
        lineStyle: {
          color: "#10b981",
          width: 2.5,
          cap: "round",
          join: "round",
        },
        itemStyle: {
          color: "#10b981",
          borderColor: "#fff",
          borderWidth: 2,
        },
        emphasis: {
          scale: 1.5,
          itemStyle: {
            shadowBlur: 8,
            shadowColor: "rgba(16, 185, 129, 0.4)",
          },
        },
      },
    ],
    animation: true,
    animationDuration: 800,
    animationEasing: "cubicOut",
  };

  return (
    <div className="mt-2 animate-in fade-in slide-in-from-bottom-2 duration-500 fill-mode-both">
      <Chart option={chartOption} height={160} />
    </div>
  );
}
