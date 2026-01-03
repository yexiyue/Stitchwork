## 1. 依赖安装

- [x] 1.1 安装 Ant Design Mobile 及相关依赖
- [x] 1.2 安装 lucide-react 图标库
- [x] 1.3 安装 TanStack Router
- [x] 1.4 安装 TanStack Query
- [x] 1.5 安装 Zustand
- [x] 1.6 安装 ahooks

## 2. 配置路径别名

- [x] 2.1 更新 `tsconfig.json` 添加 `@/*` 路径别名
- [x] 2.2 更新 `vite.config.ts` 添加对应 resolve alias

## 3. 路由配置

- [x] 3.1 创建 `src/routes/__root.tsx` 根路由
- [x] 3.2 创建 `src/routes/index.tsx` 首页路由
- [x] 3.3 创建 `src/routeTree.gen.ts` 路由树 (TanStack Router 自动生成)
- [x] 3.4 配置 Vite 插件 `@tanstack/router-plugin`

## 4. 状态管理配置

- [x] 4.1 创建 `src/stores/` 目录结构
- [x] 4.2 创建示例 store 文件

## 5. 数据请求配置

- [x] 5.1 创建 `src/lib/query-client.ts` 配置 QueryClient
- [x] 5.2 在根组件中包裹 QueryClientProvider

## 6. 入口文件整合

- [x] 6.1 更新 `src/main.tsx` 整合所有 providers
- [x] 6.2 配置 Ant Design Mobile 主题 (ConfigProvider)

## 7. 验证

- [x] 7.1 运行 `pnpm dev` 确认无编译错误
- [x] 7.2 验证路由跳转正常
- [x] 7.3 验证 UI 组件渲染正常
