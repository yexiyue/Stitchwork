import { useNavigate } from "@tanstack/react-router";
import { Card, Badge, ProgressBar } from "antd-mobile";
import { FileEdit, Store, Package, Users, TrendingUp } from "lucide-react";
import { useQuery } from "@tanstack/react-query";
import { homeApi, statsApi } from "@/api";
import { useAuthStore } from "@/stores/auth";
import { MiniChart, RelativeTime, Chart } from "@/components";
import type { BossOverview, Activity } from "@/types";
import type { EChartsOption } from "echarts";
import dayjs from "dayjs";
import { useMemo } from "react";

export function BossHome() {
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

  // 本月订单概览
  const monthParams = {
    startDate: dayjs().startOf("month").format("YYYY-MM-DD"),
    endDate: dayjs().endOf("month").format("YYYY-MM-DD"),
  };

  const { data: orderOverview } = useQuery({
    queryKey: ["home-order-overview", monthParams],
    queryFn: () => statsApi.orderOverview(monthParams),
  });

  const { data: customerData } = useQuery({
    queryKey: ["home-customer-contribution", monthParams],
    queryFn: () => statsApi.customerContribution(monthParams),
  });

  const { data: workerData } = useQuery({
    queryKey: ["home-worker-production", monthParams],
    queryFn: () => statsApi.workerProduction(monthParams),
  });

  const { data: progressData } = useQuery({
    queryKey: ["home-order-progress"],
    queryFn: () => statsApi.orderProgress({}),
  });

  const boss = overview as BossOverview | undefined;
  const chartData = dailyData?.list?.map((d) => d.totalQuantity) ?? [];
  const customers = customerData?.list ?? [];
  const workers = workerData?.list ?? [];
  const orders = progressData?.list ?? [];

  // 订单状态饼图
  const statusPieOption: EChartsOption = useMemo(() => {
    if (!orderOverview) return {};
    const data = [
      {
        name: "待处理",
        value: orderOverview.pendingOrders,
        itemStyle: { color: "#f59e0b" },
      },
      {
        name: "进行中",
        value: orderOverview.processingOrders,
        itemStyle: { color: "#3b82f6" },
      },
      {
        name: "已完成",
        value: orderOverview.completedOrders,
        itemStyle: { color: "#22c55e" },
      },
      {
        name: "已出货",
        value: orderOverview.deliveredOrders,
        itemStyle: { color: "#8b5cf6" },
      },
    ].filter((d) => d.value > 0);

    return {
      tooltip: { trigger: "item", formatter: "{b}: {c} ({d}%)" },
      legend: { show: false },
      series: [
        {
          type: "pie",
          radius: ["40%", "70%"],
          center: ["50%", "50%"],
          itemStyle: { borderRadius: 4 },
          label: { show: true, formatter: "{b}\n{c}", fontSize: 10 },
          data,
        },
      ],
    };
  }, [orderOverview]);

  // 客户贡献饼图
  const customerPieOption: EChartsOption = useMemo(() => {
    const data = customers.slice(0, 5).map((c) => ({
      name: c.customerName,
      value: parseFloat(c.totalAmount),
    }));

    return {
      tooltip: { trigger: "item", formatter: "{b}: ¥{c} ({d}%)" },
      legend: { show: false },
      series: [
        {
          type: "pie",
          radius: ["40%", "70%"],
          center: ["50%", "50%"],
          itemStyle: { borderRadius: 4 },
          label: { show: true, formatter: "{b}\n{d}%", fontSize: 10 },
          data,
        },
      ],
    };
  }, [customers]);

  return (
    <div className="p-4 pb-20">
      {user?.workshop ? (
        <div
          className="p-4 bg-gray-50 rounded-lg"
          onClick={() => navigate({ to: "/workshop" })}
        >
          <div className="flex items-start gap-3">
            {user.workshop.image ? (
              <img
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
        <div
          className="p-4 bg-gray-50 rounded-lg text-center text-gray-500"
          onClick={() => navigate({ to: "/workshop" })}
        >
          点击创建工坊
        </div>
      )}

      <div className="mt-4 grid grid-cols-2 gap-3">
        <Card className="p-3!">
          <div className="text-sm text-gray-500">今日产量</div>
          <div className="text-xl font-bold text-blue-600 mt-1">
            {boss?.todayQuantity ?? 0}
          </div>
          <div className="text-xs text-gray-400">
            ¥{parseFloat(boss?.todayAmount ?? "0").toFixed(2)}
          </div>
        </Card>
        <Card className="p-3!">
          <div className="text-sm text-gray-500">本月产量</div>
          <div className="text-xl font-bold text-green-600 mt-1">
            {boss?.monthQuantity ?? 0}
          </div>
          <div className="text-xs text-gray-400">
            ¥{parseFloat(boss?.monthAmount ?? "0").toFixed(2)}
          </div>
        </Card>
      </div>

      {chartData.length > 0 && (
        <Card className="mt-4 p-3!">
          <div className="flex items-center gap-2 text-sm text-gray-500 mb-2">
            <TrendingUp size={14} />
            近7日产量趋势
          </div>
          <MiniChart data={chartData} color="#3b82f6" height={60} />
        </Card>
      )}

      {/* 订单状态分布 */}
      {orderOverview && orderOverview.totalOrders > 0 && (
        <Card className="mt-4 p-3!">
          <div className="flex items-center justify-between mb-2">
            <div className="flex items-center gap-2 text-sm text-gray-500">
              <Package size={14} />
              本月订单状态
            </div>
            <span className="text-xs text-gray-400">
              共{orderOverview.totalOrders}单
            </span>
          </div>
          <Chart option={statusPieOption} height={160} />
        </Card>
      )}

      {/* 客户贡献度 */}
      {customers.length > 0 && (
        <Card className="mt-4 p-3!">
          <div className="flex items-center gap-2 text-sm text-gray-500 mb-2">
            <Users size={14} />
            本月客户贡献
          </div>
          <Chart option={customerPieOption} height={160} />
        </Card>
      )}

      {/* 员工产量排行 */}
      {workers.length > 0 && (
        <Card className="mt-4 p-3!">
          <div className="flex items-center gap-2 text-sm text-gray-500 mb-2">
            <Users size={14} />
            本月员工产量
          </div>
          <div className="space-y-2">
            {workers.slice(0, 5).map((w, i) => (
              <div key={w.userId} className="flex items-center gap-2 text-sm">
                <span
                  className={`w-5 h-5 rounded-full flex items-center justify-center text-xs text-white ${
                    i === 0
                      ? "bg-yellow-500"
                      : i === 1
                      ? "bg-gray-400"
                      : i === 2
                      ? "bg-amber-600"
                      : "bg-gray-300"
                  }`}
                >
                  {i + 1}
                </span>
                <span className="flex-1 truncate">{w.userName}</span>
                <span className="text-gray-500">{w.totalQuantity}件</span>
                <span className="text-green-600 font-medium">
                  ¥{parseFloat(w.totalAmount).toFixed(0)}
                </span>
              </div>
            ))}
          </div>
        </Card>
      )}

      {/* 进行中订单进度 */}
      {orders.length > 0 && (
        <Card className="mt-4 p-3!">
          <div className="flex items-center justify-between mb-2">
            <div className="flex items-center gap-2 text-sm text-gray-500">
              <Package size={14} />
              订单进度
            </div>
            <span
              className="text-xs text-blue-500"
              onClick={() => navigate({ to: "/orders" })}
            >
              查看全部
            </span>
          </div>
          <div className="space-y-3">
            {orders.slice(0, 3).map((order) => (
              <div
                key={order.orderId}
                className="cursor-pointer"
                onClick={() =>
                  navigate({ to: "/orders/$id", params: { id: order.orderId } })
                }
              >
                <div className="flex items-center justify-between text-sm mb-1">
                  <span className="truncate flex-1">{order.productName}</span>
                  <span className="text-xs text-gray-400 ml-2">
                    {order.customerName}
                  </span>
                </div>
                <ProgressBar
                  percent={Math.min(order.progress * 100, 100)}
                  style={{
                    "--fill-color": order.progress >= 1 ? "#22c55e" : "#3b82f6",
                    "--track-color": "#e5e7eb",
                  }}
                />
                <div className="flex items-center justify-between text-xs text-gray-400 mt-0.5">
                  <span>
                    {order.completedQuantity}/{order.totalQuantity}件
                  </span>
                  <span>{(order.progress * 100).toFixed(0)}%</span>
                </div>
              </div>
            ))}
          </div>
        </Card>
      )}

      <div className="mt-4 grid grid-cols-2 gap-3">
        <Card
          className="cursor-pointer"
          onClick={() => navigate({ to: "/orders" })}
        >
          <div className="flex items-center gap-3 py-1">
            <FileEdit size={24} className="text-blue-500" />
            <div className="flex-1">
              <div className="font-medium">快捷记件</div>
              <div className="text-xs text-gray-400">
                进行中: {boss?.processingOrderCount ?? 0}
              </div>
            </div>
          </div>
        </Card>
        <Card
          className="cursor-pointer"
          onClick={() => navigate({ to: "/pending" })}
        >
          <div className="flex items-center gap-3 py-1">
            <Badge content={boss?.pendingCount || ""}>
              <Package size={24} className="text-orange-500" />
            </Badge>
            <div className="flex-1">
              <div className="font-medium">待审核</div>
              <div className="text-xs text-gray-400">需处理</div>
            </div>
          </div>
        </Card>
      </div>

      {activities?.list && activities.list.length > 0 && (
        <div className="mt-4">
          <div className="text-sm font-medium text-gray-600 mb-2">最近动态</div>
          <div className="space-y-2">
            {activities.list.slice(0, 5).map((act: Activity) => (
              <Card key={act.id} className="p-3!">
                <div className="flex items-start gap-3">
                  <div
                    className={`w-8 h-8 rounded-full flex items-center justify-center text-white text-xs font-medium ${
                      act.activityType === "submit"
                        ? "bg-orange-500"
                        : act.activityType === "approve"
                        ? "bg-green-500"
                        : "bg-red-500"
                    }`}
                  >
                    {act.userName.slice(0, 1)}
                  </div>
                  <div className="flex-1 min-w-0">
                    <div className="flex items-center gap-2">
                      <span className="font-medium text-sm">
                        {act.userName}
                      </span>
                      <span
                        className={`text-xs px-1.5 py-0.5 rounded ${
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
                    <div className="text-sm text-gray-600 mt-1">
                      {act.orderName} · {act.processName}
                      <span className="text-blue-600 ml-1 font-medium">
                        {act.quantity}件
                      </span>
                    </div>
                    <div className="text-xs text-gray-400 mt-1">
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
