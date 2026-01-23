# 使用 Rust Axum + SeaORM 实现 assistant-ui 自定义会话持久化

在构建 AI 聊天应用时，会话（Thread）持久化是一个常见需求。本文将详细介绍如何使用 Rust Axum + SeaORM 作为后端，结合 `@assistant-ui/react` 的 Custom Thread List 功能，实现完整的会话管理系统。

## 架构概览

```mermaid
flowchart TB
    subgraph Frontend["Frontend (React)"]
        direction TB
        Runtime[useRemoteThreadListRuntime]
        ThreadAdapter[ThreadListAdapter<br/>会话列表管理]
        HistoryAdapter[HistoryAdapter<br/>消息历史管理]
        ChatUI[Thread UI 组件]

        Runtime --> ThreadAdapter
        Runtime --> HistoryAdapter
        Runtime --> ChatUI
    end

    subgraph Backend["Backend (Rust Axum)"]
        direction TB
        Controller[Controller 层<br/>路由处理]
        Service[Service 层<br/>业务逻辑]
        Entity[Entity 层<br/>SeaORM 模型]
        DB[(PostgreSQL)]

        Controller --> Service --> Entity --> DB
    end

    subgraph API["RESTful API"]
        ThreadsAPI["/api/threads"]
        MessagesAPI["/api/threads/{id}/messages"]
    end

    ThreadAdapter <-->|HTTP| ThreadsAPI
    HistoryAdapter <-->|HTTP| MessagesAPI
    ThreadsAPI --> Controller
    MessagesAPI --> Controller
```

## 核心概念

### 1. 为什么需要 Custom Thread List？

| 场景 | 默认行为 | Custom Thread List |
| ---- | -------- | ------------------ |
| 会话存储 | 仅内存，刷新丢失 | 持久化到数据库 |
| 多设备同步 | 不支持 | 支持 |
| 会话归档 | 不支持 | 支持 |
| 标题管理 | 不支持 | 支持自动/手动生成 |

### 2. 两个核心适配器

assistant-ui 的 Custom Thread List 需要实现两个适配器：

```mermaid
flowchart LR
    subgraph ThreadListAdapter["RemoteThreadListAdapter"]
        list[list<br/>获取会话列表]
        initialize[initialize<br/>创建新会话]
        rename[rename<br/>重命名会话]
        archive[archive<br/>归档会话]
        delete[delete<br/>删除会话]
    end

    subgraph HistoryAdapter["ThreadHistoryAdapter"]
        load[load<br/>加载历史消息]
        append[append<br/>保存新消息]
    end
```

## 后端实现

### 第一步：定义 Entity 模型

使用 SeaORM 2.0 的 Entity First 模式定义数据模型。

#### 会话模型 (chat_thread)

```rust
// crates/entity/src/chat_thread.rs
use schemars::JsonSchema;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

/// 聊天会话线程，用于组织一组相关的聊天消息
#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
#[sea_orm(table_name = "chat_thread")]
pub struct Model {
    /// 会话唯一标识符
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,

    /// 会话标题，可由用户设置或根据首条消息自动生成
    pub title: Option<String>,

    /// 会话创建时间
    #[sea_orm(default_expr = "Expr::current_timestamp()")]
    pub created_at: DateTimeUtc,

    /// 会话最后更新时间，每次新增消息时自动更新
    #[sea_orm(auto_update, default_expr = "Expr::current_timestamp()")]
    pub updated_at: DateTimeUtc,

    /// 会话所属用户的 ID
    pub user_id: Uuid,

    /// 会话是否归档，默认为 false
    #[sea_orm(default_value = false)]
    pub archived: bool,

    // 关联关系定义
    #[serde(skip)]
    #[sea_orm(belongs_to, from = "user_id", to = "id", on_delete = "Cascade")]
    pub user: HasOne<super::user::Entity>,

    #[serde(skip)]
    #[sea_orm(has_many)]
    pub messages: HasMany<super::chat_message::Entity>,
}

impl ActiveModelBehavior for ActiveModel {}
```

#### 消息模型 (chat_message)

```rust
// crates/entity/src/chat_message.rs
use schemars::JsonSchema;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

/// 聊天消息，存储单条用户或 AI 助手的消息内容
#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
#[sea_orm(table_name = "chat_message")]
pub struct Model {
    /// 消息唯一标识符（使用前端生成的消息 ID）
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,

    /// 所属会话线程的 ID
    pub thread_id: Uuid,

    /// 父消息 ID（用于消息链表结构）
    pub parent_id: Option<String>,

    /// 消息格式（如 "ai-sdk-ui"、"thread-message"）
    pub format: String,

    /// 消息内容，JSON 格式存储编码后的消息
    #[sea_orm(column_type = "JsonBinary")]
    pub content: serde_json::Value,

    /// 消息创建时间
    #[sea_orm(default_expr = "Expr::current_timestamp()")]
    pub created_at: DateTimeUtc,

    // 关联关系
    #[serde(skip)]
    #[sea_orm(belongs_to, from = "thread_id", to = "id", on_delete = "Cascade")]
    pub thread: HasOne<super::chat_thread::Entity>,
}

impl ActiveModelBehavior for ActiveModel {}
```

