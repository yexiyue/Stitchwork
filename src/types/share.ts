export interface Share {
  id: string;
  bossId: string;
  title: string;
  description?: string;
  token: string;
  orderIds: string[];
  processIds: string[];
  isActive: boolean;
  createdAt: string;
  updatedAt: string;
}

export interface CreateShareRequest {
  title: string;
  description?: string;
  orderIds: string[];
  processIds: string[];
}

export interface UpdateShareRequest {
  title?: string;
  description?: string;
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
  orderImages: string[];
  remainingQuantity: number;
}

export interface PublicShareResponse {
  title: string;
  description?: string;
  workshopName?: string;
  workshopDesc?: string;
  workshopAddress?: string;
  workshopImage?: string;
  pieceUnit: string;
  bossPhone?: string;
  avatar?: string;
  processes: PublicProcessInfo[];
}
