export interface Share {
  id: string;
  bossId: string;
  title: string;
  token: string;
  orderIds: string[];
  processIds: string[];
  isActive: boolean;
  createdAt: string;
  updatedAt: string;
}

export interface CreateShareRequest {
  title: string;
  orderIds: string[];
  processIds: string[];
}

export interface UpdateShareRequest {
  title?: string;
  orderIds?: string[];
  processIds?: string[];
  isActive?: boolean;
}

export interface PublicOrderInfo {
  id: string;
  productName: string;
  description?: string;
  images?: unknown;
  quantity: number;
}

export interface PublicProcessInfo {
  id: string;
  name: string;
  description?: string;
  piecePrice: string;
  orderProductName: string;
}

export interface PublicShareResponse {
  title: string;
  workshopName?: string;
  workshopDesc?: string;
  avatar?: string;
  orders: PublicOrderInfo[];
  processes: PublicProcessInfo[];
}