> **设计要点**：
>
> - `id` 使用 `String` 类型，直接使用前端生成的消息 ID，避免 ID 冗余
> - `parent_id` 和 `format` 作为独立字段，便于查询和过滤
> - `content` 字段使用 `JsonBinary` 类型，存储 `formatAdapter.encode()` 编码后的消息
> - 使用 `on_delete = "Cascade"` 确保删除会话时自动清理消息

### 第二步：定义 DTO

```rust
// crates/server/src/service/chat_thread/dto.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateThreadDto {
    pub title: Option<String>,
    pub archived: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddMessageDto {
    pub id: String,
    pub parent_id: Option<String>,
    pub format: String,
    pub content: serde_json::Value,
}
```

### 第三步：实现 Service 层

Service 层封装业务逻辑，提供会话和消息的 CRUD 操作。

```rust
// crates/server/src/service/chat_thread/service.rs
use super::dto::{AddMessageDto, UpdateThreadDto};
use crate::common::{ListData, QueryParams, apply_date_filter};
use crate::error::Result;
use anyhow::anyhow;
use chrono::Utc;
use entity::chat_thread::Column;
use sea_orm::{IntoActiveModel, QueryOrder, prelude::*};
use uuid::Uuid;

/// 获取用户的会话列表（分页、排序、搜索）
pub async fn list(
    db: &DbConn,
    params: QueryParams,
    user_id: Uuid,
) -> Result<ListData<entity::chat_thread::Model>> {
    let mut query = entity::chat_thread::Entity::find()
        .filter(Column::UserId.eq(user_id));

    // 支持按标题搜索
    if let Some(ref search) = params.search {
        query = query.filter(Column::Title.contains(search));
    }

    // 支持日期范围过滤
    query = apply_date_filter(
        query,
        Column::UpdatedAt,
        params.start_date.as_deref(),
        params.end_date.as_deref(),
    );

    // 排序处理
    let order = if params.sort_order == "asc" {
        sea_orm::Order::Asc
    } else {
        sea_orm::Order::Desc
    };

    query = match params.sort_by.as_deref() {
        Some("title") => query.order_by(Column::Title, order),
        _ => query.order_by(Column::UpdatedAt, order),  // 默认按更新时间排序
    };

    // 分页查询
    let paginator = query.paginate(db, params.page_size);
    let total = paginator.num_items().await?;
    let list = paginator.fetch_page(params.page.saturating_sub(1)).await?;

    Ok(ListData { list, total })
}

/// 获取单个会话详情
pub async fn get_one(db: &DbConn, id: Uuid, user_id: Uuid) -> Result<entity::chat_thread::Model> {
    let thread = entity::chat_thread::Entity::find_by_id(id)
        .filter(Column::UserId.eq(user_id))
        .one(db)
        .await?;
    thread.ok_or_else(|| anyhow!("会话不存在或无权访问").into())
}

/// 创建新会话
pub async fn create(db: &DbConn, user_id: Uuid) -> Result<entity::chat_thread::Model> {
    let model = entity::chat_thread::ActiveModelEx::new()
        .set_user_id(user_id)
        .set_created_at(Utc::now());
    Ok(model.insert(db).await?.into())
}

/// 更新会话（标题、归档状态）
pub async fn update(
    db: &DbConn,
    id: Uuid,
    user_id: Uuid,
    dto: UpdateThreadDto,
) -> Result<entity::chat_thread::Model> {
    let thread = entity::chat_thread::Entity::find_by_id(id)
        .filter(Column::UserId.eq(user_id))
        .one(db)
        .await?
        .ok_or_else(|| anyhow!("会话不存在或无权访问"))?;

    let mut model = thread.into_active_model().into_ex();

    if let Some(title) = dto.title {
        model = model.set_title(title);
    }
    if let Some(archived) = dto.archived {
        model = model.set_archived(archived);
    }

    Ok(model.update(db).await?.into())
}

/// 删除会话（级联删除消息）
pub async fn delete(db: &DbConn, id: Uuid, user_id: Uuid) -> Result<()> {
    entity::chat_thread::Entity::delete_by_id(id)
        .filter(Column::UserId.eq(user_id))
        .exec(db)
        .await?;
    Ok(())
}

/// 添加消息到会话
pub async fn add_message(
    db: &DbConn,
    thread_id: Uuid,
    user_id: Uuid,
    dto: AddMessageDto,
) -> Result<entity::chat_message::Model> {
    // 使用 exists() 验证会话归属，比 one() 更高效
    let exists = entity::chat_thread::Entity::find_by_id(thread_id)
        .filter(entity::chat_thread::Column::UserId.eq(user_id))
        .exists(db)
        .await?;

    if !exists {
        return Err(anyhow!("会话不存在或无权访问").into());
    }

    let model = entity::chat_message::ActiveModelEx::new()
        .set_id(dto.id)
        .set_thread_id(thread_id)
        .set_parent_id(dto.parent_id)
        .set_format(dto.format)
        .set_content(dto.content)
        .set_created_at(Utc::now());
    Ok(model.insert(db).await?.into())
}

/// 获取会话的所有消息
pub async fn list_messages(
    db: &DbConn,
    thread_id: Uuid,
    user_id: Uuid,
) -> Result<Vec<entity::chat_message::Model>> {
    // 验证会话归属
    let exists = entity::chat_thread::Entity::find_by_id(thread_id)
        .filter(entity::chat_thread::Column::UserId.eq(user_id))
        .exists(db)
        .await?;

    if !exists {
        return Err(anyhow!("会话不存在或无权访问").into());
    }

    let data = entity::chat_message::Entity::find()
        .filter(entity::chat_message::COLUMN.thread_id.eq(thread_id))
        .all(db)
        .await?;

    Ok(data)
}
```

