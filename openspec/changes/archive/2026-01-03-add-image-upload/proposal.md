# Change: Add Image Upload Capability

## Why

系统需要通用的图片上传能力，支持订单图片、用户头像等多种场景。当前 Order 类型已预留 `images` 字段但无实现。

## What Changes

- 新增 AWS S3 兼容存储集成（后端）
- 新增 Presigned URL 生成接口
- 新增前端图片上传组件
- 支持图片压缩和格式校验

## Impact

- Affected specs: 新增 `image-upload` capability
- Affected code:
  - 后端: 新增 S3 配置、Presigned URL handler
  - 前端: 新增 ImageUploader 组件、upload API
