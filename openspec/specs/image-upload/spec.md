# image-upload Specification

## Purpose
TBD - created by archiving change add-image-upload. Update Purpose after archive.
## Requirements
### Requirement: Presigned URL Generation

系统 SHALL 提供 Presigned URL 接口，允许客户端直传图片到 S3 兼容存储。

#### Scenario: Request presigned URL successfully

- **WHEN** 已认证用户请求 `POST /api/upload/presign` 并提供文件名和 content-type
- **THEN** 返回包含 `uploadUrl` 和 `fileUrl` 的响应
- **AND** uploadUrl 有效期为 15 分钟
- **AND** fileUrl 为上传成功后的访问地址

#### Scenario: Unauthenticated request

- **WHEN** 未认证用户请求 Presigned URL
- **THEN** 返回 401 Unauthorized

### Requirement: Image Upload Client Component

前端 SHALL 提供可复用的图片上传组件。

#### Scenario: Select and upload image

- **WHEN** 用户点击上传区域
- **THEN** 显示图片选择器（相册/拍照）
- **AND** 选择后自动压缩图片（目标 < 500KB）
- **AND** 请求 Presigned URL 并 PUT 上传
- **AND** 返回图片访问 URL

#### Scenario: Upload progress indication

- **WHEN** 图片正在上传
- **THEN** 显示上传进度

#### Scenario: Upload size limit

- **WHEN** 选择的图片超过 5MB
- **THEN** 显示错误提示 "图片大小不能超过 5MB"

#### Scenario: Multiple images upload

- **WHEN** 组件配置为多图模式
- **THEN** 支持选择和上传多张图片
- **AND** 显示已上传图片列表
- **AND** 支持删除已上传图片

