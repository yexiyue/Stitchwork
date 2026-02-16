import { makeAssistantTool } from "@assistant-ui/react";
import {
  CreateRecordSchema,
  CreateRecordSchemaType,
  CreateRecordResultType,
} from "./schema";
import { pieceRecordApi } from "@/api";
import { useAuthStore } from "@/stores/auth";
import { cn } from "@/lib/utils";
import { CheckCircle2, XCircle, ClipboardList } from "lucide-react";

// 成功状态卡片
function SuccessCard({
  orderName,
  processName,
  quantity,
  totalAmount,
}: {
  orderName: string;
  processName: string;
  quantity: number;
  totalAmount: string;
}) {
  return (
    <div
      className={cn(
        "rounded-xl border border-emerald-200 dark:border-emerald-800/50",
        "bg-emerald-50 dark:bg-emerald-950/30 p-4",
        "animate-in fade-in slide-in-from-bottom-2 duration-300"
      )}
    >
      <div className="flex items-center gap-2.5 text-emerald-700 dark:text-emerald-400">
        <CheckCircle2 className="w-5 h-5 shrink-0" />
        <span className="font-medium">计件记录已提交</span>
      </div>
      <div className="mt-2.5 text-sm text-emerald-600 dark:text-emerald-500">
        <span className="font-medium">{orderName}</span>
        <span className="mx-1.5 text-emerald-400 dark:text-emerald-600">·</span>
        <span>{processName}</span>
        <span className="mx-1.5 text-emerald-400 dark:text-emerald-600">×</span>
        <span className="tabular-nums">{quantity}</span>
        <span className="mx-1.5 text-emerald-400 dark:text-emerald-600">=</span>
        <span className="font-semibold tabular-nums">¥{totalAmount}</span>
      </div>
      <div className="mt-1.5 text-xs text-emerald-500 dark:text-emerald-600">
        待老板审核
      </div>
    </div>
  );
}

// 错误状态卡片
function ErrorCard({ error }: { error: string }) {
  return (
    <div
      className={cn(
        "rounded-xl border border-red-200 dark:border-red-800/50",
        "bg-red-50 dark:bg-red-950/30 p-4",
        "animate-in fade-in slide-in-from-bottom-2 duration-300"
      )}
    >
      <div className="flex items-center gap-2.5 text-red-700 dark:text-red-400">
        <XCircle className="w-5 h-5 shrink-0" />
        <span className="font-medium">创建失败</span>
      </div>
      <div className="mt-1.5 text-sm text-red-600 dark:text-red-500">
        {error}
      </div>
    </div>
  );
}

// 确认卡片
function ConfirmCard({
  orderName,
  processName,
  piecePrice,
  quantity,
  totalAmount,
  onConfirm,
  onCancel,
}: {
  orderName: string;
  processName: string;
  piecePrice: string;
  quantity: number;
  totalAmount: string;
  onConfirm: () => void;
  onCancel: () => void;
}) {
  return (
    <div
      className={cn(
        "rounded-xl border border-amber-200 dark:border-amber-800/50",
        "bg-amber-50 dark:bg-amber-950/30 p-4",
        "animate-in fade-in slide-in-from-bottom-2 duration-300"
      )}
    >
      <div className="flex items-center gap-2.5 mb-3">
        <ClipboardList className="w-5 h-5 text-amber-600 dark:text-amber-500 shrink-0" />
        <span className="font-medium text-amber-800 dark:text-amber-300">
          确认创建计件记录
        </span>
      </div>

      <div className="mb-4 space-y-2 text-sm">
        <div className="flex justify-between items-center">
          <span className="text-amber-700/70 dark:text-amber-400/70">订单</span>
          <span className="font-medium text-amber-800 dark:text-amber-300 truncate ml-4 max-w-[60%] text-right">
            {orderName}
          </span>
        </div>
        <div className="flex justify-between items-center">
          <span className="text-amber-700/70 dark:text-amber-400/70">工序</span>
          <span className="font-medium text-amber-800 dark:text-amber-300">
            {processName}
          </span>
        </div>
        <div className="flex justify-between items-center">
          <span className="text-amber-700/70 dark:text-amber-400/70">单价</span>
          <span className="text-amber-800 dark:text-amber-300 tabular-nums">
            ¥{piecePrice}
          </span>
        </div>
        <div className="flex justify-between items-center">
          <span className="text-amber-700/70 dark:text-amber-400/70">数量</span>
          <span className="text-amber-800 dark:text-amber-300 tabular-nums">
            {quantity}
          </span>
        </div>
        <div className="flex justify-between items-center pt-2.5 mt-1 border-t border-amber-200 dark:border-amber-800/50">
          <span className="text-amber-700/70 dark:text-amber-400/70">
            合计金额
          </span>
          <span className="text-lg font-bold text-amber-600 dark:text-amber-400 tabular-nums">
            ¥{totalAmount}
          </span>
        </div>
      </div>

      <div className="flex gap-2.5">
        <button
          className={cn(
            "flex-1 rounded-lg px-4 py-2.5 text-sm font-medium",
            "bg-amber-500 text-white",
            "hover:bg-amber-600 active:bg-amber-700",
            "transition-colors duration-150",
            "focus:outline-none focus:ring-2 focus:ring-amber-500 focus:ring-offset-2 dark:focus:ring-offset-amber-950"
          )}
          onClick={onConfirm}
        >
          确认提交
        </button>
        <button
          className={cn(
            "flex-1 rounded-lg px-4 py-2.5 text-sm font-medium",
            "border border-amber-300 dark:border-amber-700",
            "text-amber-700 dark:text-amber-400",
            "hover:bg-amber-100 dark:hover:bg-amber-900/30 active:bg-amber-200 dark:active:bg-amber-900/50",
            "transition-colors duration-150",
            "focus:outline-none focus:ring-2 focus:ring-amber-500 focus:ring-offset-2 dark:focus:ring-offset-amber-950"
          )}
          onClick={onCancel}
        >
          取消
        </button>
      </div>
    </div>
  );
}

