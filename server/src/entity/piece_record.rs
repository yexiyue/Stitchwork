use rust_decimal::Decimal;
use sea_orm::entity::prelude::*;

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "piece_record")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    pub process_id: Uuid,
    pub worker_id: Uuid,
    pub quantity: i32,
    #[sea_orm(column_type = "Decimal(Some((10, 2)))")]
    pub amount: Decimal,
    pub recorded_at: DateTime,

    #[sea_orm(belongs_to, from = "process_id", to = "id")]
    pub process: HasOne<super::process::Entity>,

    #[sea_orm(belongs_to, from = "worker_id", to = "id")]
    pub worker: HasOne<super::worker::Entity>,
}

impl ActiveModelBehavior for ActiveModel {}
