# Tasks: Add Biometric Guard

## 1. Tauri 层

- [x] 1.1 确认 `tauri-plugin-biometric` 已在 Cargo.toml 配置（仅移动端）
- [x] 1.2 确认 `lib.rs` 已初始化 biometric 插件
- [x] 1.3 配置 Android 权限 `USE_BIOMETRIC` 和 `USE_FINGERPRINT`

## 2. 前端基础设施

- [x] 2.1 安装前端依赖 `@tauri-apps/plugin-biometric`
- [x] 2.2 创建 `src/hooks/use-biometric.ts` 封装生物识别 API
  - 检测是否支持生物识别
  - 执行认证并返回结果
  - 非 Tauri 环境返回已认证状态
- [x] 2.3 创建 `src/stores/biometric.ts` 管理认证状态
  - 会话级存储（不持久化）
  - 支持超时自动失效

## 3. 守卫组件

- [x] 3.1 创建 `src/components/BiometricGuard.tsx`
  - 检查认证状态
  - 未认证时触发生物识别
  - 认证失败时导航回退
  - 显示认证中状态

## 4. 集成敏感路由

- [x] 4.1 在 `_auth/_boss/customers/index.tsx` 添加 `BiometricGuard`
- [x] 4.2 在 `_auth/_boss/customers/$id.tsx` 添加 `BiometricGuard`
- [x] 4.3 在 `_auth/_boss/payroll/index.tsx` 添加 `BiometricGuard`
- [x] 4.4 在 `_auth/_boss/payroll/$id.tsx` 添加 `BiometricGuard`
- [x] 4.5 在 `_auth/_staff/my-payrolls/index.tsx` 添加 `BiometricGuard`
- [x] 4.6 在 `_auth/_staff/my-payrolls/$id.tsx` 添加 `BiometricGuard`

## 5. 后台超时处理

- [x] 5.1 创建 `src/hooks/use-biometric-timeout.ts` 监听 document visibility change
- [x] 5.2 在 `_auth.tsx` 布局组件中启用后台超时处理
- [x] 5.3 应用切换到后台超过 5 分钟后清除认证状态

## 6. 测试验证

- [ ] 6.1 Android 设备测试生物识别流程
- [ ] 6.2 验证非 Tauri 环境正常跳过
- [ ] 6.3 验证后台超时重新验证
