import { createFileRoute, useNavigate } from "@tanstack/react-router";
import { NavBar, CapsuleTabs } from "antd-mobile";
import { ChevronLeft } from "lucide-react";
import { useQuery } from "@tanstack/react-query";
import { statsApi } from "@/api";
import { useState, useMemo } from "react";
import { Chart, StatsCard, StatsGrid, DateRangeButton } from "@/components";
import { useDateRange, useWorkshopSettings } from "@/hooks";
import type { EChartsOption } from "echarts";

export const Route = createFileRoute("/_auth/_boss/records/stats")({
  component: StatsPage,
});

type Dimension = "quantity" | "amount";

function StatsPage() {
  const navigate = useNavigate();
  const [dimension, setDimension] = useState<Dimension>("quantity");
  const { pieceUnit } = useWorkshopSettings();

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
  const { data: workerData, isLoading: workersLoading } = useQuery({
    queryKey: ["worker-production", dateParams.startDate, dateParams.endDate],
    queryFn: () => statsApi.workerProduction(dateParams),
  });

  const { data: dailyData, isLoading: dailyLoading } = useQuery({
    queryKey: ["daily-stats", dateParams.startDate, dateParams.endDate],
    queryFn: () => statsApi.dailyStats(dateParams),
  });

  const workers = workerData?.list ?? [];
  const dailyStats = dailyData?.list ?? [];

  const totalQuantity = workers.reduce((sum, w) => sum + w.totalQuantity, 0);
  const totalAmount = workers.reduce((sum, w) => sum + parseFloat(w.totalAmount), 0);

  // 柱状图
  const barOption: EChartsOption = useMemo(() => {
    const names = workers.map((w) => w.userName || "未知");
    const values = workers.map((w) =>
      dimension === "quantity" ? w.totalQuantity : parseFloat(w.totalAmount)
    );
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
        itemStyle: { color: dimension === "quantity" ? "#3b82f6" : "#22c55e" },
        label: { show: true, position: "top", fontSize: 10, formatter: dimension === "amount" ? "¥{c}" : "{c}" },
      }],
    };
  }, [workers, dimension]);

  // 饼图
  const pieOption: EChartsOption = useMemo(() => {
    const data = workers.map((w) => ({
      name: w.userName || "未知",
      value: dimension === "quantity" ? w.totalQuantity : parseFloat(w.totalAmount),
    }));
    return {
      tooltip: { trigger: "item", formatter: dimension === "amount" ? "{b}: ¥{c} ({d}%)" : "{b}: {c} ({d}%)" },
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
  }, [workers, dimension]);

  // 折线图
  const lineOption: EChartsOption = useMemo(() => {
    const dates = dailyStats.map((d) => d.date.slice(5));
    const values = dailyStats.map((d) =>
      dimension === "quantity" ? d.totalQuantity : parseFloat(d.totalAmount)
    );
    return {
      tooltip: {
        trigger: "axis",
        formatter: (params: unknown) => {
          const p = (params as { name: string; value: number }[])[0];
          return dimension === "amount" ? `${p.name}: ¥${p.value}` : `${p.name}: ${p.value}${pieceUnit}`;
        },
      },
      grid: { left: 10, right: 20, bottom: 40, top: 20, containLabel: true },
      xAxis: { type: "category", data: dates, axisLabel: { fontSize: 10 } },
      yAxis: { type: "value" },
      dataZoom: [{ type: "inside", start: 0, end: dates.length > 10 ? 80 : 100 }],
      series: [{
        type: "line",
        data: values,
        smooth: true,
        areaStyle: { opacity: 0.3 },
        itemStyle: { color: dimension === "quantity" ? "#3b82f6" : "#22c55e" },
      }],
    };
  }, [dailyStats, dimension, pieceUnit]);

  const isLoading = workersLoading || dailyLoading;

  return (
    <div className="flex flex-col h-full bg-gray-50">
      <NavBar
        onBack={() => navigate({ to: "/records" })}
        backIcon={<ChevronLeft size={24} />}
      >
        计件统计
      </NavBar>

      <div className="flex-1 overflow-auto">
        {/* 日期选择器 */}
        <div className="bg-white p-2">
          <DateRangeButton
            startDate={startDate}
            endDate={endDate}
            visible={calendarVisible}
            onVisibleChange={setCalendarVisible}
            onConfirm={handleCalendarConfirm}
          />
        </div>

        {/* 总计卡片 */}
        <div className="bg-white mt-2 p-4">
          <div className="text-sm text-gray-500 mb-2">统计总计</div>
          <StatsGrid columns={2}>
            <StatsCard value={totalQuantity} label="总数量" color="blue" />
            <StatsCard value={totalAmount.toFixed(2)} label="总金额" color="green" prefix="¥" />
          </StatsGrid>
        </div>

        {isLoading ? (
          <div className="text-center py-16 text-gray-500">加载中...</div>
        ) : (
          <>
            {/* 柱状图 - 员工产量 */}
            <div className="bg-white mt-2 p-4">
              <div className="flex items-center justify-between mb-2">
                <div className="text-sm font-medium text-gray-600">员工产量</div>
                <CapsuleTabs activeKey={dimension} onChange={(key) => setDimension(key as Dimension)} className="text-sm">
                  <CapsuleTabs.Tab title="数量" key="quantity" />
                  <CapsuleTabs.Tab title="金额" key="amount" />
                </CapsuleTabs>
              </div>
              {workers.length === 0 ? (
                <div className="text-center py-8 text-gray-400 text-sm">暂无数据</div>
              ) : (
                <Chart option={barOption} height={220} />
              )}
            </div>

            {/* 饼图 - 产量占比 */}
            <div className="bg-white mt-2 p-4">
              <div className="text-sm font-medium text-gray-600 mb-2">产量占比</div>
              {workers.length === 0 ? (
                <div className="text-center py-8 text-gray-400 text-sm">暂无数据</div>
              ) : (
                <Chart option={pieOption} height={220} />
              )}
            </div>

            {/* 折线图 - 每日趋势 */}
            <div className="bg-white mt-2 p-4">
              <div className="text-sm font-medium text-gray-600 mb-2">每日趋势</div>
              {dailyStats.length === 0 ? (
                <div className="text-center py-8 text-gray-400 text-sm">暂无数据</div>
              ) : (
                <Chart option={lineOption} height={220} />
              )}
            </div>
          </>
        )}
      </div>
    </div>
  );
}
