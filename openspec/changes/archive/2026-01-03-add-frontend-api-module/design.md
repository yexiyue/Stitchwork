## Context

服务端使用 Axum 框架，API 路径统一以 `/api` 为前缀。所有响应格式为：
```json
{ "code": 0, "message": "success", "data": ... }
```

## Goals / Non-Goals

Goals:
- 提供类型安全的 API 请求函数
- 统一错误处理
- 与 TanStack Query 集成

Non-Goals:
- 不涉及 UI 组件开发
- 不涉及离线缓存策略

## Decisions

### 请求库: 原生 fetch

选择原因：
- 无需额外依赖
- 现代浏览器原生支持
- 配合 TanStack Query 足够使用

### 目录结构

```
src/
├── api/
│   ├── client.ts       # 基础请求封装
│   ├── auth.ts         # 认证相关 API
│   ├── customer.ts     # 客户管理 API
│   ├── order.ts        # 订单管理 API
│   ├── process.ts      # 工序管理 API
│   ├── piece-record.ts # 计件记录 API
│   ├── payroll.ts      # 工资管理 API
│   ├── stats.ts        # 统计 API
│   ├── share.ts        # 分享 API
│   └── index.ts        # 统一导出
└── types/
    ├── api.ts          # 通用 API 类型
    ├── auth.ts         # 认证类型
    ├── customer.ts     # 客户类型
    ├── order.ts        # 订单类型
    ├── process.ts      # 工序类型
    ├── piece-record.ts # 计件记录类型
    ├── payroll.ts      # 工资类型
    ├── stats.ts        # 统计类型
    ├── share.ts        # 分享类型
    └── index.ts        # 统一导出
```

### API 端点汇总

| 模块 | 端点 | 方法 |
|------|------|------|
| auth | /api/login | POST |
| auth | /api/register | POST |
| auth | /api/staff | POST |
| auth | /api/invite-code | POST |
| auth | /api/bind-boss | POST |
| auth | /api/profile | PUT |
| customer | /api/customers | GET, POST |
| customer | /api/customers/{id} | GET, PUT, DELETE |
| order | /api/orders | GET, POST |
| order | /api/orders/{id} | GET, PUT, DELETE |
| order | /api/orders/{id}/status | PATCH |
| process | /api/processes | GET, POST |
| process | /api/processes/{id} | GET, PUT, DELETE |
| piece-record | /api/piece-records | GET, POST |
| piece-record | /api/piece-records/{id} | GET, PUT, DELETE |
| payroll | /api/payrolls | GET, POST |
| payroll | /api/payrolls/{id} | GET, PUT, DELETE |
| stats | /api/orders/{id}/stats | GET |
| stats | /api/stats/customers | GET |
| stats | /api/stats/workers | GET |
| share | /api/shares | GET, POST |
| share | /api/shares/{id} | PUT, DELETE |
| share | /api/public/share/{token} | GET |

## Open Questions

无
