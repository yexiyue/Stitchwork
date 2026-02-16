# Tasks: refactor-schemars-to-doc-comments

## Task List

- [x] 1. 修改 `crates/server/src/mcp/boss.rs` (15 处)
- [x] 2. 修改 `crates/server/src/mcp/staff.rs` (12 处)
- [x] 3. 修改 `crates/server/src/service/auth/dto.rs` (14 处)
- [x] 4. 修改 `crates/server/src/service/order/dto.rs` (8 处)
- [x] 5. 修改 `crates/server/src/service/workshop/dto.rs` (7 处)
- [x] 6. 修改 `crates/server/src/service/upload/dto.rs` (6 处)
- [x] 7. 修改 `crates/server/src/service/process/dto.rs` (5 处)
- [x] 8. 修改 `crates/server/src/service/payroll/dto.rs` (5 处)
- [x] 9. 修改 `crates/server/src/service/piece_record/dto.rs` (4 处)
- [x] 10. 修改 `crates/server/src/service/stats/dto.rs` (4 处)
- [x] 11. 修改 `crates/server/src/service/share/dto.rs` (4 处)
- [x] 12. 修改 `crates/server/src/service/customer/dto.rs` (3 处)
- [x] 13. 修改 `crates/server/src/service/admin/dto.rs` (2 处)
- [x] 14. 运行 `cargo check` 验证编译通过

## Notes

- 所有任务可并行执行（无依赖关系）
- 每个文件的修改模式相同：`#[schemars(description = "xxx")]` → `/// xxx`
- 注意保持字段的 `#[serde(...)]` 等其他属性不变

## Completion

✅ 所有任务已完成，`cargo check` 编译通过。