### 第四步：实现 Controller 层

Controller 层定义 RESTful API 路由。

```rust
// crates/server/src/service/chat_thread/controller.rs
use axum::extract::{Query, State};
use axum::{Extension, Router};
use axum_extra::routing::{RouterExt, TypedPath};
use serde::Deserialize;
use std::sync::Arc;
use uuid::Uuid;

use super::service;
use crate::AppState;
use crate::common::{ApiResponse, ListData, QueryParams};
use crate::error::{AppJson, Result};
use crate::service::auth::Claims;
use crate::service::chat_thread::dto::{AddMessageDto, UpdateThreadDto};
use entity::chat_message::Model as MessageModel;
use entity::chat_thread::Model as ThreadModel;

// 类型安全的路由路径定义
#[derive(Debug, TypedPath)]
#[typed_path("/threads")]
pub struct ThreadsPath;

#[derive(Debug, TypedPath, Deserialize)]
#[typed_path("/threads/{id}")]
pub struct ThreadPath {
    id: Uuid,
}

#[derive(Debug, TypedPath, Deserialize)]
#[typed_path("/threads/{thread_id}/messages")]
pub struct MessagesPath {
    thread_id: Uuid,
}

// 获取会话列表
async fn list(
    _: ThreadsPath,
    State(state): State<Arc<AppState>>,
    Extension(claims): Extension<Claims>,
    Query(params): Query<QueryParams>,
) -> Result<ApiResponse<ListData<ThreadModel>>> {
    Ok(ApiResponse::ok(
        service::list(&state.db, params, claims.sub).await?,
    ))
}

// 创建新会话
async fn create(
    _: ThreadsPath,
    State(state): State<Arc<AppState>>,
    Extension(claims): Extension<Claims>,
) -> Result<ApiResponse<ThreadModel>> {
    Ok(ApiResponse::ok(
        service::create(&state.db, claims.sub).await?,
    ))
}

// 获取单个会话
async fn get_one(
    ThreadPath { id }: ThreadPath,
    State(state): State<Arc<AppState>>,
    Extension(claims): Extension<Claims>,
) -> Result<ApiResponse<ThreadModel>> {
    Ok(ApiResponse::ok(
        service::get_one(&state.db, id, claims.sub).await?,
    ))
}

// 更新会话
async fn update(
    ThreadPath { id }: ThreadPath,
    State(state): State<Arc<AppState>>,
    Extension(claims): Extension<Claims>,
    AppJson(dto): AppJson<UpdateThreadDto>,
) -> Result<ApiResponse<ThreadModel>> {
    Ok(ApiResponse::ok(
        service::update(&state.db, id, claims.sub, dto).await?,
    ))
}

// 删除会话
async fn delete(
    ThreadPath { id }: ThreadPath,
    State(state): State<Arc<AppState>>,
    Extension(claims): Extension<Claims>,
) -> Result<ApiResponse<()>> {
    service::delete(&state.db, id, claims.sub).await?;
    Ok(ApiResponse::ok(()))
}

// 获取会话消息列表
async fn list_messages(
    MessagesPath { thread_id }: MessagesPath,
    State(state): State<Arc<AppState>>,
    Extension(claims): Extension<Claims>,
) -> Result<ApiResponse<Vec<MessageModel>>> {
    Ok(ApiResponse::ok(
        service::list_messages(&state.db, thread_id, claims.sub).await?,
    ))
}

// 添加消息
async fn add_message(
    MessagesPath { thread_id }: MessagesPath,
    State(state): State<Arc<AppState>>,
    Extension(claims): Extension<Claims>,
    AppJson(dto): AppJson<AddMessageDto>,
) -> Result<ApiResponse<MessageModel>> {
    Ok(ApiResponse::ok(
        service::add_message(&state.db, thread_id, claims.sub, dto).await?,
    ))
}

// 注册路由
pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .typed_get(get_one)
        .typed_get(list)
        .typed_post(create)
        .typed_put(update)
        .typed_delete(delete)
        .typed_get(list_messages)
        .typed_post(add_message)
}
```

