export interface Payroll {
  id: string;
  userId: string;
  bossId: string;
  amount: string;
  paymentImage?: string;
  note?: string;
  paidAt: string;
}

export interface PayrollRecord {
  id: string;
  quantity: number;
  amount: string;
  recordedAt: string;
  processName?: string;
  orderName?: string;
  orderImages?: string[];
  piecePrice?: string;
}

export interface PayrollDetail extends Payroll {
  records: PayrollRecord[];
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
