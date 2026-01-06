use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[sea_orm(table_name = "payroll_record")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub payroll_id: Uuid,
    pub piece_record_id: Uuid,

    #[sea_orm(belongs_to, from = "payroll_id", to = "id")]
    pub payroll: HasOne<super::payroll::Entity>,

    #[sea_orm(belongs_to, from = "piece_record_id", to = "id")]
    pub piece_record: HasOne<super::piece_record::Entity>,
}

impl ActiveModelBehavior for ActiveModel {}
