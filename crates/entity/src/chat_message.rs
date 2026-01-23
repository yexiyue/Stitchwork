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
    /// 消息格式（如 "aisdk/v5"）
    pub format: String,
    /// 消息内容，JSON 格式存储编码后的消息
    #[sea_orm(column_type = "JsonBinary")]
    pub content: serde_json::Value,
    /// 消息创建时间
    #[sea_orm(default_expr = "Expr::current_timestamp()")]
    pub created_at: DateTimeUtc,

    #[serde(skip)]
    #[sea_orm(belongs_to, from = "thread_id", to = "id", on_delete = "Cascade")]
    pub thread: HasOne<super::chat_thread::Entity>,
}

impl ActiveModelBehavior for ActiveModel {}
