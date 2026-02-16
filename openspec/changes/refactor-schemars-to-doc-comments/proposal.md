# Proposal: refactor-schemars-to-doc-comments

## Summary

将所有 Rust 文件中的 `#[schemars(description = "...")]` 属性宏改为使用标准文档注释 `///`。`schemars` 库会自动从文档注释生成 JSON Schema 描述，这样更符合 Rust 惯例，代码也更简洁。

## Motivation

1. **一致性**: 文档注释是 Rust 的标准做法，`schemars` 原生支持从 `///` 提取描述
2. **简洁性**: 减少属性宏的使用，代码更清晰
3. **可维护性**: 文档注释同时服务于 rustdoc 和 JSON Schema，一处维护两处受益
4. **IDE 友好**: 文档注释在 IDE 中有更好的提示支持

## Scope

涉及以下文件：

| 文件 | `#[schemars(description)]` 数量 |
|------|-------------------------------|
| `crates/server/src/mcp/boss.rs` | 15 |
| `crates/server/src/mcp/staff.rs` | 12 |
| `crates/server/src/service/workshop/dto.rs` | 7 |
| `crates/server/src/service/auth/dto.rs` | 14 |
| `crates/server/src/service/process/dto.rs` | 5 |
| `crates/server/src/service/admin/dto.rs` | 2 |
| `crates/server/src/service/upload/dto.rs` | 6 |
| `crates/server/src/service/payroll/dto.rs` | 5 |
| `crates/server/src/service/piece_record/dto.rs` | 4 |
| `crates/server/src/service/customer/dto.rs` | 3 |
| `crates/server/src/service/share/dto.rs` | 4 |
| `crates/server/src/service/stats/dto.rs` | 4 |
| `crates/server/src/service/order/dto.rs` | 8 |

**总计**: 89 处需要修改

## Approach

对于每个 `#[schemars(description = "xxx")]`，将其改为该字段上方的文档注释 `/// xxx`。

**Before:**
```rust
#[schemars(description = "用户名或手机号")]
pub username: String,
```

**After:**
```rust
/// 用户名或手机号
pub username: String,
```

## Risks

- **低风险**: 这是纯重构，不改变功能行为
- `schemars` 官方文档确认支持从 `///` 文档注释自动提取描述

## Success Criteria

1. 所有 `#[schemars(description = "...")]` 被替换为 `///` 文档注释
2. `cargo check` 编译通过
3. JSON Schema 输出保持不变（描述内容相同）
