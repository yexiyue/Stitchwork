import { useNavigate } from "@tanstack/react-router";
import { Card } from "antd-mobile";
import { FileEdit, Store, TrendingUp, Package, ImageIcon } from "lucide-react";
import { useQuery } from "@tanstack/react-query";
import { homeApi, statsApi } from "@/api";
import { useAuthStore } from "@/stores/auth";
import { MiniChart, RelativeTime, Chart, OssImage } from "@/components";
import type { StaffOverview, Activity } from "@/types";
import type { EChartsOption } from "echarts";
import dayjs from "dayjs";
import { useMemo } from "react";

export function StaffHome() {
  const navigate = useNavigate();
  const user = useAuthStore((s) => s.user);

  const { data: overview } = useQuery({
    queryKey: ["home-overview"],
    queryFn: homeApi.overview,
  });

  const { data: activities } = useQuery({
    queryKey: ["home-activities"],
    queryFn: homeApi.activities,
  });

  const { data: dailyData } = useQuery({
    queryKey: ["daily-stats-mini"],
    queryFn: () =>
      statsApi.dailyStats({
        startDate: dayjs().subtract(6, "day").format("YYYY-MM-DD"),
        endDate: dayjs().format("YYYY-MM-DD"),
      }),
  });

  // 本月统计参数
  const monthParams = {
    startDate: dayjs().startOf("month").format("YYYY-MM-DD"),
    endDate: dayjs().endOf("month").format("YYYY-MM-DD"),
  };

  const { data: byOrderData } = useQuery({
    queryKey: ["staff-stats-by-order", monthParams],
    queryFn: () => statsApi.statsByOrder(monthParams),
  });

  const staff = overview as StaffOverview | undefined;
  const chartData = dailyData?.list?.map((d) => d.totalQuantity) ?? [];
  const byOrder = byOrderData?.list ?? [];

  // 按订单统计柱状图
  const orderBarOption: EChartsOption = useMemo(() => {
    const data = byOrder.slice(0, 5);
    return {
      tooltip: { trigger: "axis", formatter: "{b}: {c}件" },
      grid: { left: 10, right: 10, bottom: 10, top: 10, containLabel: true },
      xAxis: {
        type: "category",
        data: data.map((d) =>
          d.name.length > 6 ? d.name.slice(0, 6) + "..." : d.name
        ),
        axisLabel: { fontSize: 10, rotate: 15 },
      },
      yAxis: { type: "value", axisLabel: { fontSize: 10 } },
      series: [
        {
          type: "bar",
          data: data.map((d) => d.totalQuantity),
          barMaxWidth: 30,
          itemStyle: { color: "#3b82f6", borderRadius: [4, 4, 0, 0] },
        },
      ],
    };
  }, [byOrder]);

  return (
    <div className="p-4 pb-20">
      {user?.workshop ? (
        <div className="p-4 bg-gray-50 rounded-lg">
          <div className="flex items-start gap-3">
            {user.workshop.image ? (
              <OssImage
                src={user.workshop.image}
                className="w-16 h-16 rounded-lg object-cover"
              />
            ) : (
              <div className="w-16 h-16 rounded-lg bg-gray-200 flex items-center justify-center">
                <Store size={24} className="text-gray-400" />
              </div>
            )}
            <div className="flex-1 min-w-0">
              <h2 className="font-medium">{user.workshop.name}</h2>
              {user.workshop.address && (
                <p className="text-sm text-gray-500 mt-1">
                  {user.workshop.address}
                </p>
              )}
            </div>
          </div>
        </div>
      ) : (
        <div className="p-4 bg-gray-50 rounded-lg text-center text-gray-500">
          暂未加入工坊
        </div>
      )}

      {/* 本月数据卡片 */}
      <div className="mt-4 grid grid-cols-3 gap-3">
        <Card className="p-3!">
          <div className="text-xs text-gray-500">本月产量</div>
          <div className="text-xl font-bold text-blue-600 mt-1">
            {staff?.monthQuantity ?? 0}
          </div>
          <div className="text-xs text-gray-400">件</div>
        </Card>
        <Card className="p-3!">
          <div className="text-xs text-gray-500">本月收入</div>
          <div className="text-lg font-bold text-green-600 mt-1">
            ¥{parseFloat(staff?.monthAmount ?? "0").toFixed(0)}
          </div>
          <div className="text-xs text-gray-400">元</div>
        </Card>
        <Card className="p-3!">
          <div className="text-xs text-gray-500">参与订单</div>
          <div className="text-xl font-bold text-purple-600 mt-1">
            {byOrder.length}
          </div>
          <div className="text-xs text-gray-400">个</div>
        </Card>
      </div>

      {/* 近7日趋势 */}
      {chartData.length > 0 && (
        <Card className="mt-4">
          <div className="flex items-center gap-2 text-sm text-gray-500 mb-2">
            <TrendingUp size={14} />
            近7日产量趋势
          </div>
          <MiniChart data={chartData} color="#22c55e" height={60} />
        </Card>
      )}

      {/* 按订单统计 */}
      {byOrder.length > 0 && (
        <Card className="mt-4">
          <div className="flex items-center gap-2 text-sm text-gray-500 mb-2">
            <Package size={14} />
            本月订单产量
          </div>
          <Chart option={orderBarOption} height={160} />
        </Card>
      )}

      {/* 快捷入口 */}

      <div className="grid grid-cols-2">
        <Card
          className="cursor-pointer"
          onClick={() => navigate({ to: "/my-records" })}
        >
          <div className="flex items-center gap-2">
            <FileEdit size={20} className="text-blue-500" />
            <span className="text-sm font-medium">我的计件</span>
          </div>
        </Card>
        <Card
          className="cursor-pointer"
          onClick={() => navigate({ to: "/my-records/stats" })}
        >
          <div className="flex items-center gap-2">
            <TrendingUp size={20} className="text-purple-500" />
            <span className="text-sm font-medium">统计报表</span>
          </div>
        </Card>
      </div>

      {/* 最近记录 */}
      {activities?.list && activities.list.length > 0 && (
        <div className="mt-4">
          <div className="text-sm font-medium text-gray-600 mb-2">最近记录</div>
          <div className="space-y-1">
            {activities.list.map((act: Activity) => (
              <Card key={act.id} className="shadow-sm border border-gray-100">
                <div className="flex items-center gap-3">
                  <div className="shrink-0 w-10 h-10 rounded overflow-hidden bg-gray-100 flex items-center justify-center">
                    {act.orderImage ? (
                      <OssImage
                        src={act.orderImage}
                        width="100%"
                        height="100%"
                        fit="cover"
                      />
                    ) : (
                      <ImageIcon size={16} className="text-gray-400" />
                    )}
                  </div>
                  <div className="flex-1 min-w-0">
                    <div className="flex items-center justify-between">
                      <span className="text-sm truncate">{act.orderName}</span>
                      <span
                        className={`text-xs px-1.5 py-0.5 rounded shrink-0 ${
                          act.activityType === "submit"
                            ? "bg-orange-50 text-orange-600"
                            : act.activityType === "approve"
                            ? "bg-green-50 text-green-600"
                            : "bg-red-50 text-red-600"
                        }`}
                      >
                        {act.activityType === "submit"
                          ? "待审核"
                          : act.activityType === "approve"
                          ? "已通过"
                          : "已驳回"}
                      </span>
                    </div>
                    <div className="flex items-center justify-between text-xs text-gray-400 mt-1.5">
                      <span>
                        {act.processName} · {act.quantity}件
                      </span>
                      <RelativeTime date={act.createdAt} />
                    </div>
                  </div>
                </div>
              </Card>
            ))}
          </div>
        </div>
      )}
    </div>
  );
}
