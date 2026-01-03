export type OrderStatus = "pending" | "processing" | "completed" | "delivered" | "cancelled";

export interface Order {
  id: string;
  customerId: string;
  productName: string;
  description?: string;
  images?: string[];
  quantity: number;
  unitPrice: string;
  status: OrderStatus;
  createdAt: string;
  updatedAt: string;
}

export interface CreateOrderDto {
  customerId: string;
  productName: string;
  description?: string;
  images?: string[];
  quantity: number;
  unitPrice: string;
}

export interface UpdateOrderDto {
  productName?: string;
  description?: string;
  images?: string[];
  quantity?: number;
  unitPrice?: string;
  status?: string;
}

export interface UpdateOrderStatusDto {
  status: string;
}

export interface OrderQueryParams {
  customerId?: string;
  status?: string;
}
