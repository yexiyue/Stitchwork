# Change: 集成前端技术栈

## Why

当前项目仅有基础的 React 配置，缺少移动端 UI 组件库、路由、状态管理、数据请求等核心基础设施。需要集成完整的前端技术栈以支撑业务开发。

## What Changes

- 集成 Ant Design Mobile 作为移动端 UI 组件库
- 集成 lucide-react 作为图标库
- 集成 TanStack Router 作为路由方案
- 集成 TanStack Query 作为数据请求/缓存方案
- 集成 Zustand 作为全局状态管理
- 集成 ahooks 作为通用 hooks 库

## Impact

- Affected specs: frontend-infrastructure (新增)
- Affected code:
  - `package.json` - 新增依赖
  - `src/main.tsx` - 配置 providers
  - `src/routes/` - 路由配置 (新增)
  - `src/stores/` - 状态管理 (新增)
  - `src/lib/` - 工具配置 (新增)
  - `vite.config.ts` - 路径别名配置
  - `tsconfig.json` - 路径别名配置
