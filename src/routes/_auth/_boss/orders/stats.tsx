import { createFileRoute, useNavigate } from "@tanstack/react-router";
import { NavBar, ProgressBar, CapsuleTabs } from "antd-mobile";
import { TrendingUp, Users, Package, ClipboardList, ChevronLeft } from "lucide-react";
import { useQuery } from "@tanstack/react-query";
import { statsApi } from "@/api";
import { useState, useMemo } from "react";
import { Chart, StatsCard, StatsGrid, DateRangeButton } from "@/components";
import { useDateRange } from "@/hooks";
import type { EChartsOption } from "echarts";

export const Route = createFileRoute("/_auth/_boss/orders/stats")({
  component: OrderStatsPage,
});

type TrendMode = "daily" | "monthly";

function OrderStatsPage() {
  const navigate = useNavigate();
  const [trendMode, setTrendMode] = useState<TrendMode>("daily");

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
  const { data: overviewData, isLoading: overviewLoading } = useQuery({
    queryKey: ["order-overview", dateParams.startDate, dateParams.endDate],
    queryFn: () => statsApi.orderOverview(dateParams),
  });

  const { data: dailyData, isLoading: dailyLoading } = useQuery({
    queryKey: ["daily-order-stats", dateParams.startDate, dateParams.endDate],
    queryFn: () => statsApi.dailyOrderStats(dateParams),
    enabled: trendMode === "daily",
  });

  const { data: monthlyData, isLoading: monthlyLoading } = useQuery({
    queryKey: ["monthly-order-stats", dateParams.startDate, dateParams.endDate],
    queryFn: () => statsApi.monthlyOrderStats(dateParams),
    enabled: trendMode === "monthly",
  });

  const { data: customerData, isLoading: customerLoading } = useQuery({
    queryKey: ["customer-contribution", dateParams.startDate, dateParams.endDate],
    queryFn: () => statsApi.customerContribution(dateParams),
  });

  const { data: progressData, isLoading: progressLoading } = useQuery({
    queryKey: ["order-progress", dateParams.startDate, dateParams.endDate],
    queryFn: () => statsApi.orderProgress(dateParams),
  });

  const overview = overviewData;
  const dailyStats = dailyData?.list ?? [];
  const monthlyStats = monthlyData?.list ?? [];
  const customers = customerData?.list ?? [];
  const orders = progressData?.list ?? [];

  // 订单状态分布饼图
  const statusPieOption: EChartsOption = useMemo(() => {
    if (!overview) return {};
    const data = [
      { name: "待处理", value: overview.pendingOrders, itemStyle: { color: "#f59e0b" } },
      { name: "进行中", value: overview.processingOrders, itemStyle: { color: "#3b82f6" } },
      { name: "已完成", value: overview.completedOrders, itemStyle: { color: "#22c55e" } },
      { name: "已出货", value: overview.deliveredOrders, itemStyle: { color: "#8b5cf6" } },
      { name: "已取消", value: overview.cancelledOrders, itemStyle: { color: "#ef4444" } },
    ].filter((d) => d.value > 0);

    return {
      tooltip: { trigger: "item", formatter: "{b}: {c} ({d}%)" },
      legend: { show: false },
      series: [{
        type: "pie",
        radius: ["40%", "70%"],
        avoidLabelOverlap: true,
        itemStyle: { borderRadius: 4 },
        label: { show: true, formatter: "{b}\n{c}", fontSize: 10 },
        emphasis: { label: { show: true, fontSize: 12, fontWeight: "bold" } },
        data,
      }],
    };
  }, [overview]);

  // 每日趋势图
  const dailyLineOption: EChartsOption = useMemo(() => {
    const dates = dailyStats.map((d) => d.date.slice(5));
    const counts = dailyStats.map((d) => d.orderCount);
    const amounts = dailyStats.map((d) => parseFloat(d.totalAmount) / 1000);

    return {
      tooltip: {
        trigger: "axis",
        formatter: (params: unknown) => {
          const p = params as { name: string; value: number; seriesName: string }[];
          return p.map((item) =>
            item.seriesName === "金额(千)"
              ? `${item.name}: ¥${(item.value * 1000).toFixed(0)}`
              : `${item.name}: ${item.value}单`
          ).join("<br/>");
        },
      },
      legend: { data: ["订单数", "金额(千)"], bottom: 0, textStyle: { fontSize: 10 } },
      grid: { left: 10, right: 10, bottom: 30, top: 20, containLabel: true },
      xAxis: { type: "category", data: dates, axisLabel: { fontSize: 10 } },
      yAxis: [
        { type: "value", axisLabel: { fontSize: 10 } },
        { type: "value", axisLabel: { fontSize: 10, formatter: "{value}千" } },
      ],
      dataZoom: [{ type: "inside", start: 0, end: dates.length > 15 ? 70 : 100 }],
      series: [
        { name: "订单数", type: "bar", data: counts, barMaxWidth: 20, itemStyle: { color: "#3b82f6" } },
        { name: "金额(千)", type: "line", yAxisIndex: 1, data: amounts, smooth: true, itemStyle: { color: "#22c55e" } },
      ],
    };
  }, [dailyStats]);

  // 月度趋势图
  const monthlyLineOption: EChartsOption = useMemo(() => {
    const months = monthlyStats.map((m) => m.month.slice(5));
    const counts = monthlyStats.map((m) => m.orderCount);
    const amounts = monthlyStats.map((m) => parseFloat(m.totalAmount) / 10000);

    return {
      tooltip: {
        trigger: "axis",
        formatter: (params: unknown) => {
          const p = params as { name: string; value: number; seriesName: string }[];
          return p.map((item) =>
            item.seriesName === "金额(万)"
              ? `${item.name}月: ¥${(item.value * 10000).toFixed(0)}`
              : `${item.name}月: ${item.value}单`
          ).join("<br/>");
        },
      },
      legend: { data: ["订单数", "金额(万)"], bottom: 0, textStyle: { fontSize: 10 } },
      grid: { left: 10, right: 10, bottom: 30, top: 20, containLabel: true },
      xAxis: { type: "category", data: months, axisLabel: { fontSize: 10 } },
      yAxis: [
        { type: "value", axisLabel: { fontSize: 10 } },
        { type: "value", axisLabel: { fontSize: 10, formatter: "{value}万" } },
      ],
      series: [
        { name: "订单数", type: "bar", data: counts, barMaxWidth: 20, itemStyle: { color: "#3b82f6" } },
        { name: "金额(万)", type: "line", yAxisIndex: 1, data: amounts, smooth: true, itemStyle: { color: "#22c55e" } },
      ],
    };
  }, [monthlyStats]);

  // 客户贡献度饼图
  const customerPieOption: EChartsOption = useMemo(() => {
    const data = customers.slice(0, 8).map((c) => ({
      name: c.customerName,
      value: parseFloat(c.totalAmount),
    }));

    return {
      tooltip: { trigger: "item", formatter: "{b}: ¥{c} ({d}%)" },
      legend: { show: false },
      series: [{
        type: "pie",
        radius: ["40%", "70%"],
        avoidLabelOverlap: true,
        itemStyle: { borderRadius: 4 },
        label: { show: true, formatter: "{b}\n{d}%", fontSize: 10 },
        emphasis: { label: { show: true, fontSize: 12, fontWeight: "bold" } },
        data,
      }],
    };
  }, [customers]);

  const trendLoading = trendMode === "daily" ? dailyLoading : monthlyLoading;
  const trendStats = trendMode === "daily" ? dailyStats : monthlyStats;
  const trendOption = trendMode === "daily" ? dailyLineOption : monthlyLineOption;
  const isLoading = overviewLoading || customerLoading || progressLoading;

  return (
    <div className="flex flex-col h-full bg-gray-50">
      <NavBar onBack={() => navigate({ to: "/orders" })} backIcon={<ChevronLeft size={24} />}>订单统计</NavBar>

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

        {isLoading ? (
          <div className="text-center py-16 text-gray-500">加载中...</div>
        ) : (
          <>
            {/* 概览卡片 */}
            <div className="bg-white mt-2 p-4">
              <div className="flex items-center gap-2 text-sm font-medium text-gray-600 mb-3">
                <Package size={16} />
                订单概览
              </div>
              <StatsGrid columns={3}>
                <StatsCard value={overview?.totalOrders ?? 0} label="总订单" color="blue" />
                <StatsCard value={overview?.totalQuantity ?? 0} label="总数量" color="green" />
                <StatsCard
                  value={parseFloat(overview?.totalAmount ?? "0").toFixed(0)}
                  label="总金额"
                  color="purple"
                  prefix="¥"
                  small
                />
              </StatsGrid>
            </div>

            {/* 订单状态分布 */}
            <div className="bg-white mt-2 p-4">
              <div className="flex items-center gap-2 text-sm font-medium text-gray-600 mb-2">
                <ClipboardList size={16} />
                订单状态分布
              </div>
              {overview && overview.totalOrders > 0 ? (
                <Chart option={statusPieOption} height={200} />
              ) : (
                <div className="text-center py-8 text-gray-400 text-sm">暂无数据</div>
              )}
            </div>

            {/* 订单趋势 */}
            <div className="bg-white mt-2 p-4">
              <div className="flex items-center justify-between mb-2">
                <div className="flex items-center gap-2 text-sm font-medium text-gray-600">
                  <TrendingUp size={16} />
                  订单趋势
                </div>
                <CapsuleTabs activeKey={trendMode} onChange={(key) => setTrendMode(key as TrendMode)}>
                  <CapsuleTabs.Tab title="按日" key="daily" />
                  <CapsuleTabs.Tab title="按月" key="monthly" />
                </CapsuleTabs>
              </div>
              {trendLoading ? (
                <div className="text-center py-8 text-gray-400 text-sm">加载中...</div>
              ) : trendStats.length > 0 ? (
                <Chart option={trendOption} height={220} />
              ) : (
                <div className="text-center py-8 text-gray-400 text-sm">暂无数据</div>
              )}
            </div>

            {/* 客户贡献度 */}
            <div className="bg-white mt-2 p-4">
              <div className="flex items-center gap-2 text-sm font-medium text-gray-600 mb-2">
                <Users size={16} />
                客户贡献度
              </div>
              {customers.length > 0 ? (
                <>
                  <Chart option={customerPieOption} height={200} />
                  <div className="mt-2 space-y-2">
                    {customers.slice(0, 5).map((c, i) => (
                      <div key={c.customerId} className="flex items-center gap-2 text-sm">
                        <span className="w-5 h-5 rounded-full bg-gray-100 flex items-center justify-center text-xs text-gray-500">
                          {i + 1}
                        </span>
                        <span className="flex-1 truncate">{c.customerName}</span>
                        <span className="text-gray-500">{c.orderCount}单</span>
                        <span className="text-green-600 font-medium">¥{parseFloat(c.totalAmount).toFixed(0)}</span>
                      </div>
                    ))}
                  </div>
                </>
              ) : (
                <div className="text-center py-8 text-gray-400 text-sm">暂无数据</div>
              )}
            </div>

            {/* 订单进度概览 */}
            <div className="bg-white mt-2 p-4 mb-4">
              <div className="flex items-center gap-2 text-sm font-medium text-gray-600 mb-3">
                <Package size={16} />
                进行中订单进度
              </div>
              {orders.length > 0 ? (
                <div className="space-y-3">
                  {orders.slice(0, 10).map((order) => (
                    <div
                      key={order.orderId}
                      className="border border-gray-100 rounded-lg p-3"
                      onClick={() => navigate({ to: "/orders/$id", params: { id: order.orderId } })}
                    >
                      <div className="flex items-center justify-between mb-2">
                        <div className="font-medium text-sm truncate flex-1">{order.productName}</div>
                        <div className="text-xs text-gray-400 ml-2">{order.customerName}</div>
                      </div>
                      <ProgressBar
                        percent={Math.min(order.progress * 100, 100)}
                        style={{
                          "--fill-color": order.progress >= 1 ? "#22c55e" : "#3b82f6",
                          "--track-color": "#e5e7eb",
                        }}
                      />
                      <div className="flex items-center justify-between mt-1 text-xs text-gray-500">
                        <span>{order.completedQuantity} / {order.totalQuantity} 件</span>
                        <span>{(order.progress * 100).toFixed(0)}%</span>
                      </div>
                    </div>
                  ))}
                </div>
              ) : (
                <div className="text-center py-8 text-gray-400 text-sm">暂无进行中的订单</div>
              )}
            </div>
          </>
        )}
      </div>
    </div>
  );
}
