## Why

当前注册码系统分裂为两套独立机制：超管创建 B-XXXXXXXX 格式码（持久化到数据库）供老板注册，老板生成 16 位 hex 邀请码（存内存 HashMap，24h 过期）供员工注册。这导致：代码重复、员工邀请码服务重启丢失、前端无法复用管理界面、两套注册接口维护成本高。统一为一套 6 位数字码 + 数据库持久化方案，简化架构。

## What Changes

- **BREAKING**: 注册码格式从 `B-XXXXXXXX`（8 位字母数字）改为 6 位纯数字
- **BREAKING**: 合并 `POST /api/register` 和 `POST /api/register-staff` 为统一的 `POST /api/register`，注册后统一自动登录返回 token
- **BREAKING**: 删除 `POST /api/invite-code` 内存邀请码接口
- **BREAKING**: 删除 `GET/POST/DELETE /api/admin/register-codes` 超管专用路由，统一为 `/api/register-codes`（按 JWT 角色过滤数据）
- 注册码 entity 删除 `is_active` 字段，新增 `created_by`（创建者）和 `workshop_id`（关联工坊，用于区分 Boss 码和 Staff 码）
- 不要的码改为直接删除（仅未使用的可删），替代原来的"禁用"功能
- 移除 AppState 中的内存 `InviteCodes` HashMap
- 前端：抽取 `RegisterCodeManager` 共享组件，超管和 Boss 复用同一套注册码管理 UI
- 前端：Boss 新增注册码管理独立页面，入口在员工管理页 NavBar + 个人中心

## Capabilities

### New Capabilities
- `register-code`: 统一注册码系统，覆盖码的生成（6 位数字）、CRUD 接口、权限控制（超管创建 Boss 码、Boss 创建 Staff 码）、以及统一注册消费流程

### Modified Capabilities
- `auth-api`: 合并注册接口为统一 `POST /api/register`，注册后统一自动登录返回 token；删除 `POST /api/register-staff` 和 `POST /api/invite-code`
- `auth-ui`: 前端注册页去除 B- 前缀判断，统一注册流程；Boss 新增注册码管理页面入口

## Impact

- **后端**: `crates/entity/src/register_code.rs`（entity 重构）、`crates/server/src/service/admin/`（移除注册码相关逻辑）、`crates/server/src/service/auth/`（统一注册接口）、`crates/server/src/service/workshop/`（移除邀请码）、`crates/server/src/state.rs`（移除 InviteCodes）、新建 `crates/server/src/service/register_code/`
- **前端**: `src/api/`（统一 API）、`src/types/`（类型更新）、`src/routes/register.tsx`（统一注册流程）、`src/routes/_auth/admin/register-codes.tsx`（改用共享组件）、新建 `src/components/register-code-manager.tsx` 和 `src/routes/_auth/_boss/register-codes.tsx`
- **数据库**: `register_code` 表结构变更（drop `is_active`，add `created_by`/`workshop_id`），旧 B- 格式数据保留
- **API 破坏性变更**: 前后端需同步部署
