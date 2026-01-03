use rust_decimal::Decimal;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, EnumString, Display, DeriveValueType,
)]
#[serde(rename_all = "camelCase")]
#[strum(serialize_all = "camelCase")]
#[sea_orm(value_type = "String")]
pub enum OrderStatus {
    Pending,
    Processing,
    Completed,
    Delivered,
    Cancelled,
}

impl OrderStatus {
    pub fn can_transition_to(&self, next: Self) -> bool {
        matches!(
            (self, next),
            (Self::Pending, Self::Processing)
                | (Self::Pending, Self::Cancelled)
                | (Self::Processing, Self::Completed)
                | (Self::Processing, Self::Cancelled)
                | (Self::Completed, Self::Delivered)
        )
    }
}

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
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
    pub status: OrderStatus,
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
