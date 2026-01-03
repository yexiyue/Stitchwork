export interface PieceRecord {
  id: string;
  processId: string;
  userId: string;
  quantity: number;
  amount: string;
  createdAt: string;
  updatedAt: string;
}

export interface CreatePieceRecordDto {
  processId: string;
  userId: string;
  quantity: number;
}

export interface UpdatePieceRecordDto {
  quantity?: number;
}
