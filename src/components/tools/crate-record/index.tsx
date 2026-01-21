import { makeAssistantTool } from "@assistant-ui/react";
import {
  CreateRecordSchema,
  CreateRecordSchemaType,
  CreateRecordResultType,
} from "./schema";
import { pieceRecordApi } from "@/api";
import { useAuthStore } from "@/stores/auth";

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

  render: ({ args, status, result, resume, interrupt }) => {
    console.log("CreateRecordTool", status, result, interrupt);
    const totalAmount = (
      parseFloat(args.piecePrice || "0") * (args.quantity || 0)
    ).toFixed(2);

    // 已完成
    if (result) {
      if (result.success) {
        return (
          <div className="rounded-lg border border-green-200 bg-green-50 p-4">
            <div className="flex items-center gap-2 text-green-700">
              <span className="text-lg">✅</span>
              <span className="font-medium">计件记录已提交</span>
            </div>
            <div className="mt-2 text-sm text-green-600">
              {args.orderName} · {args.processName} × {args.quantity} = ¥
              {totalAmount}
            </div>
            <div className="mt-1 text-xs text-green-500">待老板审核</div>
          </div>
        );
      } else {
        return (
          <div className="rounded-lg border border-red-200 bg-red-50 p-4">
            <div className="flex items-center gap-2 text-red-700">
              <span className="text-lg">❌</span>
              <span className="font-medium">创建失败</span>
            </div>
            <div className="mt-1 text-sm text-red-600">{result.error}</div>
          </div>
        );
      }
    }

    // 等待用户确认 (running: 执行到 human(), requires-action: human() 已触发)
    if (status.type === "running" || status.type === "requires-action") {
      return (
        <div className="rounded-lg border border-orange-200 bg-orange-50 p-4">
          <div className="mb-3 font-medium text-orange-800">
            确认创建计件记录
          </div>
          <div className="mb-4 space-y-1 text-sm text-orange-700">
            <div className="flex justify-between">
              <span>订单:</span>
              <span className="font-medium">{args.orderName}</span>
            </div>
            <div className="flex justify-between">
              <span>工序:</span>
              <span className="font-medium">{args.processName}</span>
            </div>
            <div className="flex justify-between">
              <span>单价:</span>
              <span>¥{args.piecePrice}</span>
            </div>
            <div className="flex justify-between">
              <span>数量:</span>
              <span>{args.quantity}</span>
            </div>
            <div className="flex justify-between border-t border-orange-200 pt-2 mt-2">
              <span>合计金额:</span>
              <span className="text-lg font-bold text-orange-600">
                ¥{totalAmount}
              </span>
            </div>
          </div>
          <div className="flex gap-2">
            <button
              className="flex-1 rounded-lg bg-orange-500 px-4 py-2 text-white hover:bg-orange-600 transition-colors"
              onClick={() => resume({ confirmed: true })}
            >
              确认提交
            </button>
            <button
              className="flex-1 rounded-lg border border-gray-300 px-4 py-2 hover:bg-gray-50 transition-colors"
              onClick={() => resume({ confirmed: false })}
            >
              取消
            </button>
          </div>
        </div>
      );
    }

    // 默认状态 (loading)
    return (
      <div className="rounded-lg border p-4">
        <div className="flex items-center gap-2 text-sm text-gray-500">
          <div className="h-4 w-4 animate-spin rounded-full border-2 border-gray-300 border-t-orange-500" />
          正在准备...
        </div>
      </div>
    );
  },
});
