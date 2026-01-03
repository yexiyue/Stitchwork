export type PieceRecordStatus = "pending" | "approved" | "rejected";
export type RecordedBy = "self" | "boss";

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
}

export interface CreatePieceRecordDto {
  processId: string;
  userId: string;
  quantity: number;
}

export interface UpdatePieceRecordDto {
  quantity?: number;
}
