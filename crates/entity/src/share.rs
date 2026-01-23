use schemars::JsonSchema;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

/// 分享链接，用于老板分享订单/工序信息给外部人员查看
#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
#[sea_orm(table_name = "share")]
pub struct Model {
    /// 分享记录唯一标识符
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    /// 创建分享的老板用户 ID
    pub boss_id: Uuid,
    /// 分享标题
    pub title: String,
    /// 分享描述
    pub description: Option<String>,
    /// 分享链接的唯一 token，用于构建访问 URL
    #[sea_orm(unique)]
    pub token: String,
    /// 分享的订单 ID 列表
    #[sea_orm(column_type = "JsonBinary")]
    pub order_ids: Json,
    /// 分享的工序 ID 列表
    #[sea_orm(column_type = "JsonBinary")]
    pub process_ids: Json,
    /// 分享链接是否有效
    #[sea_orm(default_value = true)]
    pub is_active: bool,
    /// 分享创建时间
    #[sea_orm(default_expr = "Expr::current_timestamp()")]
    pub created_at: DateTimeUtc,

    #[serde(skip)]
    #[sea_orm(belongs_to, from = "boss_id", to = "id")]
    pub boss: HasOne<super::user::Entity>,
}

impl ActiveModelBehavior for ActiveModel {}
