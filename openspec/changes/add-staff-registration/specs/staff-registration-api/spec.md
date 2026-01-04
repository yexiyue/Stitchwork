# staff-registration-api Specification

## Purpose

提供员工注册接口，支持通过邀请码注册并自动绑定工坊。

## ADDED Requirements

### Requirement: Staff Registration API

系统 SHALL 提供员工注册接口 `POST /register-staff`

#### Scenario: 员工注册成功
- Given 员工有有效的邀请码
- When 提交注册请求（username, password, inviteCode）
- Then 创建 Staff 角色用户
- And 用户的 workshop_id 设置为邀请码对应的工坊
- And 邀请码从缓存中移除（一次性使用）
- And 返回 token 和用户信息（与登录响应格式一致）

#### Scenario: 邀请码无效
- Given 邀请码不存在或已使用
- When 提交注册请求
- Then 返回错误 "邀请码无效"

#### Scenario: 邀请码已过期
- Given 邀请码已超过 24 小时有效期
- When 提交注册请求
- Then 返回错误 "邀请码已过期"

#### Scenario: 用户名已存在
- Given 用户名已被其他用户注册
- When 提交注册请求
- Then 返回错误 "用户名已存在"
