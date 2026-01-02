# 图片上传功能调研

## 1. 使用场景

- 订单拿货时拍照记录款式
- 计件完成时拍照存档
- 其他业务凭证

## 2. 技术方案对比

| 方案 | 优点 | 缺点 | 成本 |
|------|------|------|------|
| 本地文件存储 | 简单、无额外费用 | 磁盘空间有限、备份麻烦 | 低 |
| 阿里云 OSS | 稳定、CDN加速、无限扩展 | 需要额外配置和费用 | 中 |
| 数据库存储 | 事务一致性好 | 性能差、数据库膨胀 | 不推荐 |

**推荐方案：阿里云 OSS**（你已经用阿里云 ECS，OSS 集成方便）

## 3. 实现方案

### 3.1 后端依赖

```toml
# Cargo.toml 新增
axum = { version = "0.8.8", features = ["macros", "multipart"] }
tokio = { version = "1", features = ["full", "fs"] }

# 方案A: 本地存储（无需额外依赖）

# 方案B: 阿里云 OSS
aliyun-oss-rust-sdk = "0.10"
```

### 3.2 API 设计

```text
POST   /api/images              # 上传图片，返回 image_id
GET    /api/images/:id          # 获取图片
DELETE /api/images/:id          # 删除图片
```

### 3.3 数据库设计

```sql
CREATE TABLE image (
    id UUID PRIMARY KEY,
    filename VARCHAR(255) NOT NULL,
    content_type VARCHAR(100) NOT NULL,
    storage_path VARCHAR(500) NOT NULL,  -- 本地路径或 OSS key
    size_bytes BIGINT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW()
);

-- 订单关联图片（可选）
ALTER TABLE "order" ADD COLUMN images UUID[] DEFAULT '{}';
```

### 3.4 核心代码示例

```rust
// routes/image.rs
use axum::{
    extract::Multipart,
    response::Json,
};
use uuid::Uuid;

pub async fn upload(mut multipart: Multipart) -> Result<Json<ImageResponse>, AppError> {
    while let Some(field) = multipart.next_field().await? {
        let filename = field.file_name().unwrap_or("unknown").to_string();
        let content_type = field.content_type().unwrap_or("application/octet-stream").to_string();
        let data = field.bytes().await?;

        let id = Uuid::new_v4();
        let path = format!("uploads/{}/{}", id, filename);

        // 保存文件
        tokio::fs::create_dir_all("uploads").await?;
        tokio::fs::write(&path, &data).await?;

        // 保存记录到数据库
        // ...

        return Ok(Json(ImageResponse { id, url: format!("/api/images/{}", id) }));
    }
    Err(AppError::BadRequest("No file uploaded".into()))
}
```

### 3.5 前端（React）

```tsx
const uploadImage = async (file: File) => {
  const formData = new FormData();
  formData.append('file', file);

  const res = await fetch('/api/images', {
    method: 'POST',
    body: formData,
  });
  return res.json();
};
```

## 4. 安全考虑

- 限制文件大小（建议 5MB）
- 限制文件类型（仅 image/jpeg, image/png）
- 生成随机文件名防止路径遍历
- 图片访问权限控制

## 5. 实施步骤

1. 添加 `axum` multipart 特性
2. 创建 image 表
3. 实现上传/获取/删除 API
4. 前端集成拍照/选图功能
5. （可选）迁移到 OSS

## 6. 阿里云 OSS 配置（后续扩展）

```rust
// 使用预签名 URL 方式，前端直传 OSS
// 后端只负责生成签名，减轻服务器压力

pub async fn get_upload_url() -> Json<PresignedUrl> {
    // 生成 OSS 预签名上传 URL
    // 前端拿到 URL 后直接 PUT 到 OSS
}
```

费用参考：OSS 标准存储 0.12 元/GB/月，流量另计
