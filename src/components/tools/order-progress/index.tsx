import { makeAssistantToolUI } from "@assistant-ui/react";
import type { Output, OrderProgressList, OrderProgressItem } from "../types";

// 进度条组件
function ProgressBar({ progress }: { progress: number }) {
  const clampedProgress = Math.min(100, Math.max(0, progress));
  const isComplete = clampedProgress >= 100;

  return (
    <div className="w-full h-2 bg-muted rounded-full overflow-hidden">
      <div
        className={`h-full transition-all duration-300 ${
          isComplete ? "bg-green-500" : "bg-blue-500"
        }`}
        style={{ width: `${clampedProgress}%` }}
      />
    </div>
  );
}

// 订单进度卡片
function OrderProgressCard({ item }: { item: OrderProgressItem }) {
  const statusMap: Record<string, { label: string; color: string }> = {
    pending: { label: "待处理", color: "text-yellow-600 bg-yellow-50" },
    processing: { label: "进行中", color: "text-blue-600 bg-blue-50" },
    completed: { label: "已完成", color: "text-green-600 bg-green-50" },
    delivered: { label: "已出货", color: "text-green-600 bg-green-50" },
    cancelled: { label: "已取消", color: "text-red-600 bg-red-50" },
  };

  const statusInfo = statusMap[item.status] || {
    label: item.status,
    color: "text-gray-600 bg-gray-50",
  };

  return (
    <div className="p-3 border rounded-lg bg-card">
      <div className="flex items-center justify-between mb-2">
        <span className="font-medium text-sm truncate flex-1 mr-2">
          {item.productName}
        </span>
        <span
          className={`text-xs px-2 py-0.5 rounded-full ${statusInfo.color}`}
        >
          {statusInfo.label}
        </span>
      </div>
      <div className="text-xs text-muted-foreground mb-2">
        {item.customerName}
      </div>
      <ProgressBar progress={item.progress} />
      <div className="flex items-center justify-between mt-2 text-xs">
        <span className="text-muted-foreground">
          {item.completedQuantity} / {item.totalQuantity} 件
        </span>
        <span className="font-medium">{item.progress.toFixed(1)}%</span>
      </div>
    </div>
  );
}

// 加载骨架屏
function OrderProgressLoading() {
  return (
    <div className="space-y-3 animate-pulse">
      {[1, 2, 3].map((i) => (
        <div key={i} className="p-3 border rounded-lg bg-card">
          <div className="flex items-center justify-between mb-2">
            <div className="h-4 w-24 bg-muted rounded" />
            <div className="h-5 w-12 bg-muted rounded-full" />
          </div>
          <div className="h-3 w-16 bg-muted rounded mb-2" />
          <div className="h-2 w-full bg-muted rounded-full" />
          <div className="flex items-center justify-between mt-2">
            <div className="h-3 w-16 bg-muted rounded" />
            <div className="h-3 w-10 bg-muted rounded" />
          </div>
        </div>
      ))}
    </div>
  );
}

// 订单进度列表 UI
export const OrderProgressToolUi = makeAssistantToolUI<unknown, Output>({
  toolName: "get_order_progress",
  render: ({ status, result }) => {
    // 加载状态
    if (status?.type === "running") {
      return <OrderProgressLoading />;
    }

    // 无结果则不渲染
    if (!result || result.length === 0) return null;

    // 解析 JSON 数据
    const data: OrderProgressList = JSON.parse(result[0].text);

    if (data.list.length === 0) {
      return (
        <div className="p-4 text-center text-muted-foreground text-sm">
          暂无进行中的订单
        </div>
      );
    }

    return (
      <div className="space-y-3 max-h-[400px] overflow-y-auto">
        {data.list.map((item) => (
          <OrderProgressCard key={item.orderId} item={item} />
        ))}
      </div>
    );
  },
});
