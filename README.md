# StitchWork

服装加工流程管理系统 - 专为服装工坊设计的订单跟踪、计件记录和工资管理平台。

## 功能特性

- **订单管理** - 创建订单、添加工序、跟踪进度、出货结算
- **计件录入** - 员工提交计件、老板审批、自动计算工资
- **工资发放** - 一键生成工资单、历史记录查询
- **实时通知** - SSE 推送，新计件/审批结果即时提醒
- **多端支持** - 移动端 (Android)、Web 浏览器

## 用户角色

| 角色       | 说明                                   |
| ---------- | -------------------------------------- |
| 超级管理员 | 平台级管理，管理注册码和所有用户       |
| 老板       | 工坊管理者，管理订单、工序、员工和工资 |
| 员工       | 工坊工人，查看分配任务、提交计件记录   |

## 技术栈

### 前端

- React 18 + TypeScript + Vite
- TanStack Router / Query
- Ant Design Mobile + Tailwind CSS 4
- Zustand 状态管理

### 移动端

- Tauri 2.0 (Rust)
- 系统通知、二维码扫描、生物识别

### 后端

- Rust + Axum
- SeaORM 2.0 (Entity First)
- PostgreSQL
- JWT 认证 + SSE 实时推送

## 快速开始

### 环境要求

- Node.js 18+, pnpm
- Rust 1.83+
- PostgreSQL

### 安装

```bash
# 克隆项目
git clone https://github.com/yexiyue/Stitchwork.git
cd StitchWork

# 安装前端依赖
pnpm install

# 安装后端依赖
cd server && cargo build
```

### 配置

```bash
# 前端 .env
VITE_API_URL=http://localhost:3000

# 后端 server/.env
DATABASE_URL=postgres://user:pass@localhost/stitchwork
JWT_SECRET=your-secret

# 可选：阿里云 OSS 图片存储
AWS_REGION=oss-cn-hangzhou
AWS_ACCESS_KEY_ID=xxx
AWS_SECRET_ACCESS_KEY=xxx
S3_BUCKET=your-bucket
S3_ENDPOINT=https://oss-cn-hangzhou.aliyuncs.com
```

### 运行

```bash
# 启动后端 (端口 3000)
cd server && cargo run

# 启动前端 (端口 1420)
pnpm dev

# 启动 Android 应用
pnpm tauri android dev
```

## 构建

```bash
# Web 构建
pnpm build

# Android APK/AAB
pnpm tauri android build

# 后端
cd server && cargo build --release
```

## 项目结构

```text
StitchWork/
├── src/                    # React 前端
│   ├── api/                # API 客户端
│   ├── components/         # 共享组件
│   ├── hooks/              # 自定义 Hooks
│   ├── routes/             # 页面路由
│   └── stores/             # 状态管理
├── src-tauri/              # Tauri 移动端
│   └── src/
│       ├── sse.rs          # SSE 实时通知
│       └── image.rs        # 图片压缩
├── server/                 # Axum 后端
│   └── src/
│       ├── entity/         # 数据库实体
│       └── service/        # 业务模块
└── docs/                   # 开发文档
```

## 业务流程

```mermaid
flowchart LR
    A[拿货] --> B[创建订单]
    B --> C[添加工序]
    C --> D[分配任务]
    D --> E[计件录入]
    E --> F[审批]
    F --> G[出货/工资发放]
```

## License

Copyright © 2026. All Rights Reserved.

未经授权，禁止商业使用。如需商业授权，请联系作者。
