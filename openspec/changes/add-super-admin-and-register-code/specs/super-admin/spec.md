# Super Admin Capability

## ADDED Requirements

### Requirement: Super Admin Role
The system SHALL support a super admin role for platform-level management of registration codes and users.

#### Scenario: User has super admin flag
- Given: 用户表存在 `is_super_admin` 字段
- When: 用户登录
- Then: 返回的用户信息包含 `isSuperAdmin` 字段

#### Scenario: Super admin accesses admin API
- Given: 用户 `is_super_admin = true`
- When: 请求 `/api/admin/*` 接口
- Then: 请求成功

#### Scenario: Non-super admin accesses admin API
- Given: 用户 `is_super_admin = false`
- When: 请求 `/api/admin/*` 接口
- Then: 返回 403 Forbidden

### Requirement: Registration Code Management
The system SHALL allow super admins to create, list, and disable registration codes.

#### Scenario: Create registration code
- Given: 超管用户已登录
- When: POST `/api/admin/register-codes`
- Then: 返回新创建的注册码（8位随机字符串）

#### Scenario: List registration codes
- Given: 超管用户已登录
- When: GET `/api/admin/register-codes`
- Then: 返回所有注册码列表，包含使用状态

#### Scenario: Disable registration code
- Given: 超管用户已登录，注册码存在
- When: DELETE `/api/admin/register-codes/:id`
- Then: 注册码被标记为 `is_active = false`

### Requirement: User Management
The system SHALL allow super admins to view all users in the system.

#### Scenario: List users
- Given: 超管用户已登录
- When: GET `/api/admin/users`
- Then: 返回所有用户列表（不含密码）

## MODIFIED Requirements

### Requirement: Phone Number Required
The system SHALL require a unique phone number during user registration.

#### Scenario: Register with phone number
- Given: 用户注册（Boss 或 Staff）
- When: 提交注册信息
- Then: phone 字段必填，否则返回 400 Bad Request

#### Scenario: Phone number uniqueness
- Given: 手机号已被其他用户使用
- When: 使用该手机号注册
- Then: 返回 400 Bad Request "手机号已被使用"

### Requirement: Login with Phone or Username
The system SHALL support login using either username or phone number.

#### Scenario: Login with username
- Given: 用户使用用户名登录
- When: POST `/api/auth/login` with username
- Then: 验证成功后返回 token

#### Scenario: Login with phone number
- Given: 用户使用手机号登录
- When: POST `/api/auth/login` with phone as username field
- Then: 系统按手机号查找用户，验证成功后返回 token

### Requirement: Boss Registration with Code
The system SHALL require a valid registration code for boss registration.

#### Scenario: Register with valid code
- Given: 注册码存在且未使用
- When: POST `/api/auth/register` with `registerCode`
- Then: 注册成功，注册码被标记为已使用

#### Scenario: Register with invalid code
- Given: 注册码不存在或已使用
- When: POST `/api/auth/register` with invalid `registerCode`
- Then: 返回 400 Bad Request "注册码无效"

#### Scenario: Register with disabled code
- Given: 注册码存在但 `is_active = false`
- When: POST `/api/auth/register` with `registerCode`
- Then: 返回 400 Bad Request "注册码已禁用"

#### Scenario: Register without code
- Given: 请求未提供 `registerCode`
- When: POST `/api/auth/register`
- Then: 返回 400 Bad Request "请提供注册码"
