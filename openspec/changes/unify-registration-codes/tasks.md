# Tasks: unify-registration-codes

## Backend

- [x] 1. 修改 `generate_code()` 函数，生成 `B-XXXXXXXX` 格式
- [x] 2. 修改注册验证逻辑，兼容新旧格式
- [x] 3. cargo check 验证

## Frontend

- [x] 4. 合并 `/register.tsx` 和 `/register-staff.tsx` 为统一注册页
- [x] 5. 根据码格式（`B-` 前缀）自动调用对应注册接口
- [x] 6. 支持 `?code=` 参数预填充
- [x] 7. 更新 `main.tsx` deep link 处理，统一使用 `stitchwork://register?code=xxx`
- [x] 8. 删除 `/register-staff.tsx`
- [x] 9. 超管注册码列表添加二维码展示
- [x] 10. pnpm tsc --noEmit 验证
