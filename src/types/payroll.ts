export interface Payroll {
  id: string;
  userId: string;
  bossId: string;
  amount: string;
  paymentImage?: string;
  note?: string;
  paidAt: string;
}

export interface CreatePayrollDto {
  userId: string;
  amount: string;
  recordIds: string[];
  paymentImage?: string;
  note?: string;
}

export interface UpdatePayrollDto {
  amount?: string;
  paymentImage?: string;
  note?: string;
}
