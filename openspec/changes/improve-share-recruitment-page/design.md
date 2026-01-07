# Design: 招工分享页面改进

## 数据结构变更

### PublicShareResponse (修改)

```rust
pub struct PublicShareResponse {
    pub title: String,
    pub workshop_name: Option<String>,
    pub workshop_address: Option<String>,  // 新增：工坊地址
    pub boss_phone: Option<String>,         // 新增：老板联系方式
    pub avatar: Option<String>,
    pub processes: Vec<PublicProcessInfo>,  // 保留，增加字段
    // 移除：orders, workshop_desc
}

pub struct PublicProcessInfo {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub piece_price: Decimal,
    pub order_product_name: String,
    pub remaining_quantity: i32,  // 新增：剩余数量
}
```

## 剩余数量计算

`remaining_quantity = order.quantity - sum(approved_piece_records.quantity)`

SQL 查询逻辑：

```sql
SELECT p.id, p.name, p.piece_price, o.quantity,
       COALESCE(SUM(pr.quantity) FILTER (WHERE pr.status = 'Approved' OR pr.status = 'Settled'), 0) as completed
FROM process p
JOIN order o ON p.order_id = o.id
LEFT JOIN piece_record pr ON pr.process_id = p.id
WHERE p.id IN (...)
GROUP BY p.id, p.name, p.piece_price, o.quantity
```

剩余数量 = o.quantity - completed

## 前端页面

路由：`/share/$token` (公开页面，无需认证)

布局：

1. 顶部：工坊名称 + 老板头像
2. 联系信息卡片：手机号（可点击拨打）+ 地址
3. 工序列表：卡片形式展示每个工序
   - 工序名称
   - 单价
   - 剩余数量
   - 产品名称（辅助信息）

## 图片导出功能

### 技术方案

使用 `html2canvas` 将页面内容渲染为图片：

```typescript
import html2canvas from 'html2canvas';

const exportImage = async (element: HTMLElement) => {
  const canvas = await html2canvas(element, {
    scale: 2, // 高清
    useCORS: true,
  });
  return canvas.toDataURL('image/png');
};
```

### 二维码生成

使用 `qrcode.react` 生成指向当前页面的二维码：

```tsx
import { QRCodeSVG } from 'qrcode.react';

<QRCodeSVG value={window.location.href} size={80} />
```

### 导出图片布局

```text
┌────────────────────────┐
│     工坊名称 + 头像     │
├────────────────────────┤
│   📍 地址 | 📞 电话     │
├────────────────────────┤
│  ┌──────┐  ┌──────┐   │
│  │工序1 │  │工序2 │   │
│  │单价  │  │单价  │   │
│  │剩余  │  │剩余  │   │
│  └──────┘  └──────┘   │
├────────────────────────┤
│  [二维码]  扫码查看详情  │
└────────────────────────┘
```

### 依赖

- `html2canvas`: 页面截图
- `qrcode.react`: 二维码生成（项目已有）

## 兼容性

- 保留 `order_ids` 和 `process_ids` 字段用于筛选
- `orders` 字段从响应中移除，前端不再使用