### API 接口汇总

| 方法 | 路径 | 描述 |
| ---- | ---- | ---- |
| GET | `/api/threads` | 获取会话列表 |
| POST | `/api/threads` | 创建新会话 |
| GET | `/api/threads/{id}` | 获取单个会话 |
| PUT | `/api/threads/{id}` | 更新会话 |
| DELETE | `/api/threads/{id}` | 删除会话 |
| GET | `/api/threads/{id}/messages` | 获取消息列表 |
| POST | `/api/threads/{id}/messages` | 添加消息 |

## 前端实现

### 第一步：创建 API 客户端

```typescript
// src/api/chat.ts
import { client } from "./client";
import type { ListData, QueryParams } from "@/types";

/** 会话模型 */
export interface ChatThread {
  id: string;
  user_id: string;
  title: string | null;
  archived: boolean;
  created_at: string;
  updated_at: string;
}

/** 消息模型 */
export interface ChatMessage {
  id: string;
  threadId: string;
  parentId: string | null;
  format: string;
  content: unknown;
  createdAt: string;
}

/** 添加消息 DTO */
export interface AddMessageDto {
  id: string;
  parentId: string | null;
  format: string;
  content: unknown;
}

/** 更新会话 DTO */
export interface UpdateThreadDto {
  title?: string;
  archived?: boolean;
}

export const chatApi = {
  // 会话管理
  listThreads: (params?: QueryParams) =>
    client.get<ListData<ChatThread>>("/api/threads", params),

  getThread: (id: string) =>
    client.get<ChatThread>(`/api/threads/${id}`),

  createThread: () =>
    client.post<ChatThread>("/api/threads"),

  updateThread: (id: string, data: UpdateThreadDto) =>
    client.put<ChatThread>(`/api/threads/${id}`, data),

  deleteThread: (id: string) =>
    client.delete<void>(`/api/threads/${id}`),

  // 消息管理
  listMessages: (threadId: string) =>
    client.get<ChatMessage[]>(`/api/threads/${threadId}/messages`),

  addMessage: (threadId: string, dto: AddMessageDto) =>
    client.post<ChatMessage>(`/api/threads/${threadId}/messages`, dto),
};
```

### 第二步：实现 ThreadListAdapter

`RemoteThreadListAdapter` 负责会话列表的 CRUD 操作。

```typescript
// src/lib/chat/thread-list-adapter.ts
import type {
  unstable_RemoteThreadListAdapter as RemoteThreadListAdapter
} from "@assistant-ui/react";
import { chatApi } from "@/api";

export const createThreadListAdapter = (): RemoteThreadListAdapter => ({
  /**
   * 获取会话列表
   * 在组件挂载时调用，用于初始化会话列表
   */
  async list() {
    const data = await chatApi.listThreads({ page: 1, pageSize: 100 });
    return {
      threads: data.list.map((t) => ({
        remoteId: t.id,
        externalId: undefined,
        title: t.title ?? undefined,
        status: t.archived ? ("archived" as const) : ("regular" as const),
      })),
    };
  },

  /**
   * 初始化新会话
   * 当用户发送第一条消息时调用
   */
  async initialize(_threadId: string) {
    const data = await chatApi.createThread();
    return { remoteId: data.id, externalId: undefined };
  },

  /**
   * 重命名会话
   */
  async rename(remoteId: string, newTitle: string) {
    await chatApi.updateThread(remoteId, { title: newTitle });
  },

  /**
   * 归档会话
   */
  async archive(remoteId: string) {
    await chatApi.updateThread(remoteId, { archived: true });
  },

  /**
   * 取消归档
   */
  async unarchive(remoteId: string) {
    await chatApi.updateThread(remoteId, { archived: false });
  },

  /**
   * 删除会话
   */
  async delete(remoteId: string) {
    await chatApi.deleteThread(remoteId);
  },

  /**
   * 获取单个会话
   */
  async fetch(threadId: string) {
    const data = await chatApi.getThread(threadId);
    return {
      remoteId: data.id,
      externalId: undefined,
      title: data.title ?? undefined,
      status: data.archived ? ("archived" as const) : ("regular" as const),
    };
  },

  /**
   * 生成标题（可选实现）
   */
  async generateTitle(_remoteId: string, _messages: readonly unknown[]) {
    // TODO: 调用 AI 生成标题
    return new ReadableStream({
      start(controller) {
        controller.close();
      },
    }) as never;
  },
});
```

