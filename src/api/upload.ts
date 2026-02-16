import { client } from "./client";

interface PresignResponse {
  uploadUrl: string | null;
  key: string;
  exists: boolean;
}

export const uploadApi = {
  presign: (filename: string, contentType: string, hash?: string) =>
    client.post<PresignResponse>("/api/upload/presign", { filename, contentType, hash }),
};

/** 根据 key 生成图片访问 URL（通过后端重定向） */
export function getFileUrl(key: string): string {
  if (!key) return "";
  return `${import.meta.env.VITE_API_URL}/api/files/${key}`;
}
