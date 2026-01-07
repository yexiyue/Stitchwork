import imageCompression from "browser-image-compression";
import { invoke } from "@tauri-apps/api/core";
import { uploadApi } from "@/api";

const API_URL = import.meta.env.VITE_API_URL || "http://localhost:3000";

/** Check if running inside Tauri */
function isTauri(): boolean {
  return typeof window !== "undefined" && "__TAURI__" in window;
}

function getToken(): string {
  return localStorage.getItem("token") || "";
}

/** Upload result from Tauri */
interface UploadResult {
  key: string;
  exists: boolean;
  size: number;
}

interface UploadOptions {
  maxSizeMB?: number;
  maxWidthOrHeight?: number;
  quality?: number;
}

/** 上传图片，返回 key */
export async function uploadImage(
  file: File | Blob,
  options?: UploadOptions
): Promise<string> {
  const { maxWidthOrHeight = 2560, quality = 88 } = options || {};

  if (isTauri()) {
    // Tauri: compress + hash + upload all in Rust
    const arrayBuffer = await file.arrayBuffer();
    const imageData = Array.from(new Uint8Array(arrayBuffer));

    const result = await invoke<UploadResult>("upload_image", {
      imageData,
      apiUrl: API_URL,
      token: getToken(),
      options: {
        maxDimension: maxWidthOrHeight,
        quality,
      },
    });

    if (result.exists) {
      console.log("Instant upload: file already exists", result.key);
    }

    return result.key;
  } else {
    // Browser fallback: JS-side compression
    return uploadImageBrowser(file, options);
  }
}

async function uploadImageBrowser(
  file: File | Blob,
  options?: UploadOptions
): Promise<string> {
  const { maxSizeMB = 0.5, maxWidthOrHeight = 1920 } = options || {};

  const compressed = await imageCompression(
    file instanceof File
      ? file
      : new File([file], "image.jpg", { type: "image/jpeg" }),
    { maxSizeMB, maxWidthOrHeight, useWebWorker: true }
  );

  const { uploadUrl, key } = await uploadApi.presign(
    "image.jpg",
    compressed.type || "image/jpeg"
  );

  await fetch(uploadUrl!, {
    method: "PUT",
    body: compressed,
    headers: { "Content-Type": compressed.type || "image/jpeg" },
  });

  return key;
}
