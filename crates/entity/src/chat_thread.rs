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

    #[serde(skip)]
    #[sea_orm(belongs_to, from = "user_id", to = "id", on_delete = "Cascade")]
    pub user: HasOne<super::user::Entity>,

    #[serde(skip)]
    #[sea_orm(has_many)]
    pub messages: HasMany<super::chat_message::Entity>,
}

impl ActiveModelBehavior for ActiveModel {}
