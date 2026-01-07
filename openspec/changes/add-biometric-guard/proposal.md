# Change: Add Biometric Guard for Sensitive Data

## Why

敏感数据（客户信息、工资记录）需要额外的安全保护。当设备被他人使用时，即使已登录也应阻止访问这些数据。生物识别认证（指纹/面容）提供便捷的二次验证方式。

## What Changes

- 添加 `useBiometric` hook 封装 Tauri 生物识别插件
- 添加 `BiometricGuard` 组件用于保护敏感路由
- 添加 `useBiometricStore` 管理认证状态（会话级缓存）
- 在敏感页面入口添加生物识别验证

## Impact

- Affected specs: auth-ui
- Affected code:
  - `src/hooks/useBiometric.ts` (新增)
  - `src/stores/biometricStore.ts` (新增)
  - `src/components/BiometricGuard.tsx` (新增)
  - `src/routes/_auth/_boss/customers/*` (添加守卫)
  - `src/routes/_auth/_boss/payroll/*` (添加守卫)
  - `src/routes/_auth/_staff/my-payrolls/*` (添加守卫)

## Protected Routes

| 角色 | 路由 | 敏感数据 |
|------|------|----------|
| Boss | `/customers/*` | 客户联系方式 |
| Boss | `/payroll/*` | 员工工资明细 |
| Staff | `/my-payrolls/*` | 个人工资记录 |

## Biometric Behavior

1. **首次访问敏感页面**：弹出生物识别验证
2. **验证成功**：标记当前会话已验证，后续访问不再重复验证
3. **验证失败/取消**：返回上一页
4. **应用切换到后台超时**：清除验证状态，下次需重新验证
5. **非移动端**：跳过生物识别（仅 Tauri mobile 支持）
