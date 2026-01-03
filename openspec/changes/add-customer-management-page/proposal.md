# Change: Add Customer Management Page for Boss

## Why
老板角色需要一个客户管理页面来查看、添加、编辑和删除客户信息。后端 API 和前端 hooks 已存在，但缺少 UI 页面。

## What Changes
- 新增 `/_auth/_boss/customers.tsx` 客户管理页面
- 页面包含客户列表展示、新增、编辑、删除功能
- 在首页添加客户管理入口卡片

## Impact
- Affected specs: frontend-ui (新增)
- Affected code: `src/routes/_auth/_boss/customers.tsx`, `src/routes/_auth/index.tsx`
