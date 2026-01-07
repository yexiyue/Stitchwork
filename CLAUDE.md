# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

StitchWork is a garment workshop management system (服装加工流程管理系统) for tracking orders, piece-work records, and payroll. Full-stack app with Tauri mobile client (Android), React frontend (browser), and Rust backend.

## Environment Requirements

- Node.js 18+, pnpm
- Rust 1.83+
- PostgreSQL

## Development Commands

```bash
# Frontend (React + Vite)
pnpm dev              # Dev server on port 1420
pnpm build            # TypeScript check + Vite build
pnpm tsc --noEmit     # TypeScript check only (no build)

# Android
pnpm tauri android dev    # Run Android app in dev mode
pnpm tauri android build  # Build Android APK/AAB

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
│   ├── hooks/              # Custom hooks (useNotify, etc.)
│   ├── routes/             # TanStack Router file-based routes
│   ├── stores/             # Zustand state (auth, app)
│   └── types/              # TypeScript types
├── src-tauri/              # Tauri mobile wrapper (Android)
│   └── src/
│       ├── sse.rs          # SSE client for realtime notifications
│       └── image.rs        # Image compression + blake3 dedup
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

### Tauri

**Plugins:**

- **notification**: System local notifications (Android uses high-priority channels)
- **os**: Platform detection (`platform()` returns "android", "ios", etc.)
- **deep-link**: Custom URI scheme `stitchwork://` for staff invitation QR codes
- **barcode-scanner**: QR code scanning on mobile (Android/iOS only). Request permission in login page before navigating to scan page.
- **biometric**: Fingerprint/face authentication (Android/iOS only)
- **store**: Persistent key-value storage
- **opener**: Open external URLs

**SSE Client** (`src-tauri/src/sse.rs`): Rust-native SSE client for realtime notifications. Uses `Authorization: Bearer <token>` header. Runs in background, maintains connection when app is backgrounded, sends local notifications and emits events to frontend.

**Browser Compatibility**: The app runs in both Tauri and browser environments. Use `isTauri()` from `@/utils/platform` to detect runtime. Key patterns:

- Dynamic import Tauri modules to avoid browser errors
- `TauriStoreState` falls back to localStorage in browser
- Biometric auth auto-passes in browser
- SSE notifications use native `EventSource` with query parameter auth (`?token=<jwt>`)

### Backend

- **Entity First ORM**: Define entities in `server/src/entity/`, schema auto-syncs on startup via `db.get_schema_registry("stitchwork_server::entity::*").sync(&db)`
- **Service structure**: Each feature has `mod.rs`, `dto.rs`, `controller.rs`, `service.rs`
- **Auth**: JWT tokens, Argon2 password hashing
- **API response format**: `{ code: 0, message: "", data: T }` where code 0 = success
- **Realtime notifications**: SSE endpoint at `GET /api/sse/events`. Supports `Authorization: Bearer <token>` header (Tauri) or `?token=<jwt>` query param (browser). Notifier service uses DashMap + tokio broadcast channels
- **Relation queries**: Choose pattern based on use case:

  - `.load().with()` (ModelEx): 单条记录或不分页列表，直接访问关联
  - `load_one`/`load_many`: 统计聚合、需后续处理的场景
  - HashMap 批量查询: 分页查询后批量加载、按 ID 索引

```rust
// ModelEx 模式 - 推荐用于非分页场景
let records = piece_record::Entity::load()
    .with(user::Entity)                                  // 单个关联
    .with((process::Entity, order::Entity))              // 嵌套关联
    .filter(...)
    .all(db)
    .await?;
r.process.as_ref().map(|p| p.name.clone())               // 直接访问

// 分页场景 - 使用 HashMap 批量加载
let records = query.paginate(db, page_size).fetch_page(page).await?;
let processes: HashMap<Uuid, Model> = process::Entity::find()
    .filter(Column::Id.is_in(ids))
    .all(db).await?.into_iter().map(|p| (p.id, p)).collect();
```

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
# Object Storage (Aliyun OSS via S3-compatible API)
AWS_REGION=
AWS_ACCESS_KEY_ID=
AWS_SECRET_ACCESS_KEY=
S3_BUCKET=
S3_ENDPOINT=  # e.g., https://oss-cn-hangzhou.aliyuncs.com
```

## Business Domain

- **Roles**:
  - **Super Admin (超管)**: Platform-level admin, manages registration codes and all users. Not associated with any workshop. Access via `/admin/*` routes.
  - **Boss (老板)**: Workshop owner, manages orders, processes, staff, and payroll within their workshop.
  - **Staff (员工)**: Workshop employee, views own piece-work records and payroll.
- **Registration**: Boss registration requires a valid registration code (8-char, one-time use). Phone number is required and must be unique. Login supports both username and phone number.
- **Flow**: 拿货 → 创建订单 → 添加工序 → 分配任务 → 计件录入 → 出货/工资发放
- **Entities**: user, workshop, customer, order, process, piece_record, payroll, share, register_code

## Documentation

- `docs/design.md` - System design and business flow
- `docs/database.md` - ER diagrams
- `docs/seaorm-entity-first.md` - SeaORM 2.0 patterns (dense format, schema sync, relations)
- `docs/ui-components.md` - Ant Design Mobile component patterns (Form, Picker, VirtualList)
- `docs/image-upload.md` - Image upload and compression patterns
- `docs/dev-notes/` - Development notes:
  - `realtime-notifications.md` - SSE + local notifications (Tauri & browser)
  - `biometric-auth.md` - Tauri biometric authentication
  - `browser-compatibility.md` - Tauri/browser dual-environment patterns
  - `barcode-scanner.md` - QR code scanning
  - `image-upload-optimization.md` - Tauri Rust image compression + blake3 dedup
  - `tauri-notification.md` - Tauri notification plugin setup
  - `jwt-auth.md` - JWT authentication implementation
  - `entity-refactor-employment.md` - Entity relationship refactoring notes

**Diagrams**: Use Mermaid format (`\`\`\`mermaid`) for all diagrams in documentation.

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