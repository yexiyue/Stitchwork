# Proposal: add-admin-dashboard

## Summary

为超级管理员添加仪表盘首页，展示平台统计数据和图表，并为用户列表和注册码列表添加分页支持。

## Motivation

当前超管首页只是一个简单的导航列表，缺乏平台运营数据的可视化展示。随着用户和注册码数量增长，列表页面需要分页和虚拟滚动支持。

## Scope

### Backend Changes

1. **新增统计接口** `GET /api/admin/stats`
   - 用户统计：总数、老板数、员工数、今日/本周/本月新增
   - 工坊统计：总数、活跃数
   - 注册码统计：总数、已使用、可用、已禁用
   - 平台活跃度：今日/本月订单数、计件记录数

2. **用户列表分页** `GET /api/admin/users`
   - 添加 page, pageSize 参数
   - 返回 { list, total } 格式

3. **注册码列表分页** `GET /api/admin/register-codes`
   - 添加 page, pageSize 参数
   - 返回 { list, total } 格式

### Frontend Changes

1. **超管首页重构** `/admin`
   - 统计卡片：用户数、工坊数、注册码使用率
   - 图表：用户增长趋势、注册码使用情况饼图
   - 快捷入口保留

2. **用户列表页** `/admin/users`
   - 使用 VirtualList 组件
   - 支持下拉刷新和无限滚动

3. **注册码列表页** `/admin/register-codes`
   - 使用 VirtualList 组件
   - 支持下拉刷新和无限滚动

## Out of Scope

- 用户详情页
- 用户编辑/禁用功能
- 注册码批量操作

## Dependencies

- 现有 VirtualList 组件
- 现有 Chart 组件
- 现有分页模式 (useInfiniteList hook)
