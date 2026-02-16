## Context

当前系统有两套注册码机制：
1. **Boss 注册码**: 超管通过 `/api/admin/register-codes` 创建 `B-XXXXXXXX` 格式码，存 `register_code` 表，有 `is_active` 禁用功能
2. **Staff 邀请码**: Boss 通过 `/api/invite-code` 生成 16 位 hex 码，存 AppState 内存 HashMap，24h 过期

两者代码路径完全独立，前端也无法复用管理 UI。本次统一为一套基于数据库的 6 位数字码系统。

## Goals / Non-Goals

**Goals:**
- 统一注册码 entity、API、前端 UI 为一套机制
- Boss 和超管复用同一套注册码管理组件
- 移除内存邀请码，所有码持久化到数据库
- 统一注册接口，注册后自动登录

**Non-Goals:**
- 注册码过期机制（本次不加，后续可扩展 `expires_at` 字段）
- 注册码批量创建
- 已使用码的清理/归档策略

## Decisions

### 1. Entity 变更

**决定**: 删除 `is_active`，新增 `created_by: Uuid` 和 `workshop_id: Option<Uuid>`

- `created_by` 记录创建者（超管或 Boss），用于权限过滤和审计
- `workshop_id` 决定码的语义：有值 → Staff 码（消费后加入该 workshop），无值 → Boss 码
- 不再需要 `is_active`，不要的码直接 DELETE（仅未使用的可删）

**替代方案**: 用 `role: Enum(Boss, Staff)` 字段代替 `workshop_id` 隐式推断。放弃原因：`workshop_id` 本身就携带了角色信息且是 Staff 注册所必需的数据，额外加 role 字段是冗余。

### 2. 码格式

**决定**: 6 位纯数字 `000000-999999`，不带前缀

生成逻辑加碰撞检测重试。100 万种可能对此应用规模足够。注册接口应加频率限制防暴力猜测（但频率限制不在本次范围内）。

### 3. 模块组织

**决定**: 新建 `crates/server/src/service/register_code/` 独立模块

```
service/register_code/
├── mod.rs
├── dto.rs          ← RegisterCodeResponse, CreateCodeParams
├── service.rs      ← create, list, delete, generate_code
└── controller.rs   ← GET/POST/DELETE /api/register-codes
```

controller 通过 JWT claims 获取用户角色和 ID：
- 超管：创建的码 `workshop_id = None`，list 时返回所有无 workshop 的码
- Boss：创建的码 `workshop_id = Some(boss 的 workshop_id)`，list 时返回自己创建的码

### 4. 统一注册接口

**决定**: 合并为 `POST /api/register`，请求体统一为 `{ code, username, password, phone }`

后端逻辑：
1. 查 `register_code` 表找到码（必须 `used_by IS NULL`）
2. 根据 `workshop_id` 决定创建 Boss 还是 Staff
3. 标记码为已使用
4. 统一返回 `LoginResponse { token, user }`（自动登录）

删除 `POST /api/register-staff` 和 `POST /api/invite-code`。

### 5. 前端组件复用

**决定**: 抽取 `src/components/register-code-manager.tsx` 共享组件

组件接收 API 函数作为 props（list/create/delete），超管和 Boss 页面传入不同的 query key。两者调用同一个 `/api/register-codes` 后端接口，后端按角色过滤。

Boss 注册码管理页面入口：员工管理页 NavBar 图标 + 个人中心菜单项。

## Risks / Trade-offs

- **6 位数字安全性较弱** → 码为一次性使用，活跃码数量极少，实际可猜中概率极低。后续可加注册接口频率限制。
- **旧 B- 格式数据保留** → schema sync 会 drop `is_active` 列、add `created_by`/`workshop_id` 列。旧数据的 `created_by` 和 `workshop_id` 将为 NULL。已使用的旧码仍可查看（`used_by` 不为空），未使用的旧码格式不一致但功能不受影响。
- **前后端必须同步部署** → API 有破坏性变更，旧前端调旧接口会 404。
