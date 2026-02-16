use schemars::JsonSchema;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

/// 客户，服装订单的来源方
#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
#[sea_orm(table_name = "customer")]
pub struct Model {
    /// 客户唯一标识符
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    /// 创建该客户的用户（老板）ID
    pub user_id: Uuid,
    /// 客户名称
    pub name: String,
    /// 客户联系电话
    pub phone: Option<String>,
    /// 客户备注说明
    pub description: Option<String>,
    /// 客户创建时间
    #[sea_orm(default_expr = "Expr::current_timestamp()")]
    pub created_at: DateTimeUtc,
    /// 客户信息最后更新时间
    #[sea_orm(auto_update, default_expr = "Expr::current_timestamp()")]
    pub updated_at: DateTimeUtc,

    #[serde(skip)]
    #[sea_orm(belongs_to, from = "user_id", to = "id")]
    pub user: HasOne<super::user::Entity>,

    #[serde(skip)]
    #[sea_orm(has_many)]
    pub orders: HasMany<super::order::Entity>,
}

impl ActiveModelBehavior for ActiveModel {}
