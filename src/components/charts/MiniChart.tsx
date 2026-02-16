import ReactEChartsCore from "echarts-for-react/lib/core";
import echarts from "./echarts";
import type { EChartsOption } from "echarts";

interface MiniChartProps {
  data: number[];
  color?: string;
  height?: number;
}

export function MiniChart({ data, color = "#3b82f6", height = 40 }: MiniChartProps) {
  const maxValue = Math.max(...data, 1); // 确保至少有 1 的范围
  const option: EChartsOption = {
    grid: { left: 0, right: 0, top: 5, bottom: 5 },
    xAxis: { type: "category", show: false, data: data.map((_, i) => i) },
    yAxis: { type: "value", show: false, min: 0, max: maxValue },
    series: [
      {
        type: "line",
        data,
        smooth: true,
        symbol: "none",
        areaStyle: { opacity: 0.3, color },
        lineStyle: { color, width: 1.5 },
      },
    ],
  };

  return (
    <ReactEChartsCore
      echarts={echarts}
      option={option}
      style={{ height, width: "100%" }}
      opts={{ renderer: "canvas" }}
    />
  );
}
