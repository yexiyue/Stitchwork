## 1. 后端 API

- [x] 1.1 在 `crates/server/src/service/workshop/dto.rs` 新增 `ResetStaffPasswordRequest` DTO（含 `new_password: String` 字段）
- [x] 1.2 将 `crates/server/src/service/auth/service.rs` 中的 `hash_password` 函数可见性改为 `pub`
- [x] 1.3 在 `crates/server/src/service/workshop/service.rs` 新增 `reset_staff_password` 函数：校验员工属于本工坊、角色为 staff、密码非空，然后更新密码哈希
- [x] 1.4 在 `crates/server/src/service/workshop/controller.rs` 新增 `reset_staff_password` handler，注册路由 `PUT /api/staff/{id}/password`

## 2. 前端 API 客户端

- [x] 2.1 在 `src/api/auth.ts` 的 `authApi` 中新增 `resetStaffPassword(staffId: string, newPassword: string)` 方法，调用 `PUT /api/staff/${staffId}/password`

## 3. 前端 UI

- [x] 3.1 在 `src/routes/_auth/_boss/staff/index.tsx` 的 SwipeAction 中新增"重置密码"按钮（蓝色，排在"移除"前面）
- [x] 3.2 实现 `handleResetPassword` 函数：弹出 Dialog 包含密码输入框，确认后调用 API，成功提示"密码重置成功"

## 4. 验证

- [x] 4.1 `cargo check --workspace` 后端编译通过
- [x] 4.2 `pnpm tsc --noEmit` 前端类型检查通过
