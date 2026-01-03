export interface Payroll {
  id: string;
  userId: string;
  amount: string;
  note?: string;
  createdAt: string;
  updatedAt: string;
}

export interface CreatePayrollDto {
  userId: string;
  amount: string;
  note?: string;
}

export interface UpdatePayrollDto {
  amount?: string;
  note?: string;
}
