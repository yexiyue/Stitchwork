## ADDED Requirements

### Requirement: Biometric Guard

系统 SHALL 提供生物识别守卫组件，用于保护敏感页面。

#### Scenario: 首次访问敏感页面

- **GIVEN** 用户已登录但本次会话未进行生物识别验证
- **WHEN** 用户访问受保护的敏感页面
- **THEN** 显示生物识别验证提示
- **AND** 触发设备生物识别（指纹/面容）

#### Scenario: 验证成功访问页面

- **GIVEN** 用户触发了生物识别验证
- **WHEN** 验证成功
- **THEN** 标记当前会话为已验证
- **AND** 显示敏感页面内容

#### Scenario: 验证失败返回

- **GIVEN** 用户触发了生物识别验证
- **WHEN** 验证失败或用户取消
- **THEN** 导航返回上一页
- **AND** 不显示敏感页面内容

#### Scenario: 已验证会话直接访问

- **GIVEN** 用户本次会话已通过生物识别验证
- **WHEN** 用户访问其他受保护页面
- **THEN** 直接显示页面内容
- **AND** 不再触发生物识别

### Requirement: Biometric Session Timeout

系统 SHALL 在应用长时间后台运行后清除生物识别验证状态。

#### Scenario: 后台超时重新验证

- **GIVEN** 用户已通过生物识别验证
- **WHEN** 应用切换到后台超过 5 分钟后返回
- **THEN** 清除验证状态
- **AND** 下次访问敏感页面需重新验证

#### Scenario: 短暂切换保持验证

- **GIVEN** 用户已通过生物识别验证
- **WHEN** 应用切换到后台不足 5 分钟后返回
- **THEN** 保持验证状态
- **AND** 可直接访问敏感页面

### Requirement: Biometric Fallback

系统 SHALL 在不支持生物识别的环境下提供降级处理。

#### Scenario: 非 Tauri 环境跳过验证

- **GIVEN** 用户在 Web 浏览器中访问（非 Tauri 环境）
- **WHEN** 访问受保护页面
- **THEN** 跳过生物识别验证
- **AND** 直接显示页面内容

#### Scenario: 设备不支持生物识别

- **GIVEN** 设备未配置生物识别（无指纹/面容）
- **WHEN** 访问受保护页面
- **THEN** 跳过生物识别验证
- **AND** 直接显示页面内容

### Requirement: Protected Routes Configuration

系统 SHALL 对以下路由启用生物识别保护：

#### Scenario: Boss 客户管理受保护

- **GIVEN** 用户角色为 Boss
- **WHEN** 访问 `/customers` 或 `/customers/:id`
- **THEN** 需通过生物识别验证

#### Scenario: Boss 工资管理受保护

- **GIVEN** 用户角色为 Boss
- **WHEN** 访问 `/payroll` 或 `/payroll/:id`
- **THEN** 需通过生物识别验证

#### Scenario: Staff 工资查看受保护

- **GIVEN** 用户角色为 Staff
- **WHEN** 访问 `/my-payrolls` 或 `/my-payrolls/:id`
- **THEN** 需通过生物识别验证
