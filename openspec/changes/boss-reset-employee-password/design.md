## Context

当前系统中密码修改功能（`PUT /api/password`）要求用户已登录且提供旧密码。员工忘记密码后完全无法恢复访问。在未接入短信验证码服务之前，利用老板与员工的管理关系，提供老板重置员工密码的能力。

现有相关代码：
- 后端员工管理在 `crates/server/src/service/workshop/`，已有 `GET /api/staff`（列表）和 `DELETE /api/staff/{id}`（移除）
- 前端员工管理页在 `src/routes/_auth/_boss/staff/index.tsx`，已有左滑移除操作
- 密码哈希使用 Argon2，工具函数 `hash_password` 已在 `auth/service.rs` 中

## Goals / Non-Goals

**Goals:**
- 老板能为本工坊的员工重置密码
- 权限校验：只能重置自己工坊的员工，不能重置其他老板或超管
- 复用现有的 UI 模式（SwipeAction + Dialog）

**Non-Goals:**
- 不实现短信验证码找回密码
- 不实现员工自助重置密码
- 不实现密码强度校验（保持与现有注册/修改密码一致）

## Decisions

### 1. API 放在 workshop 模块而非 auth 模块

**选择**：`PUT /api/staff/{id}/password` 注册在 workshop controller 中

**理由**：这是老板管理员工的操作，属于 workshop 领域。auth 模块负责用户自身的认证操作。现有的 `DELETE /api/staff/{id}` 也在 workshop controller 中，保持一致。

### 2. 老板直接指定新密码

**选择**：由老板在弹窗中输入新密码，而非系统生成随机密码

**理由**：老板可以当面告知员工新密码，简单直接。系统生成随机密码需要额外的展示和复制 UI，且在面对面场景下反而不如直接输入方便。

### 3. 复用 `hash_password` 函数

**选择**：将 `auth::service::hash_password` 改为 `pub` 可见性，在 workshop service 中复用

**理由**：避免重复实现 Argon2 哈希逻辑。该函数无状态，纯工具函数，适合跨模块共享。

## Risks / Trade-offs

- [老板可设置弱密码] → 当前系统注册时也无密码强度校验，保持一致，后续统一添加
- [老板可能滥用重置权限] → 只能重置自己工坊的员工，且 staff 角色只能看自己的数据，风险有限
