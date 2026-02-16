## MODIFIED Requirements

### Requirement: Register Page
系统 SHALL 提供统一注册页面 UI，接受 6 位数字注册码完成注册。

#### Scenario: 显示注册表单
- **WHEN** 用户访问 `/register`
- **THEN** 显示注册码、用户名、密码、手机号输入框和注册按钮

#### Scenario: URL 参数预填注册码
- **WHEN** 用户通过 `/register?code=123456` 或 deep link `stitchwork://register?code=123456` 访问
- **THEN** 注册码字段自动填充

#### Scenario: 注册成功自动登录跳转
- **WHEN** 用户注册成功
- **THEN** 自动登录并跳转到首页（Boss 和 Staff 统一行为）

### Requirement: Role Based Routing
系统 SHALL 支持根据角色渲染不同路由

#### Scenario: Boss 角色路由
- Given 用户角色为 boss
- When 渲染导航菜单
- Then 显示 boss 专属功能（员工管理、注册码管理等）

#### Scenario: Staff 角色路由
- Given 用户角色为 staff
- When 渲染导航菜单
- Then 隐藏 boss 专属功能

## ADDED Requirements

### Requirement: Register Code Manager Component
系统 SHALL 提供可复用的注册码管理组件 `RegisterCodeManager`，支持创建、列表展示、删除和 QR 码生成。

#### Scenario: 创建注册码
- **WHEN** 用户点击创建按钮
- **THEN** 调用 API 创建新码
- **AND** 弹窗展示 QR 码和 6 位数字码
- **AND** 支持复制码到剪贴板

#### Scenario: 展示注册码列表
- **WHEN** 组件加载
- **THEN** 以虚拟滚动列表展示注册码
- **AND** 每项显示码值、状态标签（可用/已使用）、创建时间
- **AND** 已使用的码显示使用者用户名

#### Scenario: 删除未使用的码
- **WHEN** 用户左滑可用码并点击删除
- **THEN** 确认后调用 API 删除该码
- **AND** 已使用的码不显示删除操作

### Requirement: Boss Register Code Page
系统 SHALL 为 Boss 角色提供独立的注册码管理页面 `/_auth/_boss/register-codes`。

#### Scenario: 从员工管理页进入
- **WHEN** Boss 在员工管理页点击注册码图标
- **THEN** 跳转到注册码管理页面

#### Scenario: 从个人中心进入
- **WHEN** Boss 在个人中心点击「注册码管理」菜单项
- **THEN** 跳转到注册码管理页面

### Requirement: Admin Register Code Page Update
超管的注册码管理页面 SHALL 改用共享的 `RegisterCodeManager` 组件。

#### Scenario: 超管管理注册码
- **WHEN** 超管访问注册码管理页面
- **THEN** 展示所有 Boss 注册码（通过 `RegisterCodeManager` 组件）
- **AND** 功能与之前一致：创建、列表、删除、QR 码
