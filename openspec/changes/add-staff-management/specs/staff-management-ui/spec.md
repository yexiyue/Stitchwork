# staff-management-ui Specification

## Purpose

提供 Boss 用户管理员工的前端界面，支持查看员工列表、创建员工账号、生成邀请码。

## ADDED Requirements

### Requirement: Staff List Page

系统 SHALL 提供员工列表页面供 Boss 查看和管理员工。

#### Scenario: 显示员工列表

- Given Boss 用户访问 `/staff`
- When 页面加载
- Then 显示当前 Boss 下所有员工列表
- And 每个员工显示姓名、用户名、手机号

#### Scenario: 空列表提示

- Given Boss 没有任何员工
- When 访问员工列表
- Then 显示"暂无员工"提示

#### Scenario: 导航到新增页面

- Given Boss 在员工列表页
- When 点击"新增"按钮
- Then 跳转到 `/staff/new`

### Requirement: Create Staff Page

系统 SHALL 提供创建员工账号的表单页面。

#### Scenario: 显示创建表单

- Given Boss 访问 `/staff/new`
- When 页面加载
- Then 显示用户名、密码、姓名、手机号输入框和保存按钮

#### Scenario: 创建成功

- Given Boss 填写完整表单
- When 点击保存
- Then 调用 `authApi.createStaff`
- And 成功后返回员工列表页
- And 显示成功提示

#### Scenario: 用户名已存在

- Given Boss 输入已存在的用户名
- When 提交表单
- Then 显示错误提示"用户名已存在"

### Requirement: Invite Code Feature

系统 SHALL 支持 Boss 生成邀请码供员工绑定。

#### Scenario: 生成邀请码

- Given Boss 在员工列表页
- When 点击"邀请码"按钮
- Then 调用 `authApi.generateInviteCode`
- And 弹窗显示邀请码和有效期

#### Scenario: 复制邀请码

- Given 邀请码弹窗显示
- When 点击复制按钮
- Then 邀请码复制到剪贴板
- And 显示"已复制"提示

### Requirement: Profile Page Integration

系统 SHALL 将员工管理入口集成到 Profile 页面。

#### Scenario: 点击员工管理

- Given Boss 在 Profile 页面
- When 点击"员工管理"
- Then 跳转到 `/staff`
