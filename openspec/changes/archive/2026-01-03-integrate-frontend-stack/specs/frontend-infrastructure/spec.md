## ADDED Requirements

### Requirement: Mobile UI Framework

系统 SHALL 使用 Ant Design Mobile 作为移动端 UI 组件库。

#### Scenario: UI 组件可用
- **WHEN** 开发者在组件中导入 antd-mobile 组件
- **THEN** 组件正常渲染且样式符合移动端规范

#### Scenario: 主题配置
- **WHEN** 应用启动时
- **THEN** ConfigProvider 提供全局主题配置

### Requirement: Icon Library

系统 SHALL 使用 lucide-react 作为图标库。

#### Scenario: 图标可用
- **WHEN** 开发者从 lucide-react 导入图标组件
- **THEN** 图标正常渲染为 SVG 元素

#### Scenario: 与 UI 组件配合
- **WHEN** 在 antd-mobile 组件中使用 lucide-react 图标
- **THEN** 图标正常显示且样式可自定义

### Requirement: File-based Routing

系统 SHALL 使用 TanStack Router 实现文件系统路由。

#### Scenario: 路由自动生成
- **WHEN** 在 `src/routes/` 目录下创建 `.tsx` 文件
- **THEN** 路由自动注册到路由树

#### Scenario: 类型安全导航
- **WHEN** 使用 `<Link>` 或 `useNavigate` 进行导航
- **THEN** 路由路径和参数具有完整的 TypeScript 类型检查

### Requirement: Server State Management

系统 SHALL 使用 TanStack Query 管理服务端状态。

#### Scenario: QueryClient 配置
- **WHEN** 应用启动时
- **THEN** QueryClientProvider 包裹应用根组件

#### Scenario: 数据请求
- **WHEN** 使用 `useQuery` 发起数据请求
- **THEN** 自动处理加载状态、缓存、错误重试

### Requirement: Client State Management

系统 SHALL 使用 Zustand 管理客户端状态。

#### Scenario: Store 创建
- **WHEN** 创建 Zustand store
- **THEN** 可在任意组件中通过 hook 访问状态

### Requirement: Utility Hooks

系统 SHALL 集成 ahooks 提供通用 React Hooks。

#### Scenario: Hooks 可用
- **WHEN** 开发者导入 ahooks 中的 hook
- **THEN** hook 正常工作且类型完整

### Requirement: Path Alias

系统 SHALL 配置 `@/*` 路径别名指向 `src/*`。

#### Scenario: 路径别名解析
- **WHEN** 使用 `import { X } from '@/lib/query-client'`
- **THEN** 正确解析到 `src/lib/query-client`
