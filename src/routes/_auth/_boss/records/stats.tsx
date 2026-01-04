import { createFileRoute, useNavigate } from "@tanstack/react-router";
import { NavBar, CalendarPicker, Button, Tabs } from "antd-mobile";
import { Calendar, User } from "lucide-react";
import { useQuery } from "@tanstack/react-query";
import { statsApi } from "@/api";
import { useState, useMemo } from "react";
import dayjs from "dayjs";
import { Chart } from "@/components";
import type { EChartsOption } from "echarts";

export const Route = createFileRoute("/_auth/_boss/records/stats")({
  component: StatsPage,
});

type ChartType = "bar" | "pie" | "line";
type Dimension = "quantity" | "amount";

function StatsPage() {
  const navigate = useNavigate();

  const [startDate, setStartDate] = useState<Date>(() =>
    dayjs().startOf("month").toDate()
  );
  const [endDate, setEndDate] = useState<Date>(() =>
    dayjs().endOf("month").toDate()
  );
  const [calendarVisible, setCalendarVisible] = useState(false);
  const [chartType, setChartType] = useState<ChartType>("bar");
  const [dimension, setDimension] = useState<Dimension>("quantity");

  const dateParams = {
    startDate: dayjs(startDate).format("YYYY-MM-DD"),
    endDate: dayjs(endDate).format("YYYY-MM-DD"),
  };

  const { data: workerData, isLoading: workersLoading } = useQuery({
    queryKey: ["worker-production", dateParams.startDate, dateParams.endDate],
    queryFn: () => statsApi.workerProduction(dateParams),
  });

  const { data: dailyData, isLoading: dailyLoading } = useQuery({
    queryKey: ["daily-stats", dateParams.startDate, dateParams.endDate],
    queryFn: () => statsApi.dailyStats(dateParams),
    enabled: chartType === "line",
  });

  const workers = workerData?.list ?? [];
  const dailyStats = dailyData?.list ?? [];

  const totalQuantity = workers.reduce((sum, w) => sum + w.totalQuantity, 0);
  const totalAmount = workers.reduce(
    (sum, w) => sum + parseFloat(w.totalAmount),
    0
  );

  const barOption: EChartsOption = useMemo(() => {
    const names = workers.map((w) => w.userName || "未知");
    const values = workers.map((w) =>
      dimension === "quantity" ? w.totalQuantity : parseFloat(w.totalAmount)
    );
    return {
      tooltip: {
        trigger: "axis",
        axisPointer: { type: "shadow" },
      },
      grid: { left: 10, right: 20, bottom: 40, top: 20, containLabel: true },
      xAxis: {
        type: "category",
        data: names,
        axisLabel: { rotate: 45, fontSize: 10 },
      },
      yAxis: { type: "value" },
      dataZoom: [{ type: "inside", start: 0, end: names.length > 5 ? 60 : 100 }],
      series: [
        {
          type: "bar",
          data: values,
          itemStyle: {
            color: dimension === "quantity" ? "#3b82f6" : "#22c55e",
          },
          label: {
            show: true,
            position: "top",
            fontSize: 10,
            formatter: dimension === "amount" ? "¥{c}" : "{c}",
          },
        },
      ],
    };
  }, [workers, dimension]);

  const pieOption: EChartsOption = useMemo(() => {
    const data = workers.map((w) => ({
      name: w.userName || "未知",
      value: dimension === "quantity" ? w.totalQuantity : parseFloat(w.totalAmount),
    }));
    return {
      tooltip: {
        trigger: "item",
        formatter: dimension === "amount" ? "{b}: ¥{c} ({d}%)" : "{b}: {c} ({d}%)",
      },
      legend: { show: false },
      series: [
        {
          type: "pie",
          radius: ["40%", "70%"],
          avoidLabelOverlap: true,
          itemStyle: { borderRadius: 4 },
          label: {
            show: true,
            formatter: "{b}\n{d}%",
            fontSize: 10,
          },
          emphasis: {
            label: { show: true, fontSize: 12, fontWeight: "bold" },
          },
          data,
        },
      ],
    };
  }, [workers, dimension]);

  const lineOption: EChartsOption = useMemo(() => {
    const dates = dailyStats.map((d) => d.date.slice(5)); // MM-DD
    const values = dailyStats.map((d) =>
      dimension === "quantity" ? d.totalQuantity : parseFloat(d.totalAmount)
    );
    return {
      tooltip: {
        trigger: "axis",
        formatter: (params: unknown) => {
          const p = (params as { name: string; value: number }[])[0];
          return dimension === "amount"
            ? `${p.name}: ¥${p.value}`
            : `${p.name}: ${p.value}件`;
        },
      },
      grid: { left: 10, right: 20, bottom: 40, top: 20, containLabel: true },
      xAxis: {
        type: "category",
        data: dates,
        axisLabel: { fontSize: 10 },
      },
      yAxis: { type: "value" },
      dataZoom: [{ type: "inside", start: 0, end: dates.length > 10 ? 80 : 100 }],
      series: [
        {
          type: "line",
          data: values,
          smooth: true,
          areaStyle: { opacity: 0.3 },
          itemStyle: { color: dimension === "quantity" ? "#3b82f6" : "#22c55e" },
        },
      ],
    };
  }, [dailyStats, dimension]);

  const isLoading = chartType === "line" ? dailyLoading : workersLoading;
  const hasData = chartType === "line" ? dailyStats.length > 0 : workers.length > 0;

  return (
    <div className="flex flex-col h-full bg-gray-50">
      <NavBar onBack={() => navigate({ to: "/records" })}>计件统计</NavBar>

      {/* 日期选择器 */}
      <div className="bg-white p-4 flex items-center gap-2">
        <Calendar size={18} className="text-gray-400" />
        <Button size="small" fill="outline" onClick={() => setCalendarVisible(true)}>
          {dayjs(startDate).format("YYYY-MM-DD")} 至 {dayjs(endDate).format("YYYY-MM-DD")}
        </Button>
      </div>

      {/* 总计卡片 */}
      <div className="bg-white mt-2 p-4">
        <div className="text-sm text-gray-500 mb-2">统计总计</div>
        <div className="flex justify-around">
          <div className="text-center">
            <div className="text-2xl font-bold text-blue-600">{totalQuantity}</div>
            <div className="text-sm text-gray-500">总数量</div>
          </div>
          <div className="text-center">
            <div className="text-2xl font-bold text-green-600">¥{totalAmount.toFixed(2)}</div>
            <div className="text-sm text-gray-500">总金额</div>
          </div>
        </div>
      </div>

      {/* 图表区域 */}
      <div className="flex-1 overflow-auto mt-2 bg-white">
        {/* 维度切换 */}
        <Tabs
          activeKey={dimension}
          onChange={(key) => setDimension(key as Dimension)}
          className="px-4"
        >
          <Tabs.Tab title="数量" key="quantity" />
          <Tabs.Tab title="金额" key="amount" />
        </Tabs>

        {/* 图表类型切换 */}
        <Tabs
          activeKey={chartType}
          onChange={(key) => setChartType(key as ChartType)}
          className="px-4 border-t border-gray-100"
        >
          <Tabs.Tab title="柱状图" key="bar" />
          <Tabs.Tab title="饼图" key="pie" />
          <Tabs.Tab title="趋势图" key="line" />
        </Tabs>

        {/* 图表 */}
        <div className="p-4">
          {isLoading ? (
            <div className="text-center py-16 text-gray-500">加载中...</div>
          ) : !hasData ? (
            <div className="text-center py-16 text-gray-500">暂无数据</div>
          ) : (
            <Chart
              option={chartType === "bar" ? barOption : chartType === "pie" ? pieOption : lineOption}
              height={280}
            />
          )}
        </div>
      </div>

      {/* 员工列表 */}
      <div className="bg-white mt-2 border-t border-gray-100">
        <div className="px-4 py-3 text-sm font-medium text-gray-600">员工明细</div>
        {workers.length === 0 ? (
          <div className="text-center py-4 text-gray-500 text-sm">暂无数据</div>
        ) : (
          <div className="max-h-48 overflow-auto">
            {workers.map((worker) => (
              <div
                key={worker.userId}
                className="px-4 py-3 flex items-center gap-3 border-t border-gray-50"
              >
                <div className="shrink-0 w-8 h-8 rounded-full bg-blue-100 flex items-center justify-center">
                  <User size={16} className="text-blue-500" />
                </div>
                <div className="flex-1 min-w-0 text-sm truncate">
                  {worker.userName || "未知员工"}
                </div>
                <div className="text-sm text-gray-600">{worker.totalQuantity}件</div>
                <div className="text-sm font-medium text-green-600">
                  ¥{parseFloat(worker.totalAmount).toFixed(2)}
                </div>
              </div>
            ))}
          </div>
        )}
      </div>

      {/* 日期范围选择器 */}
      <CalendarPicker
        visible={calendarVisible}
        selectionMode="range"
        defaultValue={[startDate, endDate]}
        onClose={() => setCalendarVisible(false)}
        onConfirm={(val) => {
          if (val && val.length === 2 && val[0] && val[1]) {
            setStartDate(val[0]);
            setEndDate(val[1]);
          }
          setCalendarVisible(false);
        }}
        title="选择日期范围"
      />
    </div>
  );
}
