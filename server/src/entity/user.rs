use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, EnumString, Display, DeriveValueType,
)]
#[serde(rename_all = "camelCase")]
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
    pub phone: Option<String>,
    pub avatar: Option<String>,
    pub workshop_name: Option<String>,
    pub workshop_desc: Option<String>,
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
    #[sea_orm(self_ref, via = "employment", from = "Boss", to = "Staff")]
    pub staffs: HasMany<Entity>,
}

impl ActiveModelBehavior for ActiveModel {}
