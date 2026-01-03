## Context

服装加工作坊需要展示更丰富的信息，并支持公开招工页面吸引员工。

## Goals

- 实体支持描述和图片
- Boss 可生成公开招工链接
- 图片字段兼容 OSS 直接访问

## Non-Goals

- 不实现图片上传（由客户端直传 OSS）
- 不实现复杂的分享权限控制

## Decisions

### 1. 字段设计

| 实体 | 新增字段 | 类型 | 说明 |
|-----|---------|------|------|
| Order | description | Option<String> | 订单描述 |
| Order | images | Option<Json> | 产品图片 URL 数组 |
| Process | description | Option<String> | 工序描述 |
| Customer | description | Option<String> | 客户备注 |
| User | avatar | Option<String> | 头像 URL |
| User | workshop_name | Option<String> | 作坊名称 |
| User | workshop_desc | Option<String> | 作坊介绍 |

### 2. 图片存储格式

```json
["https://oss.example.com/img1.jpg", "https://oss.example.com/img2.jpg"]
```

使用 JSON 数组存储，客户端可直接访问 OSS URL。

### 3. 招工分享设计

#### 3.1 分享链接生成

Boss 创建分享时选择要展示的订单/工序，生成唯一 share_token：

```
GET /share/{share_token}
```

#### 3.2 分享数据结构

```rust
struct Share {
    id: Uuid,
    boss_id: Uuid,
    share_token: String,      // 唯一访问令牌
    title: String,            // 招工标题
    description: Option<String>,
    order_ids: Vec<Uuid>,     // 选择展示的订单
    process_ids: Vec<Uuid>,   // 选择展示的工序
    is_active: bool,
    created_at: DateTime,
    expires_at: Option<DateTime>,
}
```

#### 3.3 公开接口

```
GET /public/share/{token}  // 无需认证，返回招工信息
```

返回内容：
- 作坊信息（名称、介绍、头像）
- 选中的订单列表（产品名、描述、图片）
- 选中的工序列表（名称、描述、工价）
