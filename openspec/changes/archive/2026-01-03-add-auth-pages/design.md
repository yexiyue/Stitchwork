# Design: Auth Pages

## Architecture

### 登录接口增强

修改 `LoginResponse` 返回完整用户信息：
- `token`: JWT token
- `user`: 用户对象
  - `userId`, `username`, `role`
  - `displayName`, `phone`, `avatar`
  - `workshopName`, `workshopDesc` (boss 专属)

### 状态管理

使用 Zustand 创建 `useAuthStore`：
- `token`: JWT token
- `user`: 用户信息 (含 role: 'boss' | 'staff')
- `isAuthenticated`: 计算属性
- `isBoss`: 是否为老板角色
- `login()`: 调用 API 并保存状态
- `register()`: 调用注册 API
- `logout()`: 清除状态和 token

### Token 持久化

与现有 `client.ts` 的 `setToken/clearToken/getToken` 集成：
- 登录成功后同步到 localStorage
- 应用启动时从 localStorage 恢复
- 使用 zustand/middleware 的 persist 中间件

### 路由保护与角色路由

使用 TanStack Router 的 `beforeLoad` 钩子：
- 检查 `isAuthenticated` 状态
- 未认证时重定向到 `/login`
- 登录页在已认证时重定向到首页
- 根据 `role` 渲染不同的路由/菜单

## File Structure

```
server/src/service/auth/dto.rs  # 修改 LoginResponse
src/types/auth.ts               # 更新前端类型
src/stores/auth.ts              # Zustand auth store
src/routes/
├── _auth.tsx                   # 需要认证的布局
├── _auth/                      # 受保护的路由
│   └── index.tsx               # 首页（移动）
├── login.tsx                   # 登录页
└── register.tsx                # 注册页
```
