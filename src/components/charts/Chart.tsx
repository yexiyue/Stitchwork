import ReactEChartsCore from "echarts-for-react/lib/core";
import echarts from "./echarts";
import type { EChartsOption } from "echarts";
import { useRef, useEffect } from "react";

interface ChartProps {
  option: EChartsOption;
  height?: number | string;
  className?: string;
  loading?: boolean;
}

export function Chart({
  option,
  height = 300,
  className = "",
  loading = false,
}: ChartProps) {
  const chartRef = useRef<ReactEChartsCore>(null);

  // Handle responsive resize
  useEffect(() => {
    const handleResize = () => {
      chartRef.current?.getEchartsInstance()?.resize();
    };
    window.addEventListener("resize", handleResize);
    return () => window.removeEventListener("resize", handleResize);
  }, []);

  return (
    <div className={className}>
      <ReactEChartsCore
        ref={chartRef}
        echarts={echarts}
        option={option}
        style={{ height }}
        showLoading={loading}
        notMerge={true}
        opts={{ renderer: "canvas" }}
      />
    </div>
  );
}
