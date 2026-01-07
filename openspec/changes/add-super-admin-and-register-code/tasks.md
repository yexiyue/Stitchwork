# Tasks: Add Super Admin and Registration Code

## Backend

### Entity Changes
- [x] user entity 添加 `is_super_admin: bool` 字段
- [x] user entity 的 `phone` 改为必填并添加唯一约束
- [x] 创建 `register_code` entity

### Admin Service
- [x] 创建 admin service 模块结构（mod.rs, dto.rs, controller.rs, service.rs）
- [x] 实现注册码 CRUD API
- [x] 实现用户列表 API
- [x] 添加超管权限中间件/extractor

### Auth Service Changes
- [x] RegisterRequest 添加 `phone` 和 `register_code` 字段
- [x] RegisterStaffRequest 添加 `phone` 字段
- [x] register 函数增加注册码校验和手机号唯一校验
- [x] register_staff 函数增加手机号唯一校验
- [x] login 函数支持用户名或手机号登录
- [x] LoginUser 返回 `is_super_admin` 字段

### Validation
- [x] 运行 `cargo check` 确保编译通过

## Frontend

### Types
- [x] 更新 User 类型添加 `isSuperAdmin`
- [x] 添加 RegisterCode 类型
- [x] 添加 admin API 类型

### API Client
- [x] 添加 adminApi（注册码、用户列表）

### Auth Store
- [x] 添加 `selectIsSuperAdmin` selector

### Routes
- [x] 创建 `admin.tsx` 布局路由（超管权限检查）
- [x] 创建 `/admin/register-codes` 注册码管理页
- [x] 创建 `/admin/users` 用户管理页

### Register Page
- [x] 注册表单添加手机号和注册码输入框
- [x] 手机号格式校验（11位数字，1开头）

### Register Staff Page
- [x] 员工注册表单添加手机号输入框
- [x] 手机号格式校验（11位数字，1开头）

### Profile Page
- [x] 超管用户显示"管理后台"入口

### Validation
- [x] 运行 `pnpm tsc --noEmit` 确保类型正确

## Documentation
- [x] 更新 CLAUDE.md 添加超管相关说明
