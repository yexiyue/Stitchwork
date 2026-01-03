import { client } from "./client";
import type {
  ListData,
  QueryParams,
  Process,
  CreateProcessDto,
  UpdateProcessDto,
  ProcessQueryParams,
} from "@/types";

export const processApi = {
  list: (params?: QueryParams & ProcessQueryParams) =>
    client.get<ListData<Process>>("/api/processes", params),
  getOne: (id: string) =>
    client.get<Process>(`/api/processes/${id}`),
  create: (data: CreateProcessDto) =>
    client.post<Process>("/api/processes", data),
  update: (id: string, data: UpdateProcessDto) =>
    client.put<Process>(`/api/processes/${id}`, data),
  delete: (id: string) =>
    client.delete<void>(`/api/processes/${id}`),
};
