import { z } from "zod";

export const CreateRecordSchema = z.object({
  orderId: z.string().describe("订单的唯一标识符 (UUID)"),
  orderName: z.string().describe("订单/产品名称"),
  amount: z.number().positive().describe("计件数量，必须大于0"),
});

export type CreateRecordSchemaType = z.infer<typeof CreateRecordSchema>;
