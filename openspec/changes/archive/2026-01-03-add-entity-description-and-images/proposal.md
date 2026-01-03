# Change: Add Entity Description and Image Fields

## Why

1. 当前实体缺少描述字段，无法记录详细信息
2. 需要支持公开招工分享页面，展示订单/工序信息吸引员工
3. 需要预留图片字段支持后续 OSS 存储

## What Changes

- **实体扩展**: 为 Order、Process、Customer、User 添加 description 和 images 字段
- **招工分享**: Boss 可选择订单/工序生成公开招工链接
- **图片存储**: images 字段存储 JSON 数组格式的 OSS URL 列表

## Impact

- Affected entities: `order`, `process`, `customer`, `user`
- New module: `share` (招工分享)
- Database: 新增字段，需要 schema sync
