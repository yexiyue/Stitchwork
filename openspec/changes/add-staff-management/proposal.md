# Proposal: add-staff-management

## Summary

为 Boss 用户实现员工管理功能的前端 UI，包括员工列表、创建员工账号、生成邀请码三个核心功能。

## Background

- 后端 API 已完整实现（`/api/staff`, `/api/invite-code`, `/api/bind-boss`）
- 前端 `authApi` 已封装好所有接口调用
- Profile 页面已预留员工管理入口（当前 onClick 为空）
- 需要实现 UI 页面让 Boss 能够管理员工

## Scope

### In Scope

- 员工列表页面（显示当前 Boss 下的所有员工）
- 创建员工账号页面（Boss 直接创建员工账号）
- 邀请码功能（生成邀请码供员工自行绑定）

### Out of Scope

- 员工编辑/删除功能
- 员工详情页面
- 后端 API 修改

## Approach

复用现有 UI 组件规范（antd-mobile），遵循项目既有的页面布局模式：
1. 员工列表页使用 VirtualList + SwipeAction
2. 新增员工页使用 Form 组件
3. 邀请码使用 Dialog/Popup 展示

## Dependencies

- 现有 `authApi` 接口
- antd-mobile 组件库
- TanStack Router 路由
