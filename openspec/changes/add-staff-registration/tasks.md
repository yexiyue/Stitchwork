# Tasks: add-staff-registration

## Backend Tasks

- [x] 1. 新增 `RegisterStaffRequest` DTO（username, password, inviteCode）
- [x] 2. 实现 `register_staff` service 函数
  - 验证邀请码有效性
  - 创建 Staff 用户并设置 workshop_id
  - 生成 JWT token
  - 返回 LoginResponse
- [x] 3. 新增 `POST /register-staff` controller 路由
- [x] 4. 使用 cargo check 验证编译通过

## Frontend Tasks

- [x] 5. 新增 `authApi.registerStaff(req)` 接口
- [x] 6. 创建 `/register-staff` 路由页面
  - 从 URL query 获取邀请码
  - 显示邀请码（只读）
  - 用户名、密码、确认密码输入
  - 提交注册
- [x] 7. 注册成功后自动登录并跳转首页
- [x] 8. 修改邀请码二维码链接为 Web URL `/register-staff?code=xxx`

## Validation

- [x] 9. 运行 pnpm build 验证前端编译（新增代码无类型错误，预存错误与本次改动无关）
- [ ] 10. 手动测试完整流程：生成邀请码 → 扫码注册 → 员工登录
