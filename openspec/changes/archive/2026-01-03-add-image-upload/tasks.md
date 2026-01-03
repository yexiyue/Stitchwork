# Tasks

## 1. 后端实现

- [x] 1.1 添加 AWS S3 SDK 依赖 (`aws-sdk-s3`, `aws-config`)
- [x] 1.2 添加 S3 配置项 (endpoint, bucket, region, credentials)
- [x] 1.3 实现 Presigned URL 生成接口 `POST /api/upload/presign`

## 2. 前端实现

- [x] 2.1 添加 browser-image-compression 依赖
- [x] 2.2 创建 upload API 模块 (`src/api/upload.ts`)
- [x] 2.3 创建 ImageUploader 组件 (`src/components/ImageUploader.tsx`)

## 3. 集成验证

- [x] 3.1 在订单创建页面集成图片上传
- [x] 3.2 测试上传流程完整性
