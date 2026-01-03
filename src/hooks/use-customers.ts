import { createCrudHooks } from "./create-crud-hooks";
import { customerApi } from "@/api";
import type { Customer, CreateCustomerDto, UpdateCustomerDto } from "@/types";

const hooks = createCrudHooks<Customer, CreateCustomerDto, UpdateCustomerDto>("customers", customerApi);

export const useCustomers = hooks.useList;
export const useCustomer = hooks.useOne;
export const useCreateCustomer = hooks.useCreate;
export const useUpdateCustomer = hooks.useUpdate;
export const useDeleteCustomer = hooks.useDelete;
