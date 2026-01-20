import { makeAssistantTool } from "@assistant-ui/react";
import { CreateRecordSchema, CreateRecordSchemaType } from "./schema";

export const CreateRecordTool = makeAssistantTool<
  CreateRecordSchemaType,
  { success: boolean }
>({
  toolName: "create-record",
  type: "frontend",
  parameters: CreateRecordSchema,
  description: "创建计件记录",
  execute: async (args, ctx) => {
    const { human } = ctx;
    // 模拟创建计件记录
    console.log("args----ctx", args, ctx);
    const res = await human("请确认创建计件记录");
    console.log("res", res);
    return { success: true };
  },

  render: (options) => {
    const { args, status, result, resume } = options;
    console.log("CreateRecordTool", options);
    // 等待用户确认
    if (status.type === "requires-action" || status.type === "running") {
      return (
        <div className="rounded-lg border p-4">
          <div className="mb-2 font-medium">确认创建计件记录</div>
          <div className="mb-4 text-sm text-muted-foreground">
            订单: {args.orderName}
            <br />
            数量: {args.amount}
          </div>
          <div className="flex gap-2">
            <button
              className="rounded bg-primary px-4 py-2 text-primary-foreground"
              onClick={() => {
                resume("666");
              }}
            >
              确认
            </button>
            <button
              className="rounded border px-4 py-2"
              onClick={() => {
                resume("666");
              }}
            >
              取消
            </button>
          </div>
        </div>
      );
    }

    // 已完成
    return (
      <div className="rounded-lg border p-4">
        <div className="text-sm text-muted-foreground">
          {result?.success ? "✅" : "❌"} 计件记录: {args.orderName} x{" "}
          {args.amount}
        </div>
      </div>
    );
  },
});
