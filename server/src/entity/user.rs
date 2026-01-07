use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, EnumString, Display, DeriveValueType,
)]
#[serde(rename_all = "camelCase")]
#[strum(serialize_all = "camelCase")]
#[sea_orm(value_type = "String")]
pub enum Role {
    Boss,
    Staff,
}

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[sea_orm(table_name = "user")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    #[sea_orm(unique)]
    pub username: String,
    #[serde(skip)]
    pub password_hash: String,
    pub role: Role,
    pub display_name: Option<String>,

    #[sea_orm(unique)]
    pub phone: String,
    pub avatar: Option<String>,
    pub workshop_id: Option<Uuid>,

    #[sea_orm(default_value = "false")]
    pub is_super_admin: bool,
    pub created_at: DateTimeUtc,

    #[serde(skip)]
    #[sea_orm(has_many)]
    pub customers: HasMany<super::customer::Entity>,

    #[serde(skip)]
    #[sea_orm(has_many)]
    pub piece_records: HasMany<super::piece_record::Entity>,

    #[serde(skip)]
    #[sea_orm(has_many)]
    pub payrolls: HasMany<super::payroll::Entity>,

    #[serde(skip)]
    #[sea_orm(
        belongs_to,
        relation_enum = "WorkshopMember",
        from = "workshop_id",
        to = "id"
    )]
    pub workshop: HasOne<super::workshop::Entity>,

    // 钻石关系需要先断开，同步完数据库后才能恢复
    #[sea_orm(has_many, relation_enum = "WorkshopOwner", via_rel = "HasOwner")]
    pub workshops: HasMany<super::workshop::Entity>,
}

impl ActiveModelBehavior for ActiveModel {}
