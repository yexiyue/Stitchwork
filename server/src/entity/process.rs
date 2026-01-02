use rust_decimal::Decimal;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[sea_orm(table_name = "process")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub order_id: Uuid,
    pub boss_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    #[sea_orm(column_type = "Decimal(Some((10, 2)))")]
    pub piece_price: Decimal,

    #[serde(skip)]
    #[sea_orm(belongs_to, from = "order_id", to = "id")]
    pub order: HasOne<super::order::Entity>,

    #[serde(skip)]
    #[sea_orm(has_many)]
    pub piece_records: HasMany<super::piece_record::Entity>,
}

impl ActiveModelBehavior for ActiveModel {}
