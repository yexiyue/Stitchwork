import { createFileRoute, useNavigate } from "@tanstack/react-router";
import { NavBar } from "antd-mobile";
import { ChevronLeft } from "lucide-react";
import { useQuery } from "@tanstack/react-query";
import { statsApi } from "@/api";
import { useMemo } from "react";
import { Chart, DateRangeButton } from "@/components";
import { useDateRange } from "@/hooks";
import type { EChartsOption } from "echarts";

export const Route = createFileRoute("/_auth/_staff/my-records/stats")({
  component: StaffStatsPage,
});

function StaffStatsPage() {
  const navigate = useNavigate();

  // 日期范围
  const {
    startDate,
    endDate,
    dateParams,
    calendarVisible,
    setCalendarVisible,
    handleCalendarConfirm,
  } = useDateRange({ defaultToMonth: true });

  // 数据查询
  const { data: dailyData, isLoading: dailyLoading } = useQuery({
    queryKey: ["staff-daily-stats", dateParams.startDate, dateParams.endDate],
    queryFn: () => statsApi.dailyStats(dateParams),
  });

  const { data: orderData, isLoading: orderLoading } = useQuery({
    queryKey: ["staff-order-stats", dateParams.startDate, dateParams.endDate],
    queryFn: () => statsApi.statsByOrder(dateParams),
  });

  const { data: processData, isLoading: processLoading } = useQuery({
    queryKey: ["staff-process-stats", dateParams.startDate, dateParams.endDate],
    queryFn: () => statsApi.statsByProcess(dateParams),
  });

  const dailyStats = dailyData?.list ?? [];
  const orderStats = orderData?.list ?? [];
  const processStats = processData?.list ?? [];

  const totalQuantity = dailyStats.reduce((sum, d) => sum + d.totalQuantity, 0);
  const totalAmount = dailyStats.reduce((sum, d) => sum + parseFloat(d.totalAmount), 0);

  // 折线图
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
      xAxis: { type: "category", data: dates, axisLabel: { fontSize: 10 } },
      yAxis: { type: "value" },
      dataZoom: [{ type: "inside", start: 0, end: 100 }],
      series: [{
        type: "line",
        data: quantities,
        smooth: true,
        areaStyle: { opacity: 0.3 },
        itemStyle: { color: "#3b82f6" },
      }],
    };
  }, [dailyStats]);

  // 按订单柱状图
  const orderBarOption: EChartsOption = useMemo(() => {
    const names = orderStats.map((g) => g.name || "未知");
    const values = orderStats.map((g) => g.totalQuantity);
    return {
      tooltip: { trigger: "axis", axisPointer: { type: "shadow" } },
      grid: { left: 10, right: 20, bottom: 40, top: 20, containLabel: true },
      xAxis: { type: "category", data: names, axisLabel: { rotate: 45, fontSize: 10 } },
      yAxis: { type: "value" },
      dataZoom: [{ type: "inside", start: 0, end: names.length > 5 ? 60 : 100 }],
      series: [{
        type: "bar",
        data: values,
        barMaxWidth: 40,
        itemStyle: { color: "#3b82f6" },
        label: { show: true, position: "top", fontSize: 10 },
      }],
    };
  }, [orderStats]);

  // 按工序柱状图
  const processBarOption: EChartsOption = useMemo(() => {
    const names = processStats.map((g) => g.name || "未知");
    const values = processStats.map((g) => g.totalQuantity);
    return {
      tooltip: { trigger: "axis", axisPointer: { type: "shadow" } },
      grid: { left: 10, right: 20, bottom: 40, top: 20, containLabel: true },
      xAxis: { type: "category", data: names, axisLabel: { rotate: 45, fontSize: 10 } },
      yAxis: { type: "value" },
      dataZoom: [{ type: "inside", start: 0, end: names.length > 5 ? 60 : 100 }],
      series: [{
        type: "bar",
        data: values,
        barMaxWidth: 40,
        itemStyle: { color: "#22c55e" },
        label: { show: true, position: "top", fontSize: 10 },
      }],
    };
  }, [processStats]);

  const isLoading = dailyLoading || orderLoading || processLoading;

  return (
    <div className="flex flex-col h-full bg-gray-50">
      <NavBar onBack={() => navigate({ to: "/my-records" })} backIcon={<ChevronLeft size={24} />}>我的统计</NavBar>

      <div className="flex-1 overflow-auto">
        {/* 日期选择器 */}
        <div className="bg-white p-4">
          <DateRangeButton
            startDate={startDate}
            endDate={endDate}
            visible={calendarVisible}
            onVisibleChange={setCalendarVisible}
            onConfirm={handleCalendarConfirm}
          />
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

        {isLoading ? (
          <div className="text-center py-16 text-gray-500">加载中...</div>
        ) : (
          <>
            {/* 折线图 - 每日趋势 */}
            <div className="bg-white mx-4 mb-2 p-4 rounded-lg">
              <div className="text-sm font-medium text-gray-600 mb-2">每日趋势</div>
              {dailyStats.length === 0 ? (
                <div className="text-center py-8 text-gray-400 text-sm">暂无数据</div>
              ) : (
                <Chart option={lineOption} height={200} />
              )}
            </div>

            {/* 柱状图 - 按订单 */}
            <div className="bg-white mx-4 mb-2 p-4 rounded-lg">
              <div className="text-sm font-medium text-gray-600 mb-2">按订单分布</div>
              {orderStats.length === 0 ? (
                <div className="text-center py-8 text-gray-400 text-sm">暂无数据</div>
              ) : (
                <Chart option={orderBarOption} height={200} />
              )}
            </div>

            {/* 柱状图 - 按工序 */}
            <div className="bg-white mx-4 mb-4 p-4 rounded-lg">
              <div className="text-sm font-medium text-gray-600 mb-2">按工序分布</div>
              {processStats.length === 0 ? (
                <div className="text-center py-8 text-gray-400 text-sm">暂无数据</div>
              ) : (
                <Chart option={processBarOption} height={200} />
              )}
            </div>
          </>
        )}
      </div>
    </div>
  );
}
