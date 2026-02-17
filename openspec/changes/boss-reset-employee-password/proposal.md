## Why

员工忘记密码后无法登录系统，当前只有"修改密码"功能（需要登录+旧密码）。作为临时方案（未集成短信验证码前），老板应能在员工管理中直接重置员工密码，让员工能尽快恢复使用。

## What Changes

- 后端新增 `PUT /api/staff/{id}/password` 端点，老板可重置本工坊员工的密码
- 前端员工管理页面（`staff/index.tsx`）的左滑操作中新增"重置密码"按钮
- 重置时弹窗输入新密码，确认后调用 API

## Capabilities

### New Capabilities
- `boss-reset-password`: 老板重置本工坊员工密码的能力，包含后端 API、权限校验、前端交互

### Modified Capabilities

## Impact

- 后端：`crates/server/src/service/workshop/` 新增重置密码端点和 service 逻辑
- 前端：`src/routes/_auth/_boss/staff/index.tsx` 新增 SwipeAction 按钮和弹窗逻辑
- API 客户端：`src/api/auth.ts` 新增 `resetStaffPassword` 方法
