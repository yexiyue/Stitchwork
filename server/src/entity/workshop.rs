use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[sea_orm(table_name = "workshop")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub owner_id: Uuid,
    pub name: String,
    pub desc: Option<String>,
    pub address: Option<String>,
    pub image: Option<String>,
    pub created_at: DateTimeUtc,

    #[serde(skip)]
    #[sea_orm(has_many, relation_enum = "HasStaffs", via_rel = "WorkshopMember")]
    pub staffs: HasMany<super::user::Entity>,

    #[sea_orm(belongs_to, relation_enum = "HasOwner", from = "owner_id", to = "id")]
    pub boss: HasOne<super::user::Entity>,
}

impl ActiveModelBehavior for ActiveModel {}
