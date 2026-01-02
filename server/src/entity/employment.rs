use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[sea_orm(table_name = "employment")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub boss_id: Uuid,
    pub staff_id: Uuid,
    pub created_at: DateTimeUtc,

    #[serde(skip)]
    #[sea_orm(belongs_to, relation_enum = "Boss", from = "boss_id", to = "id")]
    pub boss: HasOne<super::user::Entity>,
    #[serde(skip)]
    #[sea_orm(belongs_to, relation_enum = "Staff", from = "staff_id", to = "id")]
    pub staff: HasOne<super::user::Entity>,
}

impl ActiveModelBehavior for ActiveModel {}
