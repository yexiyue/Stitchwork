## Context

当前扫码注册功能涉及多个层次：
- Tauri Rust 层：`tauri-plugin-barcode-scanner` 条件编译依赖
- Android 层：`com.google.mlkit:barcode-scanning:17.3.0`（bundled 模型，约 3-5MB）
- 前端路由：`/scan` 页面（相机扫码 + jsQR 图片识别）
- 前端组件：登录页扫码入口、注册码管理的 QR 码生成
- 前端依赖：`@tauri-apps/plugin-barcode-scanner`、`jsqr`、`qrcode.react`

注册码已简化为 6 位数字，用户手动输入即可，扫码不再是必要入口。

## Goals / Non-Goals

**Goals:**
- 移除 barcode-scanner 插件及 Android ML Kit 依赖，减小 APK 体积
- 移除 `/scan` 路由和相关前端代码
- 移除注册码管理中的 QR 码生成（注册码场景）
- 保留分享页面（`shares/index.tsx` 和 `share/$token.tsx`）的 `qrcode.react` 使用

**Non-Goals:**
- 不移除 `qrcode.react` 包（分享功能仍需要）
- 不修改 deep link 处理逻辑（`stitchwork://register` 仍保留，可能来自外部链接）
- 不修改注册页面本身的功能

## Decisions

### 1. 完全移除 vs 注释掉

选择完全移除代码而非注释。注释掉的代码会增加维护负担，且 git 历史可以追溯。

### 2. 注册码管理的 QR 码处理

移除注册码管理组件中的 QR 码生成弹窗。点击未使用的注册码改为复制到剪贴板（已有 `copyToClipboard` 工具函数）。

### 3. `qrcode.react` 保留策略

`qrcode.react` 仍被分享功能使用（`shares/index.tsx` 的二维码弹窗），不从 `package.json` 移除。

### 4. Deep Link 保留

`main.tsx` 中的 `handleDeepLink` 不依赖 barcode-scanner 插件，保留不动。用户可能从外部浏览器点击 `stitchwork://register?code=xxx` 链接进入。

## Risks / Trade-offs

- [体积收益] ML Kit bundled 模型约 3-5MB，移除后 APK 显著减小 → 直接收益
- [功能降级] 失去扫码注册能力 → 6 位数字码手动输入足够简单，可接受
- [jsqr 移除] 图片识别二维码功能一并移除 → 仅用于注册码场景，不影响其他功能
