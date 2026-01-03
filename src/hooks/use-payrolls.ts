import { createCrudHooks } from "./create-crud-hooks";
import { payrollApi } from "@/api";
import type { Payroll, CreatePayrollDto, UpdatePayrollDto } from "@/types";

const hooks = createCrudHooks<Payroll, CreatePayrollDto, UpdatePayrollDto>("payrolls", payrollApi);

export const usePayrolls = hooks.useList;
export const usePayroll = hooks.useOne;
export const useCreatePayroll = hooks.useCreate;
export const useUpdatePayroll = hooks.useUpdate;
export const useDeletePayroll = hooks.useDelete;
