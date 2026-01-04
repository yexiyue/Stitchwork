# Proposal: add-staff-registration

## Summary

实现员工扫码注册功能：员工扫描 Boss 生成的邀请码二维码，跳转到注册页完成注册并自动绑定工坊。

## Background

当前系统存在问题：
- 注册接口只能创建 Boss 角色（硬编码 `role: Role::Boss`）
- `bind_workshop` 要求用户已经是 Staff 角色
- 结果：员工无法注册和加入工坊

现有邀请码机制：
- Boss 可生成邀请码，前端显示二维码 `stitch://bind?code=xxx`
- 但没有对应的员工注册流程

## Scope

### In Scope

- 后端：员工注册接口（带邀请码，自动创建 Staff 并绑定工坊）
- 前端：员工注册页面（从二维码跳转，邀请码自动填入）
- 员工登录后的首页跳转逻辑

### Out of Scope

- 修改 Boss 注册流程
- 邀请码管理功能增强
- 员工个人中心手动输入邀请码绑定

## Approach

1. **后端**：新增 `POST /register-staff` 接口
   - 接受 `username`, `password`, `inviteCode`
   - 验证邀请码有效性
   - 创建 Staff 用户并绑定 workshop_id
   - 返回 token 和用户信息（直接登录）

2. **前端**：新增员工注册页面 `/register-staff`
   - URL 参数接收邀请码 `?code=xxx`
   - 邀请码自动填入（只读显示）
   - 注册成功后自动登录跳转首页

3. **Deep Link**：修改二维码链接格式
   - 从 `stitch://bind?code=xxx` 改为 Web URL
   - 或保持 Deep Link 由 Tauri 处理跳转

## Dependencies

- 现有邀请码机制（InviteCodes HashMap）
- auth service 和 workshop service
- 前端路由系统
