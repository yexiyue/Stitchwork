## REMOVED Requirements

### Requirement: Staff Registration Endpoint
**Reason**: 合并到统一注册接口 `POST /api/register`，不再需要独立的 `POST /api/register-staff`
**Migration**: 前端统一调用 `POST /api/register { code, username, password, phone }`

### Requirement: Invite Code Endpoint
**Reason**: 内存邀请码机制被数据库注册码替代，不再需要 `POST /api/invite-code`
**Migration**: Boss 通过 `POST /api/register-codes` 创建注册码

## MODIFIED Requirements

### Requirement: Login Response
登录接口 SHALL 返回完整用户信息

#### Scenario: 登录成功返回用户信息
- Given 用户提交正确的用户名和密码
- When 登录成功
- Then 返回 token 和用户对象
- And 用户对象包含 userId, username, role, displayName, phone, avatar
- And boss 角色额外包含 workshopName, workshopDesc

#### Scenario: 注册成功自动登录
- **WHEN** 用户通过 `POST /api/register` 注册成功
- **THEN** 返回与登录相同格式的 `LoginResponse { token, user }`
- **AND** Boss 和 Staff 注册均自动登录
