export interface Process {
  id: string;
  orderId: string;
  name: string;
  description?: string;
  piecePrice: string;
  createdAt: string;
  updatedAt: string;
}

export interface CreateProcessDto {
  orderId: string;
  name: string;
  description?: string;
  piecePrice: string;
}

export interface UpdateProcessDto {
  name?: string;
  description?: string;
  piecePrice?: string;
}

export interface ProcessQueryParams {
  orderId?: string;
}
