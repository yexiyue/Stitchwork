export interface Customer {
  id: string;
  bossId: string;
  name: string;
  phone?: string;
  description?: string;
  createdAt: string;
  updatedAt: string;
}

export interface CreateCustomerDto {
  name: string;
  phone?: string;
  description?: string;
}

export interface UpdateCustomerDto {
  name?: string;
  phone?: string;
  description?: string;
}
