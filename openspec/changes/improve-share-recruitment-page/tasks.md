# Tasks

## Backend

- [x] 修改 `PublicShareResponse` DTO，添加 `workshop_address` 和 `boss_phone`
- [x] 修改 `PublicProcessInfo` DTO，添加 `remaining_quantity`
- [x] 更新 `get_public` 服务：计算每个工序的剩余数量
- [x] 更新 `get_public` 服务：返回老板手机号和工坊地址
- [x] 移除 `orders` 字段及相关查询逻辑

## Frontend

- [x] 创建公开路由 `/share/$token`
- [x] 创建招工页面组件 `share/$token.tsx`
- [x] 实现页面布局：工坊信息 + 联系方式 + 工序列表
- [x] 添加拨打电话功能（点击手机号）
- [x] 添加 share API 客户端方法
- [x] 处理加载和错误状态
- [x] 安装 html2canvas 依赖
- [x] 实现图片导出功能（含二维码）
- [x] 添加导出按钮和保存图片逻辑

## Boss Share Management

- [x] 创建分享管理 hooks (`use-shares.ts`)
- [x] 创建分享列表页面 (`/shares`)
- [x] 创建新建分享页面 (`/shares/new`)
- [x] 创建编辑分享页面 (`/shares/$id`)
- [x] 实现复制链接功能
- [x] 实现二维码展示弹窗
- [x] 实现启用/停用分享开关

## Testing

- [ ] 验证剩余数量计算正确性
- [ ] 验证公开页面无需登录可访问
- [ ] 验证无效 token 返回 404
