# auth-ui Specification

## Purpose
TBD - created by archiving change add-auth-pages. Update Purpose after archive.
## Requirements
### Requirement: Auth Store
系统 SHALL 使用 Zustand store 管理用户认证状态（含角色信息）

#### Scenario: 用户登录成功
- Given 用户在登录页输入正确的用户名和密码
- When 调用 `login()` 方法
- Then token 和用户信息（含 role）保存到 store
- And token 同步到 localStorage

#### Scenario: 获取用户角色
- Given 用户已登录
- When 访问 `isBoss` 属性
- Then 返回用户是否为 boss 角色

#### Scenario: 应用启动恢复状态
- Given localStorage 中存在有效 token
- When 应用启动
- Then 自动恢复认证状态

#### Scenario: 用户登出
- Given 用户已登录
- When 调用 `logout()` 方法
- Then 清除 store 状态和 localStorage

### Requirement: Login Page
系统 SHALL 提供登录页面 UI

#### Scenario: 显示登录表单
- Given 用户访问 `/login`
- When 页面加载
- Then 显示用户名、密码输入框和登录按钮

#### Scenario: 登录成功跳转
- Given 用户输入正确凭据
- When 点击登录按钮
- Then 跳转到首页

### Requirement: Register Page
系统 SHALL 提供注册页面 UI

#### Scenario: 显示注册表单
- Given 用户访问 `/register`
- When 页面加载
- Then 显示用户名、密码、确认密码输入框和注册按钮

#### Scenario: 注册成功跳转
- Given 用户完成注册
- When 注册成功
- Then 跳转到登录页

### Requirement: Route Guard
系统 SHALL 实现路由保护机制

#### Scenario: 未登录访问受保护页面
- Given 用户未登录
- When 访问受保护的路由
- Then 重定向到 `/login`

#### Scenario: 已登录访问登录页
- Given 用户已登录
- When 访问 `/login`
- Then 重定向到首页

### Requirement: Role Based Routing
系统 SHALL 支持根据角色渲染不同路由

#### Scenario: Boss 角色路由
- Given 用户角色为 boss
- When 渲染导航菜单
- Then 显示 boss 专属功能（员工管理、邀请码等）

#### Scenario: Staff 角色路由
- Given 用户角色为 staff
- When 渲染导航菜单
- Then 隐藏 boss 专属功能

