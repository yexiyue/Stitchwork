# staff-registration-ui Specification

## Purpose

提供员工注册页面 UI，支持从二维码扫描跳转并完成注册。

## ADDED Requirements

### Requirement: Staff Register Page

系统 SHALL 提供员工注册页面 `/register-staff`

#### Scenario: 显示注册表单
- Given 用户访问 `/register-staff?code=xxx`
- When 页面加载
- Then 显示邀请码（从 URL 参数获取，只读展示）
- And 显示用户名、密码、确认密码输入框
- And 显示注册按钮

#### Scenario: 无邀请码访问
- Given 用户访问 `/register-staff` 无 code 参数
- When 页面加载
- Then 显示错误提示 "请通过扫描邀请码二维码访问"
- And 提供返回登录页的链接

#### Scenario: 注册成功自动登录
- Given 用户填写完整信息
- When 注册成功
- Then 自动保存 token 到 auth store
- And 跳转到首页

#### Scenario: 密码确认校验
- Given 用户输入两次密码
- When 两次密码不一致
- Then 显示错误提示 "两次密码不一致"

### Requirement: Invite QR Code Link

邀请码二维码 SHALL 使用 Web URL 格式

#### Scenario: 二维码内容格式
- Given Boss 生成邀请码
- When 显示二维码
- Then 二维码内容为 Web URL 格式
- And URL 格式为当前域名 + `/register-staff?code=xxx`

## Cross-references

- auth-ui: Login Page, Auth Store
- staff-registration-api: Staff Registration API
