## ADDED Requirements

### Requirement: Customer Management Page

老板角色 SHALL 能够通过客户管理页面管理客户信息。

#### Scenario: 访问客户管理页面
- **WHEN** 老板用户访问 `/customers` 路由
- **THEN** 显示客户管理页面
- **THEN** 页面展示客户列表

#### Scenario: 查看客户列表
- **WHEN** 客户管理页面加载完成
- **THEN** 显示当前老板的所有客户
- **THEN** 每个客户项显示名称、电话、备注信息

#### Scenario: 新增客户
- **WHEN** 老板点击新增按钮
- **THEN** 导航到客户表单页面
- **WHEN** 填写客户信息并提交
- **THEN** 创建新客户并返回列表页

#### Scenario: 编辑客户
- **WHEN** 老板点击客户项的编辑操作
- **THEN** 导航到预填充的客户表单页面
- **WHEN** 修改信息并提交
- **THEN** 更新客户信息并返回列表页

#### Scenario: 删除客户
- **WHEN** 老板触发删除操作
- **THEN** 显示确认提示
- **WHEN** 确认删除
- **THEN** 删除客户并刷新列表

### Requirement: Customer Management Entry

首页 SHALL 为老板角色提供客户管理入口。

#### Scenario: 显示客户管理入口
- **WHEN** 老板用户访问首页
- **THEN** 显示客户管理入口卡片
- **WHEN** 点击卡片
- **THEN** 导航到客户管理页面
