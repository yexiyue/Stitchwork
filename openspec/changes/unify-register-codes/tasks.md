## 1. Entity 层变更

- [x] 1.1 更新 `crates/entity/src/register_code.rs`：删除 `is_active` 字段，新增 `created_by: Uuid` 和 `workshop_id: Option<Uuid>` 字段，更新关系定义

## 2. 后端：新建 register_code 独立模块

- [x] 2.1 创建 `crates/server/src/service/register_code/mod.rs`，导出 controller/dto/service
- [x] 2.2 创建 `dto.rs`：定义 `RegisterCodeResponse`（含 used_by_username）和查询参数结构体
- [x] 2.3 创建 `service.rs`：实现 6 位数字码生成（含碰撞重试）、create（按角色设置 workshop_id）、list（按角色过滤）、delete（仅未使用的可删，仅创建者可删）
- [x] 2.4 创建 `controller.rs`：实现 `GET/POST/DELETE /api/register-codes` 路由，JWT 鉴权获取用户角色

## 3. 后端：统一注册接口

- [x] 3.1 更新 `auth/dto.rs`：合并 `RegisterRequest` 和 `RegisterStaffRequest` 为统一结构体 `{ code, username, password, phone }`，返回类型改为 `LoginResponse`
- [x] 3.2 更新 `auth/service.rs`：合并 `register()` 和 `register_staff()` 为统一函数，根据码的 `workshop_id` 决定创建 Boss 或 Staff，统一返回 token（自动登录），移除对 `InviteCodes` 的依赖
- [x] 3.3 更新 `auth/controller.rs`：删除 `/register-staff` 路由，更新 `/register` handler 签名

## 4. 后端：清理旧代码

- [x] 4.1 清理 `admin/service.rs`：删除 `generate_code()`、`create_register_code()`、`list_register_codes()`、`disable_register_code()` 函数
- [x] 4.2 清理 `admin/controller.rs`：删除 register-codes 相关路由（POST/GET/DELETE /api/admin/register-codes）
- [x] 4.3 清理 `admin/dto.rs`：删除 `RegisterCodeResponse`（已移到 register_code 模块）
- [x] 4.4 更新 `admin/service.rs` 的 `get_stats()`：调整注册码统计逻辑（移除 `is_active`/`disabled_codes` 相关查询）
- [x] 4.5 清理 `workshop/service.rs`：删除 `generate_invite_code()` 函数
- [x] 4.6 清理 `workshop/controller.rs`：删除 `/invite-code` 路由
- [x] 4.7 清理 `state.rs`：删除 `InviteCodes` 类型别名和 `AppState.invite_codes` 字段，更新 `AppState::new()`

## 5. 后端：路由注册 + 编译验证

- [x] 5.1 更新 `service/mod.rs`：添加 `register_code` 模块，更新 `routes()` 函数合并新路由
- [x] 5.2 运行 `cargo check --workspace` 确认编译通过

## 6. 前端：类型和 API 更新

- [x] 6.1 更新 `src/types/`：统一 `RegisterCode` 类型（删除 `isActive`，加 `createdBy`/`workshopId`），合并 `RegisterRequest`（删除 `registerCode`/`inviteCode`，统一为 `code`），删除 `RegisterStaffRequest`、`InviteCodeResponse`
- [x] 6.2 更新 `src/api/`：新建注册码 API（`registerCodeApi.create/list/delete` 调用 `/api/register-codes`），更新 `authApi.register` 返回 `LoginResponse`，删除 `authApi.registerStaff` 和 `authApi.generateInviteCode`，删除 `adminApi` 中注册码相关方法

## 7. 前端：共享组件 + 页面

- [x] 7.1 创建 `src/components/register-code-manager.tsx`：从 `admin/register-codes.tsx` 提取核心逻辑，接收 queryKey prop 区分超管/Boss 数据，禁用→删除，移除 `isActive` 相关状态判断
- [x] 7.2 更新 `src/routes/_auth/admin/register-codes.tsx`：改用 `RegisterCodeManager` 组件
- [x] 7.3 创建 `src/routes/_auth/_boss/register-codes.tsx`：Boss 注册码管理页面，使用 `RegisterCodeManager` 组件

## 8. 前端：入口 + 注册流程

- [x] 8.1 更新 `src/routes/_auth/_boss/staff/index.tsx`：NavBar 的 UserPlus 改为跳转到注册码管理页面（替代弹窗生成邀请码）
- [x] 8.2 更新 Boss 个人中心页面：添加「注册码管理」菜单项入口
- [x] 8.3 更新 `src/routes/register.tsx`：移除 `isBossCode()` 前缀判断，统一调用 `authApi.register()`，注册成功后统一自动登录跳转首页
- [x] 8.4 清理 `src/hooks/use-auth.ts`：删除 `useGenerateInviteCode` 和 `useBindWorkshop`

## 9. 前端：清理 + 验证

- [x] 9.1 清理 `src/routes/_auth/admin/index.tsx`：admin dashboard 统计适配（移除 `disabledCodes`）
- [x] 9.2 运行 `pnpm tsc --noEmit` 确认 TypeScript 无错误
