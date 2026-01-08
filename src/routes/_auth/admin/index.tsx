import { createFileRoute, useNavigate } from "@tanstack/react-router";
import { List, Card, SpinLoading } from "antd-mobile";
import { Key, Users, Building2, FileCheck } from "lucide-react";
import { useQuery } from "@tanstack/react-query";
import { adminApi } from "@/api";
import { Chart } from "@/components/charts";
import type { EChartsOption } from "echarts";

export const Route = createFileRoute("/_auth/admin/")({
  component: AdminIndexPage,
});

function AdminIndexPage() {
  const navigate = useNavigate();

  const { data: stats, isLoading } = useQuery({
    queryKey: ["admin", "stats"],
    queryFn: adminApi.getStats,
  });

  // 注册码使用情况饼图
  const codeChartOption: EChartsOption = {
    tooltip: { trigger: "item" },
    legend: {
      orient: "horizontal",
      bottom: 0,
      itemWidth: 10,
      itemHeight: 10,
      textStyle: { fontSize: 12 },
    },
    series: [
      {
        type: "pie",
        radius: ["40%", "70%"],
        center: ["50%", "40%"],
        avoidLabelOverlap: false,
        itemStyle: { borderRadius: 4 },
        label: { show: false },
        data: stats
          ? [
              { value: stats.usedCodes, name: "已使用", itemStyle: { color: "#94a3b8" } },
              { value: stats.availableCodes, name: "可用", itemStyle: { color: "#22c55e" } },
              { value: stats.disabledCodes, name: "已禁用", itemStyle: { color: "#ef4444" } },
            ]
          : [],
      },
    ],
  };

  if (isLoading) {
    return (
      <div className="flex items-center justify-center h-full">
        <SpinLoading color="primary" />
      </div>
    );
  }

  return (
    <div className="flex flex-col h-full overflow-hidden">
      <div className="p-4 pb-2">
        <h1 className="text-xl font-bold">管理后台</h1>
      </div>

      <div className="flex-1 overflow-auto px-4 pb-4">
        {/* 统计卡片 */}
        <div className="grid grid-cols-2 gap-3 mb-4">
          <Card className="rounded-xl!">
            <div className="flex items-center gap-3">
              <div className="w-10 h-10 rounded-full bg-blue-100 flex items-center justify-center">
                <Users size={20} className="text-blue-500" />
              </div>
              <div>
                <div className="text-2xl font-bold">{stats?.totalUsers ?? 0}</div>
                <div className="text-xs text-gray-500">总用户数</div>
              </div>
            </div>
            <div className="mt-2 text-xs text-gray-400">
              老板 {stats?.bossCount ?? 0} / 员工 {stats?.staffCount ?? 0}
            </div>
          </Card>

          <Card className="rounded-xl!">
            <div className="flex items-center gap-3">
              <div className="w-10 h-10 rounded-full bg-green-100 flex items-center justify-center">
                <Building2 size={20} className="text-green-500" />
              </div>
              <div>
                <div className="text-2xl font-bold">{stats?.totalWorkshops ?? 0}</div>
                <div className="text-xs text-gray-500">工坊数</div>
              </div>
            </div>
            <div className="mt-2 text-xs text-gray-400">
              活跃 {stats?.activeWorkshops ?? 0}
            </div>
          </Card>

          <Card className="rounded-xl!">
            <div className="flex items-center gap-3">
              <div className="w-10 h-10 rounded-full bg-purple-100 flex items-center justify-center">
                <Key size={20} className="text-purple-500" />
              </div>
              <div>
                <div className="text-2xl font-bold">{stats?.totalCodes ?? 0}</div>
                <div className="text-xs text-gray-500">注册码</div>
              </div>
            </div>
            <div className="mt-2 text-xs text-gray-400">
              可用 {stats?.availableCodes ?? 0}
            </div>
          </Card>

          <Card className="rounded-xl!">
            <div className="flex items-center gap-3">
              <div className="w-10 h-10 rounded-full bg-orange-100 flex items-center justify-center">
                <FileCheck size={20} className="text-orange-500" />
              </div>
              <div>
                <div className="text-2xl font-bold">{stats?.monthOrders ?? 0}</div>
                <div className="text-xs text-gray-500">本月订单</div>
              </div>
            </div>
            <div className="mt-2 text-xs text-gray-400">
              今日 {stats?.todayOrders ?? 0}
            </div>
          </Card>
        </div>

        {/* 新增用户统计 */}
        <Card className="rounded-xl! mb-4">
          <div className="text-sm font-medium mb-3">新增用户</div>
          <div className="grid grid-cols-3 gap-4 text-center">
            <div>
              <div className="text-xl font-bold text-blue-500">
                {stats?.todayNewUsers ?? 0}
              </div>
              <div className="text-xs text-gray-500">今日</div>
            </div>
            <div>
              <div className="text-xl font-bold text-blue-500">
                {stats?.weekNewUsers ?? 0}
              </div>
              <div className="text-xs text-gray-500">本周</div>
            </div>
            <div>
              <div className="text-xl font-bold text-blue-500">
                {stats?.monthNewUsers ?? 0}
              </div>
              <div className="text-xs text-gray-500">本月</div>
            </div>
          </div>
        </Card>

        {/* 注册码使用情况图表 */}
        <Card className="rounded-xl! mb-4">
          <div className="text-sm font-medium mb-2">注册码使用情况</div>
          <Chart option={codeChartOption} height={180} />
        </Card>

        {/* 快捷入口 */}
        <Card className="rounded-xl!">
          <div className="text-sm font-medium mb-2">快捷入口</div>
          <List className="-mx-3 -mb-3">
            <List.Item
              prefix={<Key size={18} className="text-purple-500" />}
              onClick={() => navigate({ to: "/admin/register-codes" })}
              arrow
            >
              注册码管理
            </List.Item>
            <List.Item
              prefix={<Users size={18} className="text-blue-500" />}
              onClick={() => navigate({ to: "/admin/users" })}
              arrow
            >
              用户管理
            </List.Item>
          </List>
        </Card>
      </div>
    </div>
  );
}
