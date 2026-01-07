# Tauri 与浏览器兼容性处理

本项目同时支持 Tauri 桌面/移动端和纯浏览器环境。以下是处理两种环境兼容性的最佳实践。

## 核心原则

1. **动态导入 Tauri 模块** - 避免顶层导入导致浏览器报错
2. **环境检测** - 使用统一的 `isTauri()` 函数判断运行环境
3. **优雅降级** - 浏览器环境提供合理的回退行为

## 环境检测工具

### `src/utils/platform.ts`

使用 Tauri 官方提供的 `isTauri` 函数，通过 `platform.ts` 统一导出：

```typescript
/**
 * 检测是否在 Tauri 环境中
 * 使用 Tauri 官方提供的方法
 */
export { isTauri } from "@tauri-apps/api/core";
```

统一从 `@/utils/platform` 导入，便于管理和未来扩展：

```typescript
// 正确：从统一入口导入
import { isTauri } from "@/utils/platform";

// 也可以：直接从 Tauri 导入
import { isTauri } from "@tauri-apps/api/core";

// 错误：各处重复定义
const isTauri = () => "__TAURI__" in window;
```

## 动态导入模式

### 错误示例：顶层导入

```typescript
// 错误：浏览器环境会报 Module not found 错误
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";

function MyComponent() {
  useEffect(() => {
    invoke("my_command");
  }, []);
}
```

### 正确示例：动态导入

```typescript
import { isTauri } from "@/utils/platform";

function MyComponent() {
  useEffect(() => {
    if (!isTauri()) return;

    const setup = async () => {
      const { invoke } = await import("@tauri-apps/api/core");
      const { listen } = await import("@tauri-apps/api/event");

      await invoke("my_command");
      const unlisten = await listen("my_event", (e) => {
        console.log(e.payload);
      });

      return unlisten;
    };

    let cleanup: (() => void) | undefined;
    setup().then((unlisten) => {
      cleanup = unlisten;
    });

    return () => cleanup?.();
  }, []);
}
```

## 具体场景处理

### 1. 存储 (Zustand + Tauri Store)

使用混合存储类，自动选择 Tauri Store 或 localStorage：

```typescript
// src/stores/tauriStoreState.ts
import type { StateStorage } from "zustand/middleware";
import { isTauri } from "@/utils/platform";

type TauriStore = {
  get: <T>(key: string) => Promise<T | null | undefined>;
  set: (key: string, value: unknown) => Promise<void>;
  delete: (key: string) => Promise<boolean>;
  save: () => Promise<void>;
};

export class TauriStoreState implements StateStorage {
  private store: TauriStore | null = null;
  private prefix: string;

  constructor(public storeName: string) {
    this.prefix = storeName.replace(".json", "");
  }

  async init() {
    if (isTauri()) {
      try {
        const { load } = await import("@tauri-apps/plugin-store");
        this.store = await load(this.storeName);
      } catch {
        console.warn("Tauri store not available, using localStorage");
      }
    }
  }

  async getItem(name: string) {
    if (this.store) {
      return (await this.store.get<string>(name)) || null;
    }
    return localStorage.getItem(`${this.prefix}:${name}`);
  }

  async setItem(name: string, value: string) {
    if (this.store) {
      await this.store.set(name, value);
      await this.store.save();
    } else {
      localStorage.setItem(`${this.prefix}:${name}`, value);
    }
  }

  async removeItem(name: string) {
    if (this.store) {
      await this.store.delete(name);
      await this.store.save();
    } else {
      localStorage.removeItem(`${this.prefix}:${name}`);
    }
  }
}
```

### 2. 通知 (SSE + 本地通知)

浏览器环境跳过 Tauri SSE 和本地通知：

```typescript
// src/hooks/useNotify.ts
import { isTauri } from "@/utils/platform";

export function useNotify() {
  const token = useAuthStore((s) => s.token);

  useEffect(() => {
    // 非 Tauri 环境直接跳过
    if (!token || !isTauri()) return;

    const setup = async () => {
      try {
        const { invoke } = await import("@tauri-apps/api/core");
        const { listen } = await import("@tauri-apps/api/event");
        const {
          isPermissionGranted,
          requestPermission,
          createChannel,
          channels,
          Importance,
        } = await import("@tauri-apps/plugin-notification");

        // 请求通知权限
        let granted = await isPermissionGranted();
        if (!granted) {
          const permission = await requestPermission();
          granted = permission === "granted";
        }

        // 创建通知渠道
        const existingChannels = await channels();
        if (!existingChannels.some((c) => c.id === CHANNEL_ID)) {
          await createChannel({
            id: CHANNEL_ID,
            name: "重要通知",
            importance: Importance.High,
          });
        }

        // 连接 SSE
        await invoke("connect_sse", { apiUrl, token, channelId: CHANNEL_ID });

        // 监听通知事件
        return await listen("notification", (event) => {
          // 处理通知...
        });
      } catch (error) {
        console.error("Failed to setup notifications:", error);
      }
    };

    let unlisten: (() => void) | undefined;
    setup().then((fn) => { unlisten = fn; });

    return () => {
      import("@tauri-apps/api/core")
        .then(({ invoke }) => invoke("disconnect_sse"))
        .catch(() => {});
      unlisten?.();
    };
  }, [token]);
}
```

