import { useState, useRef } from "react";
import { Button, Popup } from "antd-mobile";
import ReactCrop, { type Crop, centerCrop, makeAspectCrop } from "react-image-crop";
import "react-image-crop/dist/ReactCrop.css";

interface AvatarCropperProps {
  onConfirm: (blob: Blob) => Promise<void>;
}

export function useAvatarCropper({ onConfirm }: AvatarCropperProps) {
  const [cropSrc, setCropSrc] = useState<string | null>(null);
  const [crop, setCrop] = useState<Crop>();
  const [uploading, setUploading] = useState(false);
  const imgRef = useRef<HTMLImageElement>(null);
  const fileInputRef = useRef<HTMLInputElement>(null);

  const openFilePicker = () => fileInputRef.current?.click();

  const handleFileSelect = (e: React.ChangeEvent<HTMLInputElement>) => {
    const file = e.target.files?.[0];
    if (!file) return;
    const reader = new FileReader();
    reader.onload = () => setCropSrc(reader.result as string);
    reader.readAsDataURL(file);
    e.target.value = "";
  };

  const onImageLoad = (e: React.SyntheticEvent<HTMLImageElement>) => {
    const { width, height } = e.currentTarget;
    setCrop(centerCrop(makeAspectCrop({ unit: "%", width: 80 }, 1, width, height), width, height));
  };

  const handleCropConfirm = async () => {
    if (!imgRef.current || !crop) return;
    const canvas = document.createElement("canvas");
    const img = imgRef.current;
    const scaleX = img.naturalWidth / img.width;
    const scaleY = img.naturalHeight / img.height;

    // crop 可能是 % 单位（未拖动时），需转换为像素
    const cropX = (crop.unit === "%" ? (crop.x / 100) * img.width : crop.x) * scaleX;
    const cropY = (crop.unit === "%" ? (crop.y / 100) * img.height : crop.y) * scaleY;
    const cropW = (crop.unit === "%" ? (crop.width / 100) * img.width : crop.width) * scaleX;
    const cropH = (crop.unit === "%" ? (crop.height / 100) * img.height : crop.height) * scaleY;

    const size = Math.min(cropW, cropH);
    canvas.width = size;
    canvas.height = size;
    canvas.getContext("2d")!.drawImage(
      img, cropX, cropY, size, size, 0, 0, size, size
    );

    canvas.toBlob(async (blob) => {
      if (!blob) return;
      setUploading(true);
      try {
        await onConfirm(blob);
        setCropSrc(null);
      } finally {
        setUploading(false);
      }
    }, "image/jpeg", 0.9);
  };

  const FileInput = (
    <input ref={fileInputRef} type="file" accept="image/*" className="hidden" onChange={handleFileSelect} />
  );

  const CropPopup = (
    <Popup visible={!!cropSrc} onMaskClick={() => !uploading && setCropSrc(null)} bodyStyle={{ height: "80vh" }}>
      <div className="flex flex-col h-full">
        <div className="flex-1 overflow-auto p-4 flex items-center justify-center bg-black">
          {cropSrc && (
            <ReactCrop crop={crop} onChange={setCrop} aspect={1} circularCrop>
              <img ref={imgRef} src={cropSrc} onLoad={onImageLoad} className="max-h-full" />
            </ReactCrop>
          )}
        </div>
        <div className="p-4 flex gap-4">
          <Button block onClick={() => setCropSrc(null)} disabled={uploading}>取消</Button>
          <Button block color="primary" onClick={handleCropConfirm} loading={uploading}>确定</Button>
        </div>
      </div>
    </Popup>
  );

  return { openFilePicker, FileInput, CropPopup };
}
