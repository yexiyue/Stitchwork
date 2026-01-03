# auth-api Specification

## Purpose
TBD - created by archiving change add-auth-pages. Update Purpose after archive.
## Requirements
### Requirement: Login Response
登录接口 SHALL 返回完整用户信息

#### Scenario: 登录成功返回用户信息
- Given 用户提交正确的用户名和密码
- When 登录成功
- Then 返回 token 和用户对象
- And 用户对象包含 userId, username, role, displayName, phone, avatar
- And boss 角色额外包含 workshopName, workshopDesc

