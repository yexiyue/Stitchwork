import { useNavigate } from "@tanstack/react-router";
import { Card, Badge } from "antd-mobile";
import { ClipboardCheck, FileEdit, Users, Store, BarChart3 } from "lucide-react";
import { useQuery } from "@tanstack/react-query";
import { homeApi, statsApi } from "@/api";
import { useAuthStore } from "@/stores/auth";
import { MiniChart, RelativeTime } from "@/components";
import type { BossOverview, Activity } from "@/types";
import dayjs from "dayjs";

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

  const boss = overview as BossOverview | undefined;
  const chartData = dailyData?.list?.map((d) => d.totalQuantity) ?? [];

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
                <p className="text-sm text-gray-500 mt-1">{user.workshop.address}</p>
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
        <Card className="!p-3">
          <div className="text-sm text-gray-500">今日产量</div>
          <div className="text-xl font-bold text-blue-600 mt-1">{boss?.todayQuantity ?? 0}</div>
          <div className="text-xs text-gray-400">¥{parseFloat(boss?.todayAmount ?? "0").toFixed(2)}</div>
        </Card>
        <Card className="!p-3">
          <div className="text-sm text-gray-500">本月产量</div>
          <div className="text-xl font-bold text-green-600 mt-1">{boss?.monthQuantity ?? 0}</div>
          <div className="text-xs text-gray-400">¥{parseFloat(boss?.monthAmount ?? "0").toFixed(2)}</div>
        </Card>
      </div>

      {chartData.length > 0 && (
        <Card className="mt-4 !p-3">
          <div className="text-sm text-gray-500 mb-2">近7日趋势</div>
          <MiniChart data={chartData} color="#3b82f6" height={60} />
        </Card>
      )}

      <div className="mt-4 grid grid-cols-3 gap-3">
        <Card className="cursor-pointer" onClick={() => navigate({ to: "/pending" })}>
          <div className="flex flex-col items-center gap-2 py-2">
            <Badge content={boss?.pendingCount || ""}>
              <ClipboardCheck size={28} className="text-orange-500" />
            </Badge>
            <span className="text-sm">待审核</span>
          </div>
        </Card>
        <Card className="cursor-pointer" onClick={() => navigate({ to: "/customers" })}>
          <div className="flex flex-col items-center gap-2 py-2">
            <Users size={28} className="text-green-500" />
            <span className="text-sm">客户</span>
          </div>
        </Card>
        <Card className="cursor-pointer" onClick={() => navigate({ to: "/records/stats" })}>
          <div className="flex flex-col items-center gap-2 py-2">
            <BarChart3 size={28} className="text-purple-500" />
            <span className="text-sm">统计</span>
          </div>
        </Card>
      </div>

      <Card className="mt-4 cursor-pointer" onClick={() => navigate({ to: "/orders" })}>
        <div className="flex items-center gap-3 py-1">
          <FileEdit size={24} className="text-blue-500" />
          <div className="flex-1">
            <div className="font-medium">快捷记件</div>
            <div className="text-xs text-gray-400">进行中订单: {boss?.processingOrderCount ?? 0}</div>
          </div>
        </div>
      </Card>

      {activities?.list && activities.list.length > 0 && (
        <div className="mt-4">
          <div className="text-sm font-medium text-gray-600 mb-2">最近动态</div>
          <Card className="!p-0">
            {activities.list.slice(0, 5).map((act: Activity) => (
              <div key={act.id} className="px-3 py-2 border-b border-gray-50 last:border-0">
                <div className="flex items-center justify-between">
                  <span className="text-sm">
                    <span className="font-medium">{act.userName}</span>
                    <span className="text-gray-500 mx-1">
                      {act.activityType === "submit" ? "提交了" : act.activityType === "approve" ? "已审核" : "已驳回"}
                    </span>
                    <span className="text-blue-600">{act.quantity}件</span>
                  </span>
                  <RelativeTime date={act.createdAt} />
                </div>
                <div className="text-xs text-gray-400 mt-0.5">{act.orderName} · {act.processName}</div>
              </div>
            ))}
          </Card>
        </div>
      )}
    </div>
  );
}
