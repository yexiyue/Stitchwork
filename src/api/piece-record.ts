import { client } from "./client";
import type {
  ListData,
  QueryParams,
  PieceRecord,
  CreatePieceRecordDto,
  UpdatePieceRecordDto,
} from "@/types";

export const pieceRecordApi = {
  list: (params?: QueryParams) =>
    client.get<ListData<PieceRecord>>("/api/piece-records", params),
  getOne: (id: string) =>
    client.get<PieceRecord>(`/api/piece-records/${id}`),
  create: (data: CreatePieceRecordDto) =>
    client.post<PieceRecord>("/api/piece-records", data),
  update: (id: string, data: UpdatePieceRecordDto) =>
    client.put<PieceRecord>(`/api/piece-records/${id}`, data),
  delete: (id: string) =>
    client.delete<void>(`/api/piece-records/${id}`),
  approve: (id: string) =>
    client.post<PieceRecord>(`/api/piece-records/${id}/approve`),
  reject: (id: string) =>
    client.post<PieceRecord>(`/api/piece-records/${id}/reject`),
};
