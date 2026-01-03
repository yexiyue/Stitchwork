# Add Auth Pages

## Summary

实现登录注册页面，使用 Zustand 管理用户认证状态（token 和用户信息），并添加路由守卫保护需要认证的页面。

## Motivation

当前项目已有 API 层 (`authApi`) 和类型定义，但缺少：
- 用户认证状态管理
- 登录/注册 UI 页面
- 路由保护机制

## Scope

- 创建 `useAuthStore` Zustand store 管理 token 和用户信息
- 创建登录页面 `/login`
- 创建注册页面 `/register`
- 添加路由守卫，未登录时重定向到登录页
- 持久化 token 到 localStorage（与现有 client.ts 集成）
