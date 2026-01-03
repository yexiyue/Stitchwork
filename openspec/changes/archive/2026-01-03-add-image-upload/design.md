## Context

移动端服装加工管理系统需要图片上传能力，用于订单产品图、用户头像等场景。

## Goals / Non-Goals

Goals:

- 提供可复用的图片上传能力
- 使用 AWS S3 兼容存储
- 支持移动端拍照和相册选择
- 图片压缩以节省流量

Non-Goals:

- 视频上传
- 图片编辑功能
- CDN 加速（后续优化）

## Decisions

### 存储方案: AWS S3 兼容存储

- 原因: S3 API 是行业标准，阿里云 OSS 也兼容 S3 协议
- 使用 `aws-sdk-s3` Rust SDK，可对接 AWS S3 或 OSS S3 兼容端点

### 上传流程: Presigned URL 直传

- 流程: 前端请求 → 后端生成 Presigned PUT URL → 前端直接 PUT 上传
- 优点: 前端无需 AWS SDK，简单的 fetch PUT 即可
- Presigned URL 有效期: 15 分钟

### 图片处理: 前端压缩

- 使用 browser-image-compression 库
- 目标: 压缩到 500KB 以下

## Risks / Trade-offs

- Presigned URL 泄露风险 → 短有效期 + 单次使用
- 存储费用 → 设置图片大小限制（5MB）

## Open Questions

- 是否需要图片水印？
