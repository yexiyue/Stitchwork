use sea_orm::entity::prelude::*;

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "customer")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    pub name: String,
    pub phone: Option<String>,
    pub created_at: DateTime,

    #[sea_orm(has_many)]
    pub orders: HasMany<super::order::Entity>,
}

impl ActiveModelBehavior for ActiveModel {}