// 加载状态卡片
function LoadingCard() {
  return (
    <div className="rounded-xl border border-border bg-card p-4">
      <div className="flex items-center gap-2.5 text-sm text-muted-foreground">
        <div className="h-4 w-4 animate-spin rounded-full border-2 border-muted border-t-amber-500" />
        <span>正在准备...</span>
      </div>
    </div>
  );
}

export const CreateRecordTool = makeAssistantTool<
  CreateRecordSchemaType,
  CreateRecordResultType
>({
  toolName: "create_piece_record",
  type: "frontend",
  parameters: CreateRecordSchema,
  description:
    "创建计件记录。员工完成工序后，使用此工具录入计件数量。需要提供工序ID、工序名称、订单名称、单价和数量。",
  execute: async (args, ctx) => {
    const { human } = ctx;
    const user = useAuthStore.getState().user;

    if (!user?.id) {
      return { success: false, error: "用户未登录" };
    }

    // 等待用户确认
    const response = (await human({
      type: "confirm",
      processName: args.processName,
      orderName: args.orderName,
      piecePrice: args.piecePrice,
      quantity: args.quantity,
      totalAmount: (parseFloat(args.piecePrice) * args.quantity).toFixed(2),
    })) as { confirmed: boolean };

    if (!response.confirmed) {
      return { success: false, error: "用户取消了操作" };
    }

    // 调用 API 创建记录
    try {
      const record = await pieceRecordApi.create({
        processId: args.processId,
        userId: user.id,
        quantity: args.quantity,
      });
      return { success: true, record };
    } catch (error) {
      const message =
        error instanceof Error ? error.message : "创建计件记录失败";
      return { success: false, error: message };
    }
  },

  render: ({ args, status, result, resume }) => {
    const totalAmount = (
      parseFloat(args.piecePrice || "0") * (args.quantity || 0)
    ).toFixed(2);

    // 已完成
    if (result) {
      if (result.success) {
        return (
          <SuccessCard
            orderName={args.orderName}
            processName={args.processName}
            quantity={args.quantity}
            totalAmount={totalAmount}
          />
        );
      } else {
        return <ErrorCard error={result.error || "未知错误"} />;
      }
    }

    // 等待用户确认 (running: 执行到 human(), requires-action: human() 已触发)
    if (status.type === "running" || status.type === "requires-action") {
      return (
        <ConfirmCard
          orderName={args.orderName}
          processName={args.processName}
          piecePrice={args.piecePrice}
          quantity={args.quantity}
          totalAmount={totalAmount}
          onConfirm={() => resume({ confirmed: true })}
          onCancel={() => resume({ confirmed: false })}
        />
      );
    }

    // 默认状态 (loading)
    return <LoadingCard />;
  },
});
