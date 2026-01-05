# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

StitchWork is a garment workshop management system (服装加工流程管理系统) for tracking orders, piece-work records, and payroll. Full-stack app with Tauri desktop client, React frontend, and Rust backend.

## Environment Requirements

- Node.js 18+, pnpm
- Rust 1.83+
- PostgreSQL

## Development Commands

```bash
# Frontend (React + Vite)
pnpm dev              # Dev server on port 1420
pnpm build            # TypeScript check + Vite build
pnpm tauri dev        # Run Tauri desktop app in dev mode
pnpm tauri build      # Build desktop app

# Backend (Rust + Axum)
cd server && cargo run    # API server on port 3000
cd server && cargo build  # Build server
cd server && cargo check  # Fast type checking
```

## Architecture

```text
StitchWork/
├── src/                    # React frontend
│   ├── api/                # API client with JWT auth
│   ├── components/         # Shared UI components
│   ├── routes/             # TanStack Router file-based routes
│   ├── stores/             # Zustand state (auth, app)
│   └── types/              # TypeScript types
├── src-tauri/              # Tauri desktop wrapper
└── server/                 # Axum backend
    └── src/
        ├── entity/         # SeaORM entities (Entity First)
        ├── service/        # Feature modules (dto, controller, service)
        └── main.rs         # App entry, schema sync
```

## Key Patterns

### Frontend

- **Routing**: TanStack Router with file-based routes. Layout routes: `_auth.tsx` (requires login), `_boss.tsx`, `_staff.tsx`. Dynamic routes use `$id`. Prefix with `-` to disable a route file.
- **State**: Zustand with persist middleware for auth (`useAuthStore`)
- **Data fetching**: TanStack Query
- **UI**: Ant Design Mobile (Chinese locale), Tailwind CSS 4, lucide-react icons
- **Utilities**: ahooks for React hooks, dayjs for dates, echarts for charts, motion for animations
- **Path alias**: `@/*` → `src/*`

### Backend

- **Entity First ORM**: Define entities in `server/src/entity/`, schema auto-syncs on startup via `db.get_schema_registry("stitchwork_server::entity::*").sync(&db)`
- **Service structure**: Each feature has `mod.rs`, `dto.rs`, `controller.rs`, `service.rs`
- **Auth**: JWT tokens, Argon2 password hashing
- **API response format**: `{ code: 0, message: "", data: T }` where code 0 = success

### API Client

```typescript
// src/api/client.ts - auto-injects JWT Bearer token
client.get<T>(path, params?)
client.post<T>(path, body?)
client.put<T>(path, body?)
client.patch<T>(path, body?)
client.delete<T>(path)
```

## Environment Variables

```bash
# Frontend (.env)
VITE_API_URL=http://localhost:3000

# Backend (server/.env)
DATABASE_URL=postgres://user:pass@localhost/stitchwork
JWT_SECRET=your-secret
# Optional S3
AWS_REGION=
AWS_ACCESS_KEY_ID=
AWS_SECRET_ACCESS_KEY=
S3_BUCKET=
```

## Business Domain

- **Roles**: Boss (老板) manages everything; Staff (员工) views own records
- **Flow**: 拿货 → 创建订单 → 添加工序 → 分配任务 → 计件录入 → 出货/工资发放
- **Entities**: user, workshop, customer, order, process, piece_record, payroll, share

## Documentation

- `docs/design.md` - System design and business flow
- `docs/database.md` - ER diagrams
- `docs/seaorm-entity-first.md` - SeaORM 2.0 patterns (dense format, schema sync, relations)

<!-- OPENSPEC:START -->
## OpenSpec Instructions

These instructions are for AI assistants working in this project.

Always open `@/openspec/AGENTS.md` when the request:

- Mentions planning or proposals (words like proposal, spec, change, plan)
- Introduces new capabilities, breaking changes, architecture shifts, or big performance/security work
- Sounds ambiguous and you need the authoritative spec before coding

Use `@/openspec/AGENTS.md` to learn:

- How to create and apply change proposals
- Spec format and conventions
- Project structure and guidelines

Keep this managed block so 'openspec update' can refresh the instructions.
<!-- OPENSPEC:END -->