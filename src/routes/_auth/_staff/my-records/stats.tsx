import { createFileRoute, useNavigate } from "@tanstack/react-router";
import { NavBar, Tabs, CalendarPicker, Button } from "antd-mobile";
import { Calendar } from "lucide-react";
import { useQuery } from "@tanstack/react-query";
import { statsApi } from "@/api";
import { useState, useMemo } from "react";
import dayjs from "dayjs";
import { Chart } from "@/components";
import type { EChartsOption } from "echarts";

export const Route = createFileRoute("/_auth/_staff/my-records/stats")({
  component: StaffStatsPage,
});

type ChartType = "line" | "bar";
type GroupBy = "order" | "process";

function StaffStatsPage() {
  const navigate = useNavigate();
  const [chartType, setChartType] = useState<ChartType>("line");
  const [groupBy, setGroupBy] = useState<GroupBy>("order");
  const [startDate, setStartDate] = useState<Date>(() =>
    dayjs().startOf("month").toDate()
  );
  const [endDate, setEndDate] = useState<Date>(() =>
    dayjs().endOf("month").toDate()
  );
  const [calendarVisible, setCalendarVisible] = useState(false);

  const dateParams = {
    startDate: dayjs(startDate).format("YYYY-MM-DD"),
    endDate: dayjs(endDate).format("YYYY-MM-DD"),
  };

  const { data: dailyData, isLoading: dailyLoading } = useQuery({
    queryKey: ["staff-daily-stats", dateParams.startDate, dateParams.endDate],
    queryFn: () => statsApi.dailyStats(dateParams),
    enabled: chartType === "line",
  });

  const { data: orderData, isLoading: orderLoading } = useQuery({
    queryKey: ["staff-order-stats", dateParams.startDate, dateParams.endDate],
    queryFn: () => statsApi.statsByOrder(dateParams),
    enabled: chartType === "bar" && groupBy === "order",
  });

  const { data: processData, isLoading: processLoading } = useQuery({
    queryKey: ["staff-process-stats", dateParams.startDate, dateParams.endDate],
    queryFn: () => statsApi.statsByProcess(dateParams),
    enabled: chartType === "bar" && groupBy === "process",
  });

  const dailyStats = dailyData?.list ?? [];
  const orderStats = orderData?.list ?? [];
  const processStats = processData?.list ?? [];
  const groupStats = groupBy === "order" ? orderStats : processStats;

  // Calculate totals
  const totalQuantity = dailyStats.reduce((sum, d) => sum + d.totalQuantity, 0);
  const totalAmount = dailyStats.reduce((sum, d) => sum + parseFloat(d.totalAmount), 0);

  const lineOption: EChartsOption = useMemo(() => {
    const dates = dailyStats.map((d) => d.date.slice(5));
    const quantities = dailyStats.map((d) => d.totalQuantity);
    return {
      tooltip: {
        trigger: "axis",
        formatter: (params: unknown) => {
          const p = (params as { name: string; value: number }[])[0];
          return `${p.name}: ${p.value}件`;
        },
      },
      grid: { left: 10, right: 20, bottom: 40, top: 20, containLabel: true },
      xAxis: {
        type: "category",
        data: dates,
        axisLabel: { fontSize: 10 },
      },
      yAxis: { type: "value" },
      dataZoom: [{ type: "inside", start: 0, end: 100 }],
      series: [
        {
          type: "line",
          data: quantities,
          smooth: true,
          areaStyle: { opacity: 0.3 },
          itemStyle: { color: "#3b82f6" },
        },
      ],
    };
  }, [dailyStats]);

  const barOption: EChartsOption = useMemo(() => {
    const names = groupStats.map((g) => g.name || "未知");
    const values = groupStats.map((g) => g.totalQuantity);
    const colors = [
      "#3b82f6", "#22c55e", "#f59e0b", "#ef4444", "#8b5cf6",
      "#ec4899", "#06b6d4", "#84cc16", "#f97316", "#6366f1",
    ];
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
          data: values.map((v, i) => ({
            value: v,
            itemStyle: { color: colors[i % colors.length] },
          })),
          label: {
            show: true,
            position: "top",
            fontSize: 10,
          },
        },
      ],
    };
  }, [groupStats]);

  const isLoading =
    chartType === "line"
      ? dailyLoading
      : groupBy === "order"
        ? orderLoading
        : processLoading;

  const hasData =
    chartType === "line"
      ? dailyStats.length > 0
      : groupStats.length > 0;

  return (
    <div className="flex flex-col h-full bg-gray-50">
      <NavBar onBack={() => navigate({ to: "/my-records" })}>我的统计</NavBar>

      {/* 日期选择器 */}
      <div className="bg-white p-4 flex items-center gap-2">
        <Calendar size={18} className="text-gray-400" />
        <Button size="small" fill="outline" onClick={() => setCalendarVisible(true)}>
          {dayjs(startDate).format("YYYY-MM-DD")} 至 {dayjs(endDate).format("YYYY-MM-DD")}
        </Button>
      </div>

      {/* 总计卡片 */}
      <div className="bg-linear-to-r from-blue-500 to-blue-600 m-4 p-4 rounded-lg text-white">
        <div className="text-sm opacity-80 mb-2">统计总计</div>
        <div className="flex justify-around">
          <div className="text-center">
            <div className="text-2xl font-bold">{totalQuantity}</div>
            <div className="text-xs opacity-80">件</div>
          </div>
          <div className="w-px bg-white/30" />
          <div className="text-center">
            <div className="text-2xl font-bold">¥{totalAmount.toFixed(0)}</div>
            <div className="text-xs opacity-80">金额</div>
          </div>
        </div>
      </div>

      {/* 图表区域 */}
      <div className="flex-1 overflow-auto bg-white mx-4 mb-4 rounded-lg">
        {/* 图表类型切换 */}
        <Tabs
          activeKey={chartType}
          onChange={(key) => setChartType(key as ChartType)}
          className="px-4"
        >
          <Tabs.Tab title="趋势图" key="line" />
          <Tabs.Tab title="分布图" key="bar" />
        </Tabs>

        {/* 分组方式切换 (仅柱状图) */}
        {chartType === "bar" && (
          <Tabs
            activeKey={groupBy}
            onChange={(key) => setGroupBy(key as GroupBy)}
            className="px-4 border-t border-gray-100"
          >
            <Tabs.Tab title="按订单" key="order" />
            <Tabs.Tab title="按工序" key="process" />
          </Tabs>
        )}

        {/* 图表 */}
        <div className="p-4">
          {isLoading ? (
            <div className="text-center py-16 text-gray-500">加载中...</div>
          ) : !hasData ? (
            <div className="text-center py-16 text-gray-500">暂无数据</div>
          ) : (
            <Chart
              option={chartType === "line" ? lineOption : barOption}
              height={280}
            />
          )}
        </div>

        {/* 分组明细 (柱状图时显示) */}
        {chartType === "bar" && groupStats.length > 0 && (
          <div className="border-t border-gray-100">
            <div className="px-4 py-3 text-sm font-medium text-gray-600">
              {groupBy === "order" ? "订单" : "工序"}明细
            </div>
            <div className="max-h-40 overflow-auto">
              {groupStats.map((item) => (
                <div
                  key={item.id}
                  className="px-4 py-2 flex items-center justify-between border-t border-gray-50"
                >
                  <div className="text-sm truncate flex-1 min-w-0">
                    {item.name || "未知"}
                  </div>
                  <div className="text-sm text-gray-600 ml-2">{item.totalQuantity}件</div>
                  <div className="text-sm font-medium text-green-600 ml-2">
                    ¥{parseFloat(item.totalAmount).toFixed(0)}
                  </div>
                </div>
              ))}
            </div>
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
