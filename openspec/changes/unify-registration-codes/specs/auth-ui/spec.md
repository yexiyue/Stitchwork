# auth-ui Specification

## Purpose

统一注册码入口和使用方式。

## MODIFIED Requirements

### Requirement: Unified Registration Page

系统 SHALL 提供统一的注册页面，根据码格式自动识别注册类型。

#### Scenario: 老板注册码注册

- **GIVEN** 用户在注册页输入 `B-A3K9M2P7`
- **WHEN** 提交注册表单
- **THEN** 系统调用老板注册接口，注册成功后跳转老板首页

#### Scenario: 员工邀请码注册

- **GIVEN** 用户在注册页输入 `a1b2c3d4`
- **WHEN** 提交注册表单
- **THEN** 系统调用员工注册接口，注册成功后跳转员工首页

#### Scenario: URL 参数预填充

- **GIVEN** 用户访问 `/register?code=xxx`
- **THEN** 注册码输入框自动填充该值

### Requirement: Register Code QR Display

系统 SHALL 在超管注册码管理页展示二维码。

#### Scenario: 展示注册码二维码

- **GIVEN** 超管查看注册码列表
- **THEN** 每个注册码显示对应的二维码，扫码后跳转注册页
