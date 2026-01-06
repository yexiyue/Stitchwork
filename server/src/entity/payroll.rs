use rust_decimal::Decimal;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[sea_orm(table_name = "payroll")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub user_id: Uuid,
    pub boss_id: Uuid,
    #[sea_orm(column_type = "Decimal(Some((10, 2)))")]
    pub amount: Decimal,
    pub payment_image: Option<String>,
    pub note: Option<String>,
    pub paid_at: DateTimeUtc,

    #[sea_orm(belongs_to, from = "user_id", to = "id")]
    pub user: HasOne<super::user::Entity>,

    #[sea_orm(has_many, via = "payroll_record")]
    pub price_records: HasMany<super::piece_record::Entity>,
}

impl ActiveModelBehavior for ActiveModel {}

impl crate::common::OwnedByBoss for Model {
    fn boss_id(&self) -> Uuid {
        self.boss_id
    }
}
