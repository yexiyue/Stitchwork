# Tauri 二维码扫描

使用 `tauri-plugin-barcode-scanner` 实现移动端二维码扫描功能。

## 配置

### 1. Cargo.toml

```toml
[target.'cfg(any(target_os = "android", target_os = "ios"))'.dependencies]
tauri-plugin-barcode-scanner = "2"
```

### 2. lib.rs 注册插件

```rust
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_barcode_scanner::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

### 3. Tauri Capabilities (`src-tauri/capabilities/mobile.json`)

```json
{
  "platforms": ["android", "iOS"],
  "permissions": [
    "barcode-scanner:default",
    "barcode-scanner:allow-request-permissions",
    "barcode-scanner:allow-check-permissions",
    "barcode-scanner:allow-scan",
    "barcode-scanner:allow-cancel",
    "barcode-scanner:allow-vibrate",
    "barcode-scanner:allow-open-app-settings"
  ]
}
```

### 4. Android 权限 (`AndroidManifest.xml`)

```xml
<uses-permission android:name="android.permission.CAMERA" />
<uses-feature android:name="android.hardware.camera" android:required="false" />
```

### 5. ML Kit 依赖 (`build.gradle.kts`)

默认使用 Google Play Services 动态下载 ML Kit 模块，首次扫描需要等待下载。

打包到 APK 避免运行时下载：

```kotlin
dependencies {
    implementation("com.google.mlkit:barcode-scanning:17.3.0")
}
```

## 前端实现

### 权限检查与请求

```typescript
import {
  checkPermissions,
  requestPermissions,
} from "@tauri-apps/plugin-barcode-scanner";

let permission = await checkPermissions();
if (permission !== "granted") {
  permission = await requestPermissions();
  if (permission !== "granted") {
    Toast.show({ content: "需要相机权限才能扫码", icon: "fail" });
    return;
  }
}
```

### 扫描模式

#### Windowed 模式（相机在 WebView 上层）

```typescript
const result = await scan({ formats: [Format.QRCode], windowed: true });
```

- 相机画面覆盖在 WebView 上方
- 适合需要自定义 UI 覆盖层的场景

#### 非 Windowed 模式（相机在 WebView 下层）

```typescript
// 设置背景透明让相机画面显示
document.body.style.background = "transparent";

const result = await scan({ formats: [Format.QRCode] });

// 恢复背景
document.body.style.background = "";
```

- 相机画面在 WebView 后面
- 需要设置 WebView 背景透明
- 可以在 WebView 中绘制扫描框 UI

### 取消扫描

```typescript
import { cancel } from "@tauri-apps/plugin-barcode-scanner";

// 用户点击取消按钮
await cancel();

// 组件卸载时清理
useEffect(() => {
  return () => {
    cancel().catch(() => {});
  };
}, []);
```

## 完整示例

```tsx
// src/routes/scan.tsx
import { createFileRoute, useNavigate } from "@tanstack/react-router";
import { useEffect, useRef, useState } from "react";
import { Toast, SpinLoading } from "antd-mobile";
import { X } from "lucide-react";
import {
  scan,
  Format,
  checkPermissions,
  requestPermissions,
  cancel,
} from "@tauri-apps/plugin-barcode-scanner";

export const Route = createFileRoute("/scan")({
  component: ScanPage,
});

function ScanPage() {
  const navigate = useNavigate();
  const scanning = useRef(false);
  const [status, setStatus] = useState<"loading" | "scanning">("loading");

  useEffect(() => {
    const startScan = async () => {
      if (scanning.current) return;
      scanning.current = true;

      try {
        // 检查并请求相机权限
        let permission = await checkPermissions();
        if (permission !== "granted") {
          permission = await requestPermissions();
          if (permission !== "granted") {
            Toast.show({ content: "需要相机权限才能扫码", icon: "fail" });
            navigate({ to: "/login" });
            return;
          }
        }

        setStatus("scanning");
        document.body.style.background = "transparent";

        const result = await scan({ formats: [Format.QRCode] });

        document.body.style.background = "";

        if (result?.content) {
          // 处理扫描结果
          const url = new URL(result.content);
          // ...
        }
      } catch (e) {
        document.body.style.background = "";
        if (e instanceof Error && !e.message.includes("cancel")) {
          Toast.show({ content: "扫码失败", icon: "fail" });
        }
      }
    };

    startScan();
    return () => { cancel().catch(() => {}); };
  }, [navigate]);

  return (
    <div className={`fixed inset-0 ${status === "loading" ? "bg-black" : "bg-transparent"}`}>
      {status === "loading" ? (
        <div className="flex items-center justify-center h-full">
          <SpinLoading color="white" />
        </div>
      ) : (
        <div className="absolute inset-0 flex items-center justify-center">
          {/* 扫描框 UI */}
          <div className="relative w-64 h-64">
            <div className="absolute top-0 left-0 w-8 h-8 border-t-4 border-l-4 border-white rounded-tl-lg" />
            <div className="absolute top-0 right-0 w-8 h-8 border-t-4 border-r-4 border-white rounded-tr-lg" />
            <div className="absolute bottom-0 left-0 w-8 h-8 border-b-4 border-l-4 border-white rounded-bl-lg" />
            <div className="absolute bottom-0 right-0 w-8 h-8 border-b-4 border-r-4 border-white rounded-br-lg" />
            <div className="absolute top-0 left-0 right-0 h-0.5 bg-green-500 animate-scan" />
          </div>
        </div>
      )}
    </div>
  );
}
```

### 扫描线动画 CSS

```css
@keyframes scan {
  0% { top: 0; }
  50% { top: calc(100% - 2px); }
  100% { top: 0; }
}

.animate-scan {
  animation: scan 2s ease-in-out infinite;
}
```

## 支持的格式

```typescript
enum Format {
  QRCode,
  UPC_A,
  UPC_E,
  EAN8,
  EAN13,
  Code39,
  Code93,
  Code128,
  Codabar,
  ITF,
  Aztec,
  DataMatrix,
  PDF417,
}
```

## 常见问题

### 首次扫描很慢

默认使用 Google Play Services 动态下载 ML Kit 模块。解决方案：
- 添加 `com.google.mlkit:barcode-scanning` 依赖打包到 APK

### 相机画面不显示

- 检查是否设置了 `document.body.style.background = "transparent"`
- 检查 WebView 是否有不透明背景色

### 扫描结果为空

- 检查 `formats` 参数是否包含目标格式
- 确保二维码清晰、光线充足