### 第三步：实现 HistoryAdapter

`ThreadHistoryAdapter` 负责消息历史的加载和保存。

> **⚠️ 重要提醒**：如果你使用 `useChatRuntime`（AI SDK v5），必须实现 `withFormat` 方法！详见下文"踩坑提醒"部分。

```typescript
/**
 * ThreadHistoryAdapter 实现
 * 用于加载和保存消息历史
 *
 * AI SDK runtime 使用 withFormat 模式调用 history adapter:
 * historyAdapter.withFormat(formatAdapter).append(item)
 * historyAdapter.withFormat(formatAdapter).load()
 */
import type {
  ThreadHistoryAdapter,
  ExportedMessageRepository,
  ThreadMessage,
} from "@assistant-ui/react";
import { useAssistantApi } from "@assistant-ui/react";
import { chatApi, type ChatMessage } from "@/api/chat";
import { useMemo } from "react";

interface MessageRepositoryItem {
  message: ThreadMessage;
  parentId: string | null;
}

interface MessageFormatItem<TMessage> {
  message: TMessage;
  parentId: string | null;
}

interface MessageFormatRepository<TMessage> {
  headId?: string | null;
  messages: MessageFormatItem<TMessage>[];
}

interface MessageFormatAdapter<TMessage, TStorageFormat> {
  format: string;
  encode(item: MessageFormatItem<TMessage>): TStorageFormat;
  decode(entry: {
    id: string;
    parent_id: string | null;
    format: string;
    content: TStorageFormat;
  }): MessageFormatItem<TMessage>;
  getId(message: TMessage): string;
}

interface GenericThreadHistoryAdapter<TMessage> {
  load(): Promise<MessageFormatRepository<TMessage>>;
  append(item: MessageFormatItem<TMessage>): Promise<void>;
}

/**
 * 创建消息历史适配器的 Hook
 */
export function useHistoryAdapter(): ThreadHistoryAdapter {
  const api = useAssistantApi();

  return useMemo<ThreadHistoryAdapter>(() => {
    const getRemoteId = () => api.threadListItem().getState().remoteId;

    const initializeAndGetRemoteId = async () => {
      const { remoteId } = await api.threadListItem().initialize();
      return remoteId;
    };

    return {
      // 直接调用模式（用于 LocalRuntime）
      async load(): Promise<ExportedMessageRepository> {
        const remoteId = getRemoteId();
        if (!remoteId) return { messages: [] };

        const messages = await chatApi.listMessages(remoteId);
        const exportedMessages = messages.map(
          (m: ChatMessage) => m.content as MessageRepositoryItem,
        );

        return {
          headId: exportedMessages.at(-1)?.message.id ?? null,
          messages: exportedMessages,
        };
      },

      async append(item: MessageRepositoryItem): Promise<void> {
        const remoteId = await initializeAndGetRemoteId();
        if (remoteId) {
          await chatApi.addMessage(remoteId, {
            id: item.message.id,
            parentId: item.parentId,
            format: "thread-message",
            content: item,
          });
        }
      },

      // withFormat 模式（用于 AI SDK runtime）
      withFormat<TMessage, TStorageFormat>(
        formatAdapter: MessageFormatAdapter<TMessage, TStorageFormat>,
      ): GenericThreadHistoryAdapter<TMessage> {
        return {
          async load(): Promise<MessageFormatRepository<TMessage>> {
            const remoteId = getRemoteId();
            if (!remoteId) return { messages: [] };

            const messages = await chatApi.listMessages(remoteId);

            // 直接从数据库字段读取，不再需要嵌套解析
            const decodedMessages = messages
              .filter((m) => m.format === formatAdapter.format)
              .map((m: ChatMessage) =>
                formatAdapter.decode({
                  id: m.id,
                  parent_id: m.parentId,
                  format: m.format,
                  content: m.content as TStorageFormat,
                }),
              );

            return {
              headId:
                decodedMessages.length > 0
                  ? formatAdapter.getId(decodedMessages.at(-1)!.message)
                  : null,
              messages: decodedMessages,
            };
          },

          async append(item: MessageFormatItem<TMessage>): Promise<void> {
            const remoteId = await initializeAndGetRemoteId();
            if (!remoteId) return;

            const encoded = formatAdapter.encode(item);
            await chatApi.addMessage(remoteId, {
              id: formatAdapter.getId(item.message),
              parentId: item.parentId,
              format: formatAdapter.format,
              content: encoded,
            });
          },
        };
      },
    };
  }, [api]);
}
```

