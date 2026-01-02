use rust_decimal::Decimal;
use sea_orm::entity::prelude::*;

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "payroll")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    pub worker_id: Uuid,
    #[sea_orm(column_type = "Decimal(Some((10, 2)))")]
    pub amount: Decimal,
    pub note: Option<String>,
    pub paid_at: DateTime,

    #[sea_orm(belongs_to, from = "worker_id", to = "id")]
    pub worker: HasOne<super::worker::Entity>,
}

impl ActiveModelBehavior for ActiveModel {}
