use rust_decimal::Decimal;
use sea_orm::entity::prelude::*;

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "order")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    pub customer_id: Uuid,
    pub product_name: String,
    pub quantity: i32,
    #[sea_orm(column_type = "Decimal(Some((10, 2)))")]
    pub unit_price: Decimal,
    #[sea_orm(default_value = "pending")]
    pub status: String,
    pub received_at: DateTime,
    pub delivered_at: Option<DateTime>,

    #[sea_orm(belongs_to, from = "customer_id", to = "id")]
    pub customer: HasOne<super::customer::Entity>,

    #[sea_orm(has_many)]
    pub processes: HasMany<super::process::Entity>,
}

impl ActiveModelBehavior for ActiveModel {}
