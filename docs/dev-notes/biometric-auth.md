# Tauri 生物识别认证

使用 `tauri-plugin-biometric` 实现移动端生物识别（指纹/面容）认证功能。

## 配置

### 1. Cargo.toml

```toml
[target.'cfg(any(target_os = "android", target_os = "ios"))'.dependencies]
tauri-plugin-biometric = "2"
```

### 2. lib.rs 注册插件

```rust
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_biometric::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

### 3. Tauri Capabilities (`src-tauri/capabilities/mobile.json`)

```json
{
  "platforms": ["android", "iOS"],
  "permissions": [
    "biometric:default",
    "biometric:allow-authenticate",
    "biometric:allow-check-status"
  ]
}
```

### 4. Android 权限 (`AndroidManifest.xml`)

```xml
<uses-permission android:name="android.permission.USE_BIOMETRIC" />
```

## 前端实现

### Hook 封装 (`src/hooks/use-biometric.ts`)

```typescript
import { useCallback, useEffect, useState } from "react";
import { useBiometricStore } from "@/stores/biometric";
import { isTauri } from "@/utils/platform";

interface UseBiometricReturn {
  /** 设备是否支持生物识别 */
  isAvailable: boolean;
  /** 当前会话是否已验证 */
  isVerified: boolean;
  /** 正在检查可用性 */
  isChecking: boolean;
  /** 触发生物识别验证 */
  authenticate: (reason?: string) => Promise<boolean>;
}

export function useBiometric(): UseBiometricReturn {
  const [isAvailable, setIsAvailable] = useState(false);
  const [isChecking, setIsChecking] = useState(true);
  const { verified, setVerified } = useBiometricStore();

  // 检查生物识别是否可用
  useEffect(() => {
    const checkAvailability = async () => {
      if (!isTauri()) {
        // 非 Tauri 环境，自动跳过
        setIsAvailable(false);
        setIsChecking(false);
        return;
      }

      try {
        const { checkStatus } = await import("@tauri-apps/plugin-biometric");
        const status = await checkStatus();
        setIsAvailable(status.isAvailable);
      } catch {
        setIsAvailable(false);
      } finally {
        setIsChecking(false);
      }
    };

    checkAvailability();
  }, []);

  const authenticate = useCallback(
    async (reason = "请验证身份以继续"): Promise<boolean> => {
      // 已验证则直接返回
      if (verified) {
        return true;
      }

      // 非 Tauri 环境或不支持生物识别，自动通过
      if (!isTauri() || !isAvailable) {
        setVerified(true);
        return true;
      }

      try {
        const { authenticate: bioAuth } = await import(
          "@tauri-apps/plugin-biometric"
        );
        await bioAuth(reason, {
          allowDeviceCredential: true, // 允许使用设备密码作为备选
        });
        setVerified(true);
        return true;
      } catch {
        // 用户取消或验证失败
        return false;
      }
    },
    [verified, isAvailable, setVerified]
  );

  return {
    isAvailable,
    isVerified: verified || !isTauri() || (!isChecking && !isAvailable),
    isChecking,
    authenticate,
  };
}
```

### 状态存储 (`src/stores/biometric.ts`)

使用 Zustand 存储验证状态，会话内有效：

```typescript
import { create } from "zustand";

interface BiometricState {
  verified: boolean;
  setVerified: (verified: boolean) => void;
}

export const useBiometricStore = create<BiometricState>((set) => ({
  verified: false,
  setVerified: (verified) => set({ verified }),
}));
```

## 使用场景

### 1. 路由守卫

在需要保护的路由布局中使用：

```tsx
// src/routes/_auth.tsx
import { useBiometric } from "@/hooks/use-biometric";

function AuthLayout() {
  const { isVerified, isChecking, authenticate } = useBiometric();

  // 等待检查完成
  if (isChecking) {
    return <SpinLoading />;
  }

  // 需要验证
  if (!isVerified) {
    return (
      <BiometricGuard onAuthenticate={authenticate} />
    );
  }

  return <Outlet />;
}
```

### 2. 敏感操作前验证

```tsx
function TransferButton() {
  const { authenticate } = useBiometric();

  const handleTransfer = async () => {
    const success = await authenticate("请验证身份以完成转账");
    if (!success) {
      Toast.show({ content: "验证失败", icon: "fail" });
      return;
    }
    // 执行转账...
  };

  return <Button onClick={handleTransfer}>转账</Button>;
}
```

### 3. 生物识别守卫组件

```tsx
// src/components/BiometricGuard.tsx
import { Button } from "antd-mobile";
import { Fingerprint } from "lucide-react";

interface Props {
  onAuthenticate: () => Promise<boolean>;
}

export function BiometricGuard({ onAuthenticate }: Props) {
  return (
    <div className="flex flex-col items-center justify-center h-screen bg-gray-50">
      <Fingerprint size={64} className="text-blue-500 mb-6" />
      <h2 className="text-xl font-medium mb-2">需要身份验证</h2>
      <p className="text-gray-500 mb-6">请使用指纹或面容解锁</p>
      <Button color="primary" onClick={onAuthenticate}>
        验证身份
      </Button>
    </div>
  );
}
```

## API 参考

### checkStatus()

检查生物识别可用性：

```typescript
interface BiometricStatus {
  isAvailable: boolean;
  biometryType: "touchId" | "faceId" | "fingerprint" | "face" | "iris";
  error?: string;
}

const status = await checkStatus();
```

### authenticate(reason, options)

触发生物识别验证：

```typescript
interface AuthenticateOptions {
  /** 允许使用设备密码作为备选 */
  allowDeviceCredential?: boolean;
  /** 取消按钮文字 (Android) */
  cancelTitle?: string;
  /** 确认按钮是否必需 (Android) */
  confirmationRequired?: boolean;
}

await authenticate("请验证身份", {
  allowDeviceCredential: true,
});
```

## 浏览器兼容

在非 Tauri 环境下：

1. `checkStatus()` 会报错，需要 try-catch
2. `isAvailable` 返回 `false`
3. `authenticate()` 自动返回 `true`，跳过验证

使用动态导入确保浏览器环境不会加载 Tauri 模块：

```typescript
// 正确：动态导入
const { checkStatus } = await import("@tauri-apps/plugin-biometric");

// 错误：顶层导入会在浏览器环境报错
import { checkStatus } from "@tauri-apps/plugin-biometric";
```

## 常见问题

### 设备不支持生物识别

`checkStatus()` 返回 `isAvailable: false`，应用应提供备选验证方式或跳过验证。

### 用户未设置生物识别

即使设备支持，用户未在系统设置中注册指纹/面容时，`isAvailable` 也可能为 `false`。

### allowDeviceCredential 的作用

设置为 `true` 时，如果生物识别失败多次，系统会提示用户使用设备密码（PIN/图案/密码）作为备选。

### 验证状态持久化

当前实现中，验证状态仅在内存中保存，应用重启后需要重新验证。如需持久化，可以使用 Zustand 的 persist 中间件。