> **重要变化**：新版本将 `id`、`parentId`、`format` 作为独立字段存储，`content` 只存储编码后的消息内容。这简化了数据结构，避免了嵌套解析的复杂性。

### 踩坑提醒：AI SDK Runtime 必须实现 withFormat

这是一个非常隐蔽的坑！如果你使用 `useChatRuntime`（AI SDK v5），直接实现 `load()` 和 `append()` 方法是**不够的**。

#### 问题现象

- `HistoryAdapterProvider` 正常渲染
- `useHistoryAdapter()` 正常创建 adapter
- 但发送消息后，`append()` **永远不会被调用**

#### 问题根因

查看 `@assistant-ui/react-ai-sdk` 的源码：

```typescript
// useExternalHistory.tsx
useEffect(() => {
  return runtimeRef.current.thread.subscribe(async () => {
    // ...
    await historyAdapter?.withFormat?.(storageFormatAdapter).append({
      parentId,
      message: getExternalStoreMessages<TMessage>(message)[0]!,
    });
  });
}, [historyAdapter, storageFormatAdapter, runtimeRef]);
```

关键点：**AI SDK runtime 调用的是 `historyAdapter.withFormat(formatAdapter).append()`，而不是直接调用 `historyAdapter.append()`！**

由于 `withFormat?.()` 使用了可选链，如果 `withFormat` 未实现，返回 `undefined`，后续的 `.append()` 就不会执行。

#### 不同 Runtime 的调用方式

| Runtime            | 调用方式                                  | 需要实现                                    |
| ------------------ | ----------------------------------------- | ------------------------------------------- |
| `useLocalRuntime`  | `historyAdapter.append()`                 | `load()`, `append()`                        |
| `useChatRuntime`   | `historyAdapter.withFormat().append()`    | `load()`, `append()`, **`withFormat()`**    |

#### 存储格式设计

新版本将消息元数据提取为独立字段，简化了存储结构：

```typescript
// 数据库存储结构
{
  id: string,           // 前端消息 ID（主键）
  thread_id: string,    // 会话 ID
  parent_id: string,    // 父消息 ID
  format: string,       // 格式标识，如 "ai-sdk-ui"
  content: TStorageFormat, // formatAdapter.encode() 的结果
  created_at: string
}
```

与旧版嵌套结构相比：

| 特性 | 旧版（嵌套） | 新版（扁平） |
| ---- | ------------ | ------------ |
| ID 存储 | 后端生成 Uuid + 前端 ID 在 value 中 | 直接使用前端 ID |
| 查询过滤 | 需解析 JSON | 直接字段过滤 |
| 数据冗余 | parentId 存两处 | 无冗余 |

### 第四步：集成到 Chat 页面

```typescript
// src/routes/chat.tsx
import { useMemo, useRef } from "react";
import { createFileRoute, redirect, useNavigate } from "@tanstack/react-router";
import {
  AssistantRuntimeProvider,
  RuntimeAdapterProvider,
  unstable_useRemoteThreadListRuntime as useRemoteThreadListRuntime,
} from "@assistant-ui/react";
import {
  useChatRuntime,
  AssistantChatTransport,
} from "@assistant-ui/react-ai-sdk";
import { useAuthStore } from "@/stores/auth";
import { Thread } from "@/components/thread";
import { createThreadListAdapter, useHistoryAdapter } from "@/lib/chat";

export const Route = createFileRoute("/chat")({
  beforeLoad: () => {
    const token = useAuthStore.getState().token;
    if (!token) {
      throw redirect({ to: "/login" });
    }
  },
  component: ChatPage,
});

/**
 * History Adapter Provider
 * 用于注入消息历史适配器
 */
function HistoryAdapterProvider({ children }: { children?: React.ReactNode }) {
  const history = useHistoryAdapter();
  return (
    <RuntimeAdapterProvider adapters={{ history }}>
      {children}
    </RuntimeAdapterProvider>
  );
}

function ChatPage() {
  const navigate = useNavigate();
  const [token, user] = useAuthStore((state) => [state.token, state.user]);

  // 创建 thread list adapter
  const threadListAdapter = useMemo(() => {
    const adapter = createThreadListAdapter();
    // 注入 HistoryAdapterProvider，为每个会话提供历史适配器
    adapter.unstable_Provider = HistoryAdapterProvider;
    return adapter;
  }, []);

  // 使用 useRemoteThreadListRuntime 包装，支持会话持久化
  const runtime = useRemoteThreadListRuntime({
    // runtimeHook 为每个会话创建独立的 runtime
    runtimeHook: () =>
      useChatRuntime({
        transport: new AssistantChatTransport({
          api: `${import.meta.env.VITE_API_URL}/api/chat`,
          headers: {
            Authorization: `Bearer ${token}`,
          },
          body: {
            sessionId: user?.id,
          },
        }),
      }),
    adapter: threadListAdapter,
  });

  return (
    <AssistantRuntimeProvider runtime={runtime}>
      <div className="flex h-full flex-col">
        {/* 聊天 UI */}
        <Thread />
      </div>
    </AssistantRuntimeProvider>
  );
}
```

