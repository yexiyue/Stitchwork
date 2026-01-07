use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[sea_orm(table_name = "register_code")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    #[sea_orm(unique)]
    pub code: String,
    #[sea_orm(default_value = "true")]
    pub is_active: bool,
    pub used_by: Option<Uuid>,
    pub used_at: Option<DateTimeUtc>,
    pub created_at: DateTimeUtc,

    #[serde(skip)]
    #[sea_orm(belongs_to, from = "used_by", to = "id")]
    pub user: HasOne<super::user::Entity>,
}

impl ActiveModelBehavior for ActiveModel {}