### 3. 生物识别

浏览器环境自动通过验证：

```typescript
// src/hooks/use-biometric.ts
import { isTauri } from "@/utils/platform";

export function useBiometric() {
  const authenticate = useCallback(async () => {
    // 非 Tauri 环境，自动通过
    if (!isTauri()) {
      setVerified(true);
      return true;
    }

    try {
      const { authenticate: bioAuth } = await import(
        "@tauri-apps/plugin-biometric"
      );
      await bioAuth("请验证身份");
      setVerified(true);
      return true;
    } catch {
      return false;
    }
  }, []);

  return {
    // 非 Tauri 环境或不支持生物识别时，isVerified 为 true
    isVerified: verified || !isTauri() || !isAvailable,
    authenticate,
  };
}
```

### 4. 图片上传

浏览器使用前端压缩，Tauri 使用 Rust 压缩：

```typescript
// src/utils/upload.ts
import { isTauri } from "./platform";
import { useAuthStore } from "@/stores/auth";

export async function uploadImage(file: File | Blob): Promise<string> {
  if (isTauri()) {
    // Tauri: Rust 端压缩 + hash 秒传
    const { invoke } = await import("@tauri-apps/api/core");
    const arrayBuffer = await file.arrayBuffer();
    const imageData = Array.from(new Uint8Array(arrayBuffer));

    const result = await invoke<UploadResult>("upload_image", {
      imageData,
      apiUrl: API_URL,
      token: useAuthStore.getState().token || "",
      options: { maxDimension: 2560, quality: 88 },
    });

    return result.key;
  } else {
    // 浏览器: JS 压缩
    const compressed = await imageCompression(file, {
      maxSizeMB: 0.5,
      maxWidthOrHeight: 1920,
    });

    const { uploadUrl, key } = await uploadApi.presign("image.jpg", "image/jpeg");
    await fetch(uploadUrl!, { method: "PUT", body: compressed });

    return key;
  }
}
```

### 5. Deep Link

浏览器环境跳过 deep link 监听：

```typescript
// src/main.tsx
import { isTauri } from "@/utils/platform";

async function initDeepLink() {
  if (!isTauri()) return;

  try {
    const { onOpenUrl, getCurrent } = await import(
      "@tauri-apps/plugin-deep-link"
    );

    // 检查启动时的 deep link
    const urls = await getCurrent();
    if (urls?.length) {
      handleDeepLink(urls);
    }

    // 监听后续 deep link
    await onOpenUrl(handleDeepLink);
  } catch {
    // deep link 插件不可用
  }
}
```

## 功能对比表

| 功能 | Tauri 环境 | 浏览器环境 |
|------|-----------|-----------|
| 数据存储 | Tauri Store (文件) | localStorage |
| 实时通知 | SSE + 本地通知 | 无 (未来可加 Web Push) |
| 生物识别 | 系统指纹/面容 | 自动跳过 |
| 图片压缩 | Rust (image crate) | JS (browser-image-compression) |
| 秒传 (hash) | 支持 (blake3) | 不支持 |
| Deep Link | 自定义 URL Scheme | 不支持 |
| 二维码扫描 | 原生相机 | 不支持 |

## 调试技巧

### 1. 浏览器环境测试

```bash
# 启动开发服务器（不启动 Tauri）
pnpm dev
```

访问 `http://localhost:1420` 即可在浏览器中测试。

### 2. 检查环境

在浏览器控制台中检查：

```javascript
"__TAURI__" in window  // false = 浏览器, true = Tauri
```

### 3. 模拟 Tauri 环境

临时在浏览器中模拟（仅用于调试）：

```javascript
window.__TAURI__ = {};
```

注意：这只能绕过 `isTauri()` 检测，实际的 Tauri API 调用仍会失败。

## 最佳实践总结

1. **永远使用动态导入** - `await import("@tauri-apps/...")`
2. **优先检测环境** - 在函数开头使用 `if (!isTauri()) return`
3. **提供合理回退** - 浏览器环境应有对应的降级方案
4. **统一工具函数** - 使用 `@/utils/platform` 中的 `isTauri()`
5. **错误边界** - 动态导入使用 try-catch 包裹
