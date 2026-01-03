import { client } from "./client";
import type {
  Share,
  CreateShareRequest,
  UpdateShareRequest,
  PublicShareResponse,
} from "@/types";

export const shareApi = {
  list: () =>
    client.get<Share[]>("/api/shares"),
  create: (data: CreateShareRequest) =>
    client.post<Share>("/api/shares", data),
  update: (id: string, data: UpdateShareRequest) =>
    client.put<Share>(`/api/shares/${id}`, data),
  delete: (id: string) =>
    client.delete<void>(`/api/shares/${id}`),
  getPublic: (token: string) =>
    client.get<PublicShareResponse>(`/api/public/share/${token}`),
};
