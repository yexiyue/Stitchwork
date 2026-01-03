import { client } from "./client";

interface PresignResponse {
  uploadUrl: string;
  fileUrl: string;
}

export const uploadApi = {
  presign: (filename: string, contentType: string) =>
    client.post<PresignResponse>("/api/upload/presign", { filename, contentType }),
};
