import { ImageProps } from "antd-mobile";
import { getFileUrl } from "@/api";

interface OssImageProps extends Omit<ImageProps, "src"> {
  src?: string; // key
}

/** 支持 OSS key 的图片组件 */
export function OssImage({ src, ...props }: OssImageProps) {
  if (!src) return null;
  return <img src={getFileUrl(src)} {...props} />;
}

/** 批量转换 keys 为 URLs */
export function getOssUrls(keys: string[]): string[] {
  return keys.map(getFileUrl);
}

export { getFileUrl as getOssUrl };
