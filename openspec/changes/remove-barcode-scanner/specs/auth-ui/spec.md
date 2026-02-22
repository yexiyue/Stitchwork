## REMOVED Requirements

### Requirement: Scan Registration
**Reason**: 注册码已简化为 6 位数字，手动输入即可，扫码功能不再必要。移除可减小 APK 体积。
**Migration**: 用户通过登录页直接输入注册码，或通过 deep link (`stitchwork://register?code=xxx`) 跳转注册页。

## MODIFIED Requirements

### Requirement: Login Page
系统 SHALL 提供登录页面 UI

#### Scenario: 显示登录表单
- **WHEN** 用户访问 `/login`
- **THEN** 显示用户名、密码输入框和登录按钮
- **THEN** 不显示扫码注册入口

#### Scenario: 登录成功跳转
- **WHEN** 用户输入正确凭据并点击登录按钮
- **THEN** 跳转到首页
