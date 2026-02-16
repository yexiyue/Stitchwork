# admin-api Specification

## Purpose

超级管理员 API，提供平台级别的统计数据和管理功能。

## ADDED Requirements

### Requirement: Admin Statistics API

系统 SHALL 提供超管统计数据接口。

#### Scenario: 获取平台统计概览

- **WHEN** 超管请求 `GET /api/admin/stats`
- **THEN** 返回用户统计、工坊统计、注册码统计和平台活跃度数据

### Requirement: Paginated User List

系统 SHALL 支持分页查询用户列表。

#### Scenario: 分页获取用户列表

- **WHEN** 超管请求 `GET /api/admin/users?page=1&pageSize=20`
- **THEN** 返回 `{ list: UserListItem[], total: number }` 格式的分页数据

### Requirement: Paginated Register Code List

系统 SHALL 支持分页查询注册码列表。

#### Scenario: 分页获取注册码列表

- **WHEN** 超管请求 `GET /api/admin/register-codes?page=1&pageSize=20`
- **THEN** 返回 `{ list: RegisterCode[], total: number }` 格式的分页数据
