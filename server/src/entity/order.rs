use rust_decimal::Decimal;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq,Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[sea_orm(table_name = "order")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub customer_id: Uuid,
    pub boss_id: Uuid,
    pub product_name: String,
    pub description: Option<String>,
    #[sea_orm(column_type = "JsonBinary", nullable)]
    pub images: Option<Json>,
    pub quantity: i32,
    #[sea_orm(column_type = "Decimal(Some((10, 2)))")]
    pub unit_price: Decimal,
    #[sea_orm(default_value = "pending")]
    pub status: String,
    pub received_at: DateTimeUtc,
    pub delivered_at: Option<DateTimeUtc>,

    #[serde(skip)]
    #[sea_orm(belongs_to, from = "customer_id", to = "id")]
    pub customer: HasOne<super::customer::Entity>,

    #[serde(skip)]
    #[sea_orm(has_many)]
    pub processes: HasMany<super::process::Entity>,
}

impl ActiveModelBehavior for ActiveModel {}
