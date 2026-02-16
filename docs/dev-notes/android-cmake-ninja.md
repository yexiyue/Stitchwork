# Android 构建：CMake 必须使用 Ninja

在 Windows 上构建 Tauri Android 应用时，如果依赖了需要 CMake 编译的 crate（如 `aws-lc-sys`），必须指定使用 Ninja 作为 CMake 生成器。

## 问题现象

运行 `pnpm tauri android dev` 时报错：

```
CMake Error at CMakeLists.txt:6 (project):
  Failed to run MSBuild command:
    C:/Program Files (x86)/Microsoft Visual Studio/2022/BuildTools/MSBuild/Current/Bin/amd64/MSBuild.exe
  to get the value of VCTargetsPath:
    error : 没有为项目 'VCTargetsPath.vcxproj' 设置 BaseOutputPath/OutputPath 属性...
```

## 原因分析

CMake 在 Windows 上默认使用 MSBuild (Visual Studio) 作为生成器，但 MSBuild：

- 只能调用 MSVC 编译器 (cl.exe)
- 只能生成 Windows 目标
- 不支持 Android NDK 工具链

流程对比：

```
MSBuild 路径（失败）:
CMake → 生成 .vcxproj → MSBuild → 调用 cl.exe → ❌ 无法编译 Android

Ninja 路径（成功）:
CMake → 生成 build.ninja → Ninja → 调用 clang (NDK) → ✅ 编译 Android
```

Ninja 是通用构建执行器，不绑定特定编译器，CMake 指定用什么编译器它就用什么。

## 解决方案

### 1. 安装 Ninja

从 [GitHub Releases](https://github.com/nickel-lang/nickel/releases) 下载，或使用包管理器：

```powershell
# Scoop
scoop install ninja

# Chocolatey
choco install ninja
```

确保 `ninja.exe` 在 PATH 中：

```powershell
ninja --version
# 1.13.2
```

### 2. 设置环境变量

临时设置（当前终端）：

```powershell
$env:CMAKE_GENERATOR = "Ninja"
pnpm tauri android dev
```

永久设置（推荐）：

```powershell
[Environment]::SetEnvironmentVariable("CMAKE_GENERATOR", "Ninja", "User")
```

或在系统环境变量中添加：`CMAKE_GENERATOR=Ninja`

### 3. 清理旧缓存（如果之前用 MSBuild 构建过）

如果之前已经用 MSBuild 生成过 CMake 缓存，会报错：

```
CMake Error: Error: generator : Ninja
Does not match the generator used previously: Visual Studio 17 2022
```

清理 Android target 目录：

```powershell
Remove-Item -Recurse -Force "target\aarch64-linux-android" -ErrorAction SilentlyContinue
Remove-Item -Recurse -Force "target\armv7-linux-androideabi" -ErrorAction SilentlyContinue
Remove-Item -Recurse -Force "target\i686-linux-android" -ErrorAction SilentlyContinue
Remove-Item -Recurse -Force "target\x86_64-linux-android" -ErrorAction SilentlyContinue
```

## 涉及的 Crate

常见需要 CMake 的 crate：

- `aws-lc-sys` - AWS 的 TLS 库（reqwest 默认使用）
- `ring` - 密码学库
- `openssl-sys` - OpenSSL 绑定

## 参考

- [Ninja Build System](https://ninja-build.org/)
- [CMake Generators](https://cmake.org/cmake/help/latest/manual/cmake-generators.7.html)
