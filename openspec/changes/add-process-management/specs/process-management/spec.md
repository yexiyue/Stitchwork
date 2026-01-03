# Process Management

## ADDED Requirements

### Requirement: 订单详情页面展示工序列表

系统 SHALL 在订单详情页面展示该订单的所有工序列表。

#### Scenario: 查看订单工序列表

- Given: Boss 用户已登录
- When: 进入订单详情页面
- Then: 显示该订单的工序列表，包含工序名称、计件单价
- And: 显示工序统计信息（总数、计件总数、总金额）

### Requirement: 新增工序

系统 SHALL 允许 Boss 用户为订单添加新工序。

#### Scenario: 成功新增工序

- Given: Boss 用户在订单详情页面
- When: 点击新增工序按钮
- Then: 弹出新增工序表单
- When: 填写工序名称和计件单价并提交
- Then: 工序创建成功，列表刷新显示新工序

#### Scenario: 工序名称必填

- Given: Boss 用户在新增工序表单
- When: 未填写工序名称直接提交
- Then: 显示"请输入工序名称"错误提示

### Requirement: 编辑工序

系统 SHALL 允许 Boss 用户编辑已有工序的信息。

#### Scenario: 成功编辑工序

- Given: Boss 用户在订单详情页面
- When: 点击工序的编辑按钮
- Then: 弹出编辑工序表单，显示当前工序信息
- When: 修改信息并提交
- Then: 工序更新成功，列表刷新显示更新后的信息

### Requirement: 删除工序

系统 SHALL 允许 Boss 用户删除工序。

#### Scenario: 成功删除工序

- Given: Boss 用户在订单详情页面
- When: 点击工序的删除按钮
- Then: 弹出确认对话框
- When: 确认删除
- Then: 工序删除成功，从列表中移除

### Requirement: 搜索工序

系统 SHALL 允许 Boss 用户按名称搜索工序。

#### Scenario: 按名称搜索

- Given: Boss 用户在订单详情页面
- When: 在搜索框输入工序名称关键字
- Then: 列表过滤显示匹配的工序
