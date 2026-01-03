# Change: 添加前端 API 请求模块

## Why

前端需要与服务端 API 交互，目前缺少统一的 API 请求层。需要创建类型安全的 API 模块，覆盖所有服务端接口。

## What Changes

- 创建 `src/api/` 目录，包含所有 API 请求函数
- 定义 TypeScript 类型，与服务端 DTO 对应
- 配置 axios/fetch 基础请求实例
- 集成 TanStack Query hooks

## Impact

- Affected specs: frontend-api (新增)
- Affected code:
  - `src/api/` - API 模块 (新增)
  - `src/types/` - 类型定义 (新增)
