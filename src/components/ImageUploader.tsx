import { ImageUploader as AdmImageUploader, Toast } from "antd-mobile";
import type { ImageUploadItem } from "antd-mobile/es/components/image-uploader";
import imageCompression from "browser-image-compression";
import { uploadApi } from "@/api";

interface ImageUploaderProps {
  value?: string[];
  onChange?: (urls: string[]) => void;
  maxCount?: number;
}

export function ImageUploader({
  value = [],
  onChange,
  maxCount = 9,
}: ImageUploaderProps) {
  const fileList: ImageUploadItem[] = value.map((url) => ({ url }));

  const upload = async (file: File): Promise<ImageUploadItem> => {
    const compressed = await imageCompression(file, {
      maxSizeMB: 0.5,
      maxWidthOrHeight: 1920,
      useWebWorker: true,
    });

    const { uploadUrl, fileUrl } = await uploadApi.presign(
      file.name,
      compressed.type || "image/jpeg"
    );

    await fetch(uploadUrl, {
      method: "PUT",
      body: compressed,
      headers: { "Content-Type": compressed.type || "image/jpeg" },
    });

    return { url: fileUrl };
  };

  return (
    <AdmImageUploader
      value={fileList}
      onChange={(items) => onChange?.(items.map((i) => i.url!))}
      upload={upload}
      maxCount={maxCount}
      onCountExceed={() => Toast.show(`最多上传 ${maxCount} 张图片`)}
    />
  );
}