### 深入理解：allowNesting 机制

你可能注意到我们在 `runtimeHook` 中使用了 `useChatRuntime`，而 `useChatRuntime` 内部也调用了 `useRemoteThreadListRuntime`。这会导致嵌套吗？

答案是：**不会**，这是 assistant-ui 的设计特性。

#### 源码解析

查看 `useRemoteThreadListRuntime` 的源码：

```typescript
// @assistant-ui/react 源码
export const useRemoteThreadListRuntime = (
  options: RemoteThreadListOptions,
): AssistantRuntime => {
  const api = useAssistantApiImpl();
  const isNested = api.threadListItem.source !== null;  // 检测是否嵌套

  if (isNested) {
    if (!options.allowNesting) {
      throw new Error(
        "useRemoteThreadListRuntime cannot be nested inside another RemoteThreadListRuntime. " +
          "Set allowNesting: true to allow nesting (the inner runtime will become a no-op).",
      );
    }

    // 如果 allowNesting 为 true 且已在嵌套上下文中，
    // 直接调用 runtimeHook() 返回，跳过 thread list 逻辑
    return options.runtimeHook();
  }

  return useRemoteThreadListRuntimeImpl(options);
};
```

#### 工作流程

```mermaid
flowchart TB
    subgraph Outer["外层 useRemoteThreadListRuntime（我们的自定义 adapter）"]
        Check1{isNested?}
        Check1 -->|false| CreateThreadList[创建 ThreadListRuntime]
        CreateThreadList --> CallHook[调用 runtimeHook]
    end

    subgraph Inner["内层 useChatRuntime"]
        InnerRemote[useRemoteThreadListRuntime<br/>allowNesting: true]
        Check2{isNested?}
        InnerRemote --> Check2
        Check2 -->|true| DirectReturn[直接返回 useChatThreadRuntime]
    end

    CallHook --> InnerRemote
    DirectReturn --> Result[最终 Runtime]

    style CreateThreadList fill:#90EE90
    style DirectReturn fill:#87CEEB
```

#### 执行顺序

1. **外层** `useRemoteThreadListRuntime`（我们的代码）
   - `isNested = false`（不在任何 thread list 上下文中）
   - 正常创建 `RemoteThreadListRuntimeCore`，使用我们的自定义 adapter
   - 调用 `runtimeHook()`

2. **内层** `useChatRuntime` 中的 `useRemoteThreadListRuntime`
   - `isNested = true`（已在外层的 thread list 上下文中）
   - 由于 `allowNesting: true`，**跳过** cloud adapter 的 thread list 逻辑
   - 直接调用并返回 `useChatThreadRuntime()`

#### 结果

- 只有**我们的自定义 adapter** 生效
- `useChatRuntime` 内部的 cloud adapter 被跳过（no-op）
- 保留了 `useChatThreadRuntime` 的聊天能力

这个设计使得我们可以安全地组合使用 `useRemoteThreadListRuntime` 和 `useChatRuntime`，而不用担心冲突。

## 数据流示意

### 创建新会话流程

```mermaid
sequenceDiagram
    participant User
    participant Frontend
    participant ThreadListAdapter
    participant HistoryAdapter
    participant Backend

    User->>Frontend: 发送第一条消息
    Frontend->>ThreadListAdapter: initialize(localId)
    ThreadListAdapter->>Backend: POST /api/threads
    Backend-->>ThreadListAdapter: { id: "xxx" }
    ThreadListAdapter-->>Frontend: { remoteId: "xxx" }

    Frontend->>HistoryAdapter: append(userMessage)
    HistoryAdapter->>Backend: POST /api/threads/xxx/messages
    Backend-->>HistoryAdapter: 200 OK

    Frontend->>Backend: POST /api/chat (AI 对话)
    Backend-->>Frontend: SSE 流式响应

    Frontend->>HistoryAdapter: append(assistantMessage)
    HistoryAdapter->>Backend: POST /api/threads/xxx/messages
```

