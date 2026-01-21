import { z } from "zod";

export const CreateRecordSchema = z.object({
  processId: z
    .string()
    .describe("工序ID (UUID)，必须从 get_available_tasks 返回的 processId 字段获取"),
  processName: z
    .string()
    .describe("工序名称，必须从 get_available_tasks 返回的 processName 字段获取"),
  orderName: z
    .string()
    .describe("订单名称，必须从 get_available_tasks 返回的 orderName 字段获取"),
  piecePrice: z
    .string()
    .describe("单价，必须从 get_available_tasks 返回的 piecePrice 字段获取"),
  quantity: z
    .number()
    .positive()
    .describe("计件数量，用户指定的数量，必须大于0且不超过 remainingQuantity"),
});

export type CreateRecordSchemaType = z.infer<typeof CreateRecordSchema>;

/** 创建计件记录的返回结果 */
export const CreateRecordResultSchema = z.discriminatedUnion("success", [
  z.object({
    success: z.literal(true).describe("操作是否成功"),
    record: z
      .object({
        id: z.string().describe("新创建的计件记录ID"),
        processId: z.string().describe("工序ID"),
        userId: z.string().describe("员工ID"),
        quantity: z.number().describe("计件数量"),
        amount: z.string().describe("计件金额"),
        status: z.string().describe("记录状态，新创建的记录为 pending（待审批）"),
      })
      .describe("新创建的计件记录详情"),
  }),
  z.object({
    success: z.literal(false).describe("操作是否成功"),
    error: z.string().describe("失败原因，如：用户取消、数量超限、网络错误等"),
  }),
]);

export type CreateRecordResultType = z.infer<typeof CreateRecordResultSchema>;
