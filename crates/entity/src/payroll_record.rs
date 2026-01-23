use schemars::JsonSchema;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

/// 工资-计件关联记录，记录一次工资发放包含哪些计件记录
#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
#[sea_orm(table_name = "payroll_record")]
pub struct Model {
    /// 关联记录唯一标识符
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    /// 工资发放记录 ID
    pub payroll_id: Uuid,
    /// 计件记录 ID
    pub piece_record_id: Uuid,

    #[serde(skip)]
    #[sea_orm(belongs_to, from = "payroll_id", to = "id")]
    pub payroll: HasOne<super::payroll::Entity>,

    #[serde(skip)]
    #[sea_orm(belongs_to, from = "piece_record_id", to = "id")]
    pub piece_record: HasOne<super::piece_record::Entity>,
}

impl ActiveModelBehavior for ActiveModel {}
