# StitchWork

服装加工流程管理系统 - 用于订单跟踪、计件记录和工资管理。

## 技术栈

- **前端**: React + TypeScript + Vite + TanStack Router/Query + Ant Design Mobile + Tailwind CSS
- **桌面端**: Tauri
- **后端**: Rust + Axum + SeaORM + PostgreSQL

## 快速开始

### 环境要求

- Node.js 18+
- pnpm
- Rust 1.83+
- PostgreSQL

### 安装依赖

```bash
# 前端
pnpm install

# 后端
cd server && cargo build
```

### 配置环境变量

```bash
# 前端 .env
VITE_API_URL=http://localhost:3000

# 后端 server/.env
DATABASE_URL=postgres://user:pass@localhost/stitchwork
JWT_SECRET=your-secret
```

### 启动开发服务

```bash
# 启动后端 (端口 3000)
cd server && cargo run

# 启动前端 (端口 1420)
pnpm dev

# 或启动 Tauri 桌面应用
pnpm tauri dev
```

## 项目结构

```text
StitchWork/
├── src/                    # React 前端
│   ├── api/                # API 客户端
│   ├── components/         # 共享组件
│   ├── routes/             # 路由页面
│   ├── stores/             # Zustand 状态
│   └── types/              # TypeScript 类型
├── src-tauri/              # Tauri 桌面端
└── server/                 # Axum 后端
    └── src/
        ├── entity/         # SeaORM 实体
        └── service/        # 业务模块
```

## 业务流程

拿货 → 创建订单 → 添加工序 → 分配任务 → 计件录入 → 出货/工资发放

## 构建

```bash
# 构建前端
pnpm build

# 构建桌面应用
pnpm tauri build

# 构建后端
cd server && cargo build --release
```

## 部署

项目配置了 GitHub Actions 自动部署，推送到 main 分支时会自动构建并部署到服务器。

详见 `.github/workflows/deploy.yml`

## License

Copyright © 2026. All Rights Reserved.

未经授权，禁止商业使用。如需商业授权，请联系作者。