### 加载历史会话流程

```mermaid
sequenceDiagram
    participant User
    participant Frontend
    participant ThreadListAdapter
    participant HistoryAdapter
    participant Backend

    User->>Frontend: 打开聊天页面
    Frontend->>ThreadListAdapter: list()
    ThreadListAdapter->>Backend: GET /api/threads
    Backend-->>ThreadListAdapter: [{ id, title, ... }]
    ThreadListAdapter-->>Frontend: 会话列表

    User->>Frontend: 选择一个会话
    Frontend->>HistoryAdapter: load()
    HistoryAdapter->>Backend: GET /api/threads/xxx/messages
    Backend-->>HistoryAdapter: [{ value: {...} }]
    HistoryAdapter-->>Frontend: 历史消息
    Frontend->>User: 显示历史对话
```

## 消息格式说明

消息存储采用扁平化结构，`content` 字段存储 `formatAdapter.encode()` 编码后的消息：

### AI SDK Runtime 格式 (format: "ai-sdk-ui")

```typescript
// content 字段存储 UIMessage 格式
{
  "id": "msg-uuid",
  "role": "user" | "assistant",
  "parts": [
    { "type": "text", "text": "消息内容" },
    // 或工具调用
    { "type": "tool-call", "toolCallId": "xxx", "toolName": "xxx", "args": {} }
  ],
  "createdAt": "2024-01-01T00:00:00Z"
}
```

### LocalRuntime 格式 (format: "thread-message")

```typescript
// content 字段存储 MessageRepositoryItem 格式
{
  "message": {
    "id": "msg-uuid",
    "role": "user" | "assistant",
    "content": [{ "type": "text", "text": "消息内容" }],
    "createdAt": "2024-01-01T00:00:00Z"
  },
  "parentId": "parent-msg-uuid" | null
}
```

> **设计考量**：通过 `format` 字段区分不同 runtime 的消息格式，便于在 `load()` 时筛选和解码。

## 最佳实践

### 1. 权限控制

始终验证用户对会话的所有权：

```rust
// 每个操作都验证 user_id
let thread = entity::chat_thread::Entity::find_by_id(id)
    .filter(Column::UserId.eq(user_id))  // 关键：过滤用户
    .one(db)
    .await?;
```

### 2. 使用 exists() 而非 one()

当只需要验证存在性时，使用 `exists()` 更高效：

```rust
// 推荐
let exists = entity::chat_thread::Entity::find_by_id(thread_id)
    .filter(Column::UserId.eq(user_id))
    .exists(db)
    .await?;

// 不推荐（多余的数据加载）
let thread = entity::chat_thread::Entity::find_by_id(thread_id)
    .filter(Column::UserId.eq(user_id))
    .one(db)
    .await?;
let exists = thread.is_some();
```

### 3. 等待会话初始化

在 HistoryAdapter 的 `append` 方法中，必须等待会话初始化完成：

```typescript
async append(item: MessageFormatItem<TMessage>): Promise<void> {
  // 必须 await，避免首条消息丢失
  const remoteId = await initializeAndGetRemoteId();
  if (!remoteId) return;

  const encoded = formatAdapter.encode(item);
  await chatApi.addMessage(remoteId, {
    id: formatAdapter.getId(item.message),
    parentId: item.parentId,
    format: formatAdapter.format,
    content: encoded,
  });
}
```

### 4. 级联删除

在 Entity 定义中使用 `on_delete = "Cascade"`，确保删除会话时自动清理关联消息：

```rust
#[sea_orm(belongs_to, from = "thread_id", to = "id", on_delete = "Cascade")]
pub thread: HasOne<super::chat_thread::Entity>,
```

## 总结

本文介绍了使用 Rust Axum + SeaORM 实现 assistant-ui Custom Thread List 的完整方案：

1. **Entity 层**：使用 SeaORM Entity First 模式定义 `chat_thread` 和 `chat_message` 模型
2. **Service 层**：封装 CRUD 业务逻辑，包含权限验证
3. **Controller 层**：使用 axum-extra 的 TypedPath 定义类型安全的 RESTful API
4. **前端适配器**：
   - `RemoteThreadListAdapter`：管理会话列表
   - `ThreadHistoryAdapter`：管理消息历史
5. **集成**：使用 `useRemoteThreadListRuntime` 组合两个适配器

这套方案实现了完整的会话持久化功能，支持多会话管理、历史消息加载、会话归档等特性。
