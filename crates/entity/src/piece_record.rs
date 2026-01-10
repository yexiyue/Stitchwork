use rust_decimal::Decimal;
use schemars::JsonSchema;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
    EnumString,
    Display,
    DeriveValueType,
    JsonSchema,
)]
#[serde(rename_all = "camelCase")]
#[strum(serialize_all = "camelCase")]
#[sea_orm(value_type = "String")]
pub enum PieceRecordStatus {
    Pending,
    Approved,
    Rejected,
    Settled,
}

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
    EnumString,
    Display,
    DeriveValueType,
    JsonSchema,
)]
#[serde(rename_all = "camelCase")]
#[strum(serialize_all = "camelCase")]
#[sea_orm(value_type = "String")]
pub enum RecordedBy {
    BySelf,
    ByBoss,
}

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
#[sea_orm(table_name = "piece_record")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub process_id: Uuid,
    pub user_id: Uuid,
    pub boss_id: Uuid,
    pub quantity: i32,
    #[sea_orm(column_type = "Decimal(Some((10, 2)))")]
    pub amount: Decimal,
    pub status: PieceRecordStatus,
    pub recorded_by: RecordedBy,
    pub recorded_at: DateTimeUtc,

    #[serde(skip)]
    #[sea_orm(belongs_to, from = "process_id", to = "id")]
    pub process: HasOne<super::process::Entity>,

    #[serde(skip)]
    #[sea_orm(belongs_to, from = "user_id", to = "id")]
    pub user: HasOne<super::user::Entity>,
    
    #[serde(skip)]
    #[sea_orm(has_one, via = "payroll_record")]
    pub payroll: HasOne<super::payroll::Entity>,
}

impl ActiveModelBehavior for ActiveModel {}
