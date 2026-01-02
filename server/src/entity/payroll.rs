use rust_decimal::Decimal;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq,Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[sea_orm(table_name = "payroll")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub user_id: Uuid,
    #[sea_orm(column_type = "Decimal(Some((10, 2)))")]
    pub amount: Decimal,
    pub note: Option<String>,
    pub paid_at: DateTimeUtc,

    #[serde(skip)]
    #[sea_orm(belongs_to, from = "user_id", to = "id")]
    pub user: HasOne<super::user::Entity>,
}

impl ActiveModelBehavior for ActiveModel {}
