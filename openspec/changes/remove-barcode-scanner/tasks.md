## 1. 移除 Tauri 插件依赖

- [x] 1.1 从 `src-tauri/Cargo.toml` 移除 `tauri-plugin-barcode-scanner` 依赖
- [x] 1.2 从 `src-tauri/src/lib.rs` 移除 barcode-scanner 插件初始化代码
- [x] 1.3 从 `src-tauri/capabilities/mobile.json` 移除所有 `barcode-scanner:*` 权限

## 2. 移除 Android 依赖

- [x] 2.1 从 `src-tauri/gen/android/app/build.gradle.kts` 移除 `com.google.mlkit:barcode-scanning` 依赖

## 3. 移除前端扫码路由和依赖

- [x] 3.1 删除 `src/routes/scan.tsx` 扫码页面
- [x] 3.2 从 `package.json` 移除 `@tauri-apps/plugin-barcode-scanner` 和 `jsqr` 依赖
- [x] 3.3 运行 `pnpm install` 更新 lockfile

## 4. 清理前端组件

- [x] 4.1 从 `src/routes/login.tsx` 移除扫码入口按钮及相关 barcode-scanner import
- [x] 4.2 从 `src/components/register-code-manager.tsx` 移除 QR 码生成功能（`QRCodeSVG` 和 `showCodeQR`），改为复制注册码到剪贴板

## 5. 验证

- [x] 5.1 运行 `pnpm tsc --noEmit` 确认无 TypeScript 错误
- [x] 5.2 确认分享页面（`shares/index.tsx`、`share/$token.tsx`）的 `qrcode.react` 使用不受影响
