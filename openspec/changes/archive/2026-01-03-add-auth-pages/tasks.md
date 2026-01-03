## 1. 后端接口

- [x] 1.1 修改 `server/src/service/auth/dto.rs` LoginResponse 返回用户信息
- [x] 1.2 修改 `server/src/service/auth/service.rs` login 方法返回用户数据
- [x] 1.3 更新 `src/types/auth.ts` 前端类型定义

## 2. Auth Store

- [x] 2.1 创建 `src/stores/auth.ts` Zustand store
  - token, user (含 role) 状态
  - selectIsAuthenticated, selectIsBoss helper selectors
  - login, logout actions
  - persist 中间件持久化

## 3. 页面

- [x] 3.1 创建 `src/routes/login.tsx` 登录页
- [x] 3.2 创建 `src/routes/register.tsx` 注册页

## 4. 路由保护

- [x] 4.1 创建 `src/routes/_auth.tsx` 认证布局
- [x] 4.2 移动 `src/routes/index.tsx` 到 `src/routes/_auth/index.tsx`

## 5. 验证

- [x] 5.1 TypeScript 编译无错误
- [x] 5.2 Rust 编译无错误
