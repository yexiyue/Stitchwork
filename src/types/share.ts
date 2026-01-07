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

export interface PublicProcessInfo {
  id: string;
  name: string;
  description?: string;
  piecePrice: string;
  orderProductName: string;
  remainingQuantity: number;
}

export interface PublicShareResponse {
  title: string;
  workshopName?: string;
  workshopAddress?: string;
  bossPhone?: string;
  avatar?: string;
  processes: PublicProcessInfo[];
}
