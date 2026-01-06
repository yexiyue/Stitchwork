export type PieceRecordStatus = "pending" | "approved" | "rejected" | "settled";
export type RecordedBy = "bySelf" | "byBoss";

export interface PieceRecord {
  id: string;
  processId: string;
  userId: string;
  bossId: string;
  quantity: number;
  amount: string;
  status: PieceRecordStatus;
  recordedBy: RecordedBy;
  recordedAt: string;
  // 关联字段
  processName?: string;
  userName?: string;
  orderName?: string;
  orderId?: string;
  orderImage?: string;
  piecePrice?: string;
}

export interface CreatePieceRecordDto {
  processId: string;
  userId: string;
  quantity: number;
}

export interface UpdatePieceRecordDto {
  quantity?: number;
}
