use rust_decimal::Decimal;
use sea_orm::entity::prelude::*;

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "process")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    pub order_id: Uuid,
    pub name: String,
    #[sea_orm(column_type = "Decimal(Some((10, 2)))")]
    pub piece_price: Decimal,

    #[sea_orm(belongs_to, from = "order_id", to = "id")]
    pub order: HasOne<super::order::Entity>,

    #[sea_orm(has_many)]
    pub piece_records: HasMany<super::piece_record::Entity>,
}

impl ActiveModelBehavior for ActiveModel {}
