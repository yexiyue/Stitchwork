# Proposal: unify-registration-codes

## Summary

统一老板注册码和员工邀请码的使用方式，合并为一个注册页面，支持二维码扫描和手动输入。

## Motivation

当前老板注册码只能手动输入，员工邀请码只能扫码，且有两个独立的注册页面。统一后：
- 一个注册页面，根据码格式自动调用对应接口
- 老板注册码也可以生成二维码分享
- 员工也可以手动输入邀请码注册

## Design

### 码格式区分

| 类型 | 格式 | 长度 | 示例 |
|------|------|------|------|
| 老板注册码 | `B-XXXXXXXX` | 10位 | `B-A3K9M2P7` |
| 员工邀请码 | `xxxxxxxx` | 8位 | `a1b2c3d4` |

### 统一注册页面

`/register?code=xxx`（code 可选）：
- 输入码后自动检测格式
- `B-` 前缀 → 调用老板注册接口
- 其他 → 调用员工注册接口
- 注册成功后根据角色跳转

### Deep Link

统一使用 `stitchwork://register?code=xxx`

## Scope

### Backend

1. 修改注册码生成逻辑，加 `B-` 前缀
2. 兼容旧格式注册码

### Frontend

1. 合并 `/register` 和 `/register-staff` 为统一注册页
2. 根据码格式自动调用对应接口
3. 更新 deep link 处理
4. 超管注册码管理页添加二维码展示

## Out of Scope

- 旧格式注册码迁移（保持兼容）
