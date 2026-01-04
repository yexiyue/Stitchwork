# Proposal: enhance-boss-records-ui

## Summary

增强老板端计件管理功能，添加订单图片显示、详情页、批量审核和统计功能。

## Motivation

当前老板端计件管理存在以下不足：
1. 列表卡片没有显示订单图片，辨识度低
2. 点击卡片没有反应，无法查看完整信息或进行编辑/删除
3. 需要逐条滑动审核，当待审核记录多时效率低
4. 没有统计视图，难以快速了解员工产量和订单进度

## Proposed Changes

### 1. 列表卡片显示订单图片
- 复用已有的 `orderImage` 字段（刚在 add-staff-piece-record 中实现）
- 卡片左侧显示 56×56 的订单封面图

### 2. 计件详情页
- 点击卡片进入详情页 `/records/$id`
- 显示订单信息（图片轮播、产品名、描述）
- 显示工序信息（名称、单价）
- 显示计件信息（数量、金额、状态、录入方式、时间）
- 支持编辑数量（仅限 pending 状态）
- 支持删除记录

### 3. 批量审核功能
- 列表顶部添加"批量审核"入口
- 进入批量模式后可多选记录
- 一键批量通过/拒绝选中的 pending 记录

### 4. 统计功能
- 添加统计页面 `/records/stats`
- 员工产量统计（按员工分组显示数量、金额）
- 支持日期范围筛选
- 复用已有的 `GET /stats/workers` API

## Impact

- **Affected specs**: frontend-api（已有 stats API）
- **New specs**:
  - boss-records-list-ui
  - boss-records-detail-ui
  - boss-records-batch-approve
  - boss-records-statistics-ui
- **Affected code**:
  - `src/routes/_auth/_boss/records/index.tsx` - 添加图片、批量模式
  - `src/routes/_auth/_boss/records/$id.tsx` - 新建详情页
  - `src/routes/_auth/_boss/records/stats.tsx` - 新建统计页
  - `src/api/piece-record.ts` - 添加批量审核 API
  - `server/src/service/piece_record/` - 添加批量审核接口

## Out of Scope

- 导出 Excel 功能
- 订单级别统计（已有 order stats API）
