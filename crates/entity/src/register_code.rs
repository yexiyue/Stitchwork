use schemars::JsonSchema;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

/// 统一注册码，超管创建供老板注册，老板创建供员工注册
#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
#[sea_orm(table_name = "register_code")]
pub struct Model {
    /// 注册码唯一标识符
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    /// 6 位数字注册码，全局唯一
    #[sea_orm(unique)]
    pub code: String,
    /// 创建者用户 ID（超管或老板）
    pub created_by: Option<Uuid>,
    /// 关联工坊 ID：有值 = Staff 码，NULL = Boss 码
    pub workshop_id: Option<Uuid>,
    /// 使用该注册码的用户 ID
    pub used_by: Option<Uuid>,
    /// 注册码使用时间
    pub used_at: Option<DateTimeUtc>,
    /// 注册码创建时间
    #[sea_orm(default_expr = "Expr::current_timestamp()")]
    pub created_at: DateTimeUtc,

    #[serde(skip)]
    #[sea_orm(belongs_to, relation_enum = "UsedByUser", from = "used_by", to = "id")]
    pub user: HasOne<super::user::Entity>,

    #[serde(skip)]
    #[sea_orm(belongs_to, relation_enum = "Creator", from = "created_by", to = "id")]
    pub creator: HasOne<super::user::Entity>,

    #[serde(skip)]
    #[sea_orm(belongs_to, from = "workshop_id", to = "id")]
    pub workshop: HasOne<super::workshop::Entity>,
}

impl ActiveModelBehavior for ActiveModel {}
