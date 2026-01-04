export interface ApiResponse<T> {
  code: number;
  message: string;
  data: T;
}

export interface ListData<T> {
  list: T[];
  total: number;
}

export interface QueryParams {
  page?: number;
  pageSize?: number;
  sortBy?: string;
  sortOrder?: "asc" | "desc";
  search?: string;
  startDate?: string;
  endDate?: string;
  status?: string[];
  userId?: string;
  orderId?: string;
}
