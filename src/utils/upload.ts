import imageCompression from "browser-image-compression";
import { uploadApi } from "@/api";

/** 上传图片，返回 key */
export async function uploadImage(
  file: File | Blob,
  options?: { maxSizeMB?: number; maxWidthOrHeight?: number }
): Promise<string> {
  const { maxSizeMB = 0.5, maxWidthOrHeight = 1920 } = options || {};

  const compressed = await imageCompression(
    file instanceof File ? file : new File([file], "image.jpg", { type: "image/jpeg" }),
    { maxSizeMB, maxWidthOrHeight, useWebWorker: true }
  );

  const { uploadUrl, key } = await uploadApi.presign(
    "image.jpg",
    compressed.type || "image/jpeg"
  );

  await fetch(uploadUrl, {
    method: "PUT",
    body: compressed,
    headers: { "Content-Type": compressed.type || "image/jpeg" },
  });

  return key;
}
