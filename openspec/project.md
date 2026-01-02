# Project Context

## Purpose

StitchWork - 服装加工流程管理系统，用于管理从拿货到出货的完整加工流程，包括人员工资和计件管理。

## Tech Stack

### 客户端 (Mobile App)

- Framework: Tauri 2 (Mobile)
- Frontend: React 19, TypeScript 5.8, Vite 7
- Native: Rust

### 服务端

- Runtime: 阿里云 ECS
- Web Framework: Axum
- ORM: SeaORM
- Database: PostgreSQL
- 部署: Docker

## Project Conventions

### Code Style

- TypeScript strict mode
- ES modules (`"type": "module"`)
- React functional components
- Rust 2021 edition

### Architecture Patterns

- 前后端分离架构
- RESTful API
- Tauri IPC for frontend-backend communication

### Testing Strategy

[To be defined]

### Git Workflow

[To be defined]

## Domain Context

### 核心业务流程

1. 拿货 - 从客户处接收订单和原材料
2. 加工 - 分配任务给工人，按件计酬
3. 出货 - 完成加工后交付成品

### 关键实体

- 订单 (Order)
- 工人 (Worker)
- 工序/任务 (Task)
- 计件记录 (PieceRecord)
- 工资结算 (Payroll)

## Important Constraints

- 移动端优先设计
- 离线支持考虑
- 多用户并发操作

## External Dependencies

- Shuttle 云平台
- PostgreSQL 数据库
