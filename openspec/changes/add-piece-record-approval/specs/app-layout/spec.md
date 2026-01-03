# app-layout Specification

## ADDED Requirements

### Requirement: TabBar Layout

系统 SHALL 为已登录用户提供基于角色的 TabBar 布局。

#### Scenario: Boss 用户看到老板端 TabBar
- **WHEN** Boss 用户登录后访问应用
- **THEN** 显示底部 TabBar 包含：首页、订单、工序、我的

#### Scenario: Staff 用户看到员工端 TabBar
- **WHEN** Staff 用户登录后访问应用
- **THEN** 显示底部 TabBar 包含：首页、工序、记件、我的

### Requirement: Boss Profile Page

系统 SHALL 在老板端"我的"页面提供管理入口。

#### Scenario: 老板查看我的页面
- **WHEN** Boss 用户点击"我的" Tab
- **THEN** 显示页面包含：客户管理入口、员工管理入口、设置、退出登录

### Requirement: Staff Profile Page

系统 SHALL 在员工端"我的"页面提供个人功能入口。

#### Scenario: 员工查看我的页面
- **WHEN** Staff 用户点击"我的" Tab
- **THEN** 显示页面包含：记件记录、设置、退出登录

### Requirement: Pending Approval List

系统 SHALL 为 Boss 用户提供待审核记件列表入口。

#### Scenario: 老板查看待审核列表
- **WHEN** Boss 用户在首页点击待审核入口
- **THEN** 显示所有 `status = pending` 的记件记录列表

#### Scenario: 首页显示待审核数量
- **WHEN** Boss 用户访问首页
- **THEN** 显示待审核记件记录数量（红点或数字徽标）
