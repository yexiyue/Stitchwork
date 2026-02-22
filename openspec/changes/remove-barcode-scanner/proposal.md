## Why

注册码已简化为 6 位数字，用户可以直接手动输入，不再需要扫码注册功能。移除 `tauri-plugin-barcode-scanner` 及其 Android 依赖 `com.google.mlkit:barcode-scanning` 可以显著减小 APK 体积。同时清理前端相关的 QR 码生成/扫描代码，只保留分享页面的二维码展示功能。

## What Changes

- **BREAKING** 移除扫码注册入口（`/scan` 路由及相关 UI）
- 移除 Tauri barcode-scanner 插件（Rust 依赖、JS 依赖、capabilities 配置）
- 移除 Android ML Kit barcode-scanning 依赖（减小 APK 体积）
- 移除前端 `jsqr` 依赖（图片二维码解码）
- 移除注册码管理中的 QR 码生成（`qrcode.react` 在注册码场景的使用）
- 保留分享页面（`/share/$token`）的 `qrcode.react` 二维码展示功能

## Capabilities

### New Capabilities

（无）

### Modified Capabilities

- `auth-ui`: 移除扫码注册入口，登录页不再跳转扫码页面

## Impact

- **Tauri 层**: 移除 `tauri-plugin-barcode-scanner` crate 依赖、`lib.rs` 插件初始化、`mobile.json` capabilities
- **Android 层**: 移除 `build.gradle.kts` 中 `com.google.mlkit:barcode-scanning:17.3.0`
- **前端路由**: 删除 `/scan` 路由
- **前端依赖**: 移除 `@tauri-apps/plugin-barcode-scanner`、`jsqr`；保留 `qrcode.react`
- **前端组件**: 清理登录页扫码入口、注册码管理中的 QR 码生成
- **Deep Link**: `stitchwork://register` 的 deep link 处理保留（不依赖扫码插件）
