import { makeAssistantToolUI } from "@assistant-ui/react";
import { cn } from "@/lib/utils";
import { ClipboardList } from "lucide-react";
import { ToolLoading } from "../shared";
import type { Output, OrderProgressList, OrderProgressItem } from "../types";

// 进度条组件
function ProgressBar({
  progress,
  className,
}: {
  progress: number;
  className?: string;
}) {
  const clampedProgress = Math.min(100, Math.max(0, progress));
  const isComplete = clampedProgress >= 100;
  const isLow = clampedProgress < 30;
  const isMedium = clampedProgress >= 30 && clampedProgress < 70;

  return (
    <div
      className={cn(
        "w-full h-1.5 bg-muted rounded-full overflow-hidden",
        className
      )}
    >
      <div
        className={cn(
          "h-full transition-all duration-500 ease-out rounded-full",
          isComplete && "bg-emerald-500",
          isMedium && "bg-sky-500",
          isLow && "bg-amber-500"
        )}
        style={{ width: `${clampedProgress}%` }}
      />
    </div>
  );
}

// 订单进度卡片
function OrderProgressCard({
  item,
  index,
}: {
  item: OrderProgressItem;
  index: number;
}) {
  const statusMap: Record<
    string,
    { label: string; color: string; dotColor: string }
  > = {
    pending: {
      label: "待处理",
      color:
        "text-amber-700 dark:text-amber-400 bg-amber-50 dark:bg-amber-950/50",
      dotColor: "bg-amber-500",
    },
    processing: {
      label: "进行中",
      color: "text-sky-700 dark:text-sky-400 bg-sky-50 dark:bg-sky-950/50",
      dotColor: "bg-sky-500",
    },
    completed: {
      label: "已完成",
      color:
        "text-emerald-700 dark:text-emerald-400 bg-emerald-50 dark:bg-emerald-950/50",
      dotColor: "bg-emerald-500",
    },
    delivered: {
      label: "已出货",
      color:
        "text-emerald-700 dark:text-emerald-400 bg-emerald-50 dark:bg-emerald-950/50",
      dotColor: "bg-emerald-500",
    },
    cancelled: {
      label: "已取消",
      color: "text-red-700 dark:text-red-400 bg-red-50 dark:bg-red-950/50",
      dotColor: "bg-red-500",
    },
  };

  const statusInfo = statusMap[item.status] || {
    label: item.status,
    color: "text-muted-foreground bg-muted",
    dotColor: "bg-muted-foreground",
  };

  const baseDelay = index * 100;

  return (
    <div
      className={cn(
        "group p-3.5 border rounded-xl bg-card cursor-pointer",
        "hover:border-border/80 hover:shadow-sm",
        "transition-all duration-200 ease-out",
        "animate-in fade-in slide-in-from-bottom-2 fill-mode-both"
      )}
      style={{ animationDelay: `${baseDelay}ms` }}
    >
      <div className="flex items-start justify-between gap-2 mb-2.5">
        <span className="font-medium text-sm leading-tight truncate flex-1">
          {item.productName}
        </span>
        <span
          className={cn(
            "inline-flex items-center gap-1.5 text-[11px] font-medium px-2 py-0.5 rounded-full shrink-0",
            statusInfo.color
          )}
        >
          <span
            className={cn("w-1.5 h-1.5 rounded-full", statusInfo.dotColor)}
          />
          {statusInfo.label}
        </span>
      </div>
      <div className="text-xs text-muted-foreground mb-3">
        {item.customerName}
      </div>
      <ProgressBar progress={item.progress} />
      <div className="flex items-center justify-between mt-2.5 text-xs">
        <span className="text-muted-foreground tabular-nums">
          {item.completedQuantity.toLocaleString()} /{" "}
          {item.totalQuantity.toLocaleString()} 件
        </span>
        <span className="font-semibold tabular-nums">
          {item.progress.toFixed(1)}%
        </span>
      </div>
    </div>
  );
}

// 空状态组件
function EmptyState() {
  return (
    <div className="flex flex-col items-center justify-center py-8 text-center">
      <div className="w-12 h-12 mb-3 rounded-full bg-muted/50 flex items-center justify-center">
        <ClipboardList className="w-6 h-6 text-muted-foreground/60" />
      </div>
      <p className="text-sm text-muted-foreground">暂无进行中的订单</p>
    </div>
  );
}

// 订单进度列表 UI
export const OrderProgressToolUi = makeAssistantToolUI<unknown, Output>({
  toolName: "get_order_progress",
  render: ({ status, result }) => {
    // 加载状态
    if (status?.type === "running") {
      return <ToolLoading toolName="get_order_progress" />;
    }

    // 无结果则不渲染
    if (!result || result.length === 0) return null;

    // 解析 JSON 数据
    const data: OrderProgressList = JSON.parse(result[0].text);

    if (data.list.length === 0) {
      return <EmptyState />;
    }

    return (
      <div className="space-y-2.5 max-h-100 overflow-y-auto pr-1 -mr-1">
        {data.list.map((item, index) => (
          <OrderProgressCard key={item.orderId} item={item} index={index} />
        ))}
      </div>
    );
  },
});
