## Context

Tauri 2 移动端应用需要完整的前端技术栈支撑业务开发。选型基于以下考量：

- **Ant Design Mobile**: 国内最成熟的移动端 React UI 库，组件丰富，文档完善
- **TanStack Router**: 类型安全的文件路由，与 React 19 兼容性好
- **TanStack Query**: 服务端状态管理，自动缓存/重试/失效
- **Zustand**: 轻量级客户端状态管理，API 简洁
- **ahooks**: 阿里出品的 React Hooks 库，覆盖常见场景

## Goals / Non-Goals

Goals:
- 建立统一的前端基础设施
- 支持类型安全的路由和状态管理
- 提供移动端优化的 UI 组件

Non-Goals:
- 不涉及具体业务页面开发
- 不涉及后端 API 对接

## Decisions

### 路由方案: TanStack Router

选择原因：
- 文件系统路由，开发体验好
- 完整的 TypeScript 类型推断
- 内置数据加载 (loader)
- 与 TanStack Query 深度集成

### 状态管理: Zustand + TanStack Query

- Zustand 管理客户端状态 (UI 状态、用户偏好)
- TanStack Query 管理服务端状态 (API 数据)
- 职责分离，避免状态混乱

### 目录结构

```
src/
├── routes/           # TanStack Router 文件路由
│   ├── __root.tsx    # 根布局
│   └── index.tsx     # 首页
├── stores/           # Zustand stores
├── lib/              # 工具配置
│   └── query-client.ts
└── main.tsx          # 入口
```

## Risks / Trade-offs

- TanStack Router 相对较新，社区资源少于 React Router
  - 缓解：官方文档完善，类型安全带来的收益大于学习成本

## Open Questions

无
