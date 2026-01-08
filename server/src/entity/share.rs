use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[sea_orm(table_name = "share")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub boss_id: Uuid,
    pub title: String,
    pub description: Option<String>,
    #[sea_orm(unique)]
    pub token: String,
    #[sea_orm(column_type = "JsonBinary")]
    pub order_ids: Json,
    #[sea_orm(column_type = "JsonBinary")]
    pub process_ids: Json,
    #[sea_orm(default_value = true)]
    pub is_active: bool,
    pub created_at: DateTimeUtc,

    #[serde(skip)]
    #[sea_orm(belongs_to, from = "boss_id", to = "id")]
    pub boss: HasOne<super::user::Entity>,
}

impl ActiveModelBehavior for ActiveModel {}
