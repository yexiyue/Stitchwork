import { ImageUploader as AdmImageUploader, Toast } from "antd-mobile";
import type { ImageUploadItem } from "antd-mobile/es/components/image-uploader";
import imageCompression from "browser-image-compression";
import { uploadApi, getFileUrl } from "@/api";

interface ImageUploaderSingleProps {
  value?: string;
  onChange?: (key: string) => void;
  maxCount: 1;
}

interface ImageUploaderMultiProps {
  value?: string[];
  onChange?: (keys: string[]) => void;
  maxCount?: Exclude<number, 1>;
}

type ImageUploaderProps = ImageUploaderSingleProps | ImageUploaderMultiProps;

export function ImageUploader({
  value,
  onChange,
  maxCount = 9,
}: ImageUploaderProps) {
  const isSingle = maxCount === 1;
  const keys = Array.isArray(value) ? value : value ? [value] : [];
  const fileList: ImageUploadItem[] = keys.map((key) => ({ key, url: getFileUrl(key) }));

  const handleChange = (items: ImageUploadItem[]) => {
    const newKeys = items.map((i) => (i as { key?: string }).key || i.url!);
    if (isSingle) {
      (onChange as ((key: string) => void) | undefined)?.(newKeys[0] || "");
    } else {
      (onChange as ((keys: string[]) => void) | undefined)?.(newKeys);
    }
  };

  const upload = async (file: File): Promise<ImageUploadItem & { key: string }> => {
    const compressed = await imageCompression(file, {
      maxSizeMB: 0.5,
      maxWidthOrHeight: 1920,
      useWebWorker: true,
    });

    const { uploadUrl, key } = await uploadApi.presign(
      file.name,
      compressed.type || "image/jpeg"
    );

    await fetch(uploadUrl, {
      method: "PUT",
      body: compressed,
      headers: { "Content-Type": compressed.type || "image/jpeg" },
    });

    return { key, url: getFileUrl(key) };
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
