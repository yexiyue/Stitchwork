use schemars::JsonSchema;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

/// 注册码，老板注册时需要使用的一次性激活码
#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
#[sea_orm(table_name = "register_code")]
pub struct Model {
    /// 注册码唯一标识符
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    /// 8 位注册码，全局唯一
    #[sea_orm(unique)]
    pub code: String,
    /// 注册码是否可用
    #[sea_orm(default_value = "true")]
    pub is_active: bool,
    /// 使用该注册码的用户 ID
    pub used_by: Option<Uuid>,
    /// 注册码使用时间
    pub used_at: Option<DateTimeUtc>,
    /// 注册码创建时间
    #[sea_orm(default_expr = "Expr::current_timestamp()")]
    pub created_at: DateTimeUtc,

    #[serde(skip)]
    #[sea_orm(belongs_to, from = "used_by", to = "id")]
    pub user: HasOne<super::user::Entity>,
}

impl ActiveModelBehavior for ActiveModel {}
