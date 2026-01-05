import { ImageUploader as AdmImageUploader, Toast } from "antd-mobile";
import type { ImageUploadItem } from "antd-mobile/es/components/image-uploader";
import imageCompression from "browser-image-compression";
import { uploadApi } from "@/api";

interface ImageUploaderSingleProps {
  value?: string;
  onChange?: (url: string) => void;
  maxCount: 1;
}

interface ImageUploaderMultiProps {
  value?: string[];
  onChange?: (urls: string[]) => void;
  maxCount?: Exclude<number, 1>;
}

type ImageUploaderProps = ImageUploaderSingleProps | ImageUploaderMultiProps;

export function ImageUploader({
  value,
  onChange,
  maxCount = 9,
}: ImageUploaderProps) {
  const isSingle = maxCount === 1;
  const urls = Array.isArray(value) ? value : value ? [value] : [];
  const fileList: ImageUploadItem[] = urls.map((url) => ({ url }));

  const handleChange = (items: ImageUploadItem[]) => {
    const newUrls = items.map((i) => i.url!);
    if (isSingle) {
      (onChange as ((url: string) => void) | undefined)?.(newUrls[0] || "");
    } else {
      (onChange as ((urls: string[]) => void) | undefined)?.(newUrls);
    }
  };

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
      onChange={handleChange}
      upload={upload}
      maxCount={maxCount}
      onCountExceed={() => Toast.show(`最多上传 ${maxCount} 张图片`)}
    />
  );
}
