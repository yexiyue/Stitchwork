import { Loader2 } from "lucide-react";
import { cn } from "@/lib/utils";

// 工具名称映射
const TOOL_NAMES: Record<string, string> = {
  // 员工端
  get_my_earnings: "收入统计",
  get_my_records: "计件记录",
  get_my_payrolls: "工资单",
  get_available_tasks: "可接工序",
  create_piece_record: "创建计件",
  // 老板端
  query_orders: "订单列表",
  query_piece_records: "计件记录",
  get_worker_stats: "员工产量",
  get_overview: "工坊概览",
  get_order_progress: "订单进度",
  get_unpaid_summary: "待发工资",
};

interface ToolLoadingProps {
  /** 工具名称（英文） */
  toolName: string;
  /** 自定义显示名称（可选，优先级高于映射） */
  label?: string;
  /** 额外的 className */
  className?: string;
}

/**
 * 统一的工具加载状态组件
 * 显示 loading 图标和工具中文名称
 */
export function ToolLoading({ toolName, label, className }: ToolLoadingProps) {
  const displayName = label || TOOL_NAMES[toolName] || toolName;

  return (
    <div
      className={cn(
        "flex items-center gap-2.5 py-3 px-4",
        "rounded-xl border border-border bg-card",
        "animate-in fade-in duration-200",
        className
      )}
    >
      <Loader2 className="w-4 h-4 text-muted-foreground animate-spin" />
      <span className="text-sm text-muted-foreground">
        正在获取<span className="font-medium text-foreground">{displayName}</span>...
      </span>
    </div>
  );
}

/**
 * 获取工具的中文名称
 */
export function getToolDisplayName(toolName: string): string {
  return TOOL_NAMES[toolName] || toolName;
}
