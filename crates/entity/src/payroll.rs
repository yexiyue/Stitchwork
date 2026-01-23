use rust_decimal::Decimal;
use schemars::JsonSchema;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

/// 工资发放记录
#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
#[sea_orm(table_name = "payroll")]
pub struct Model {
    /// 工资记录唯一标识符
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    /// 收款员工的用户 ID
    pub user_id: Uuid,
    /// 发放工资的老板用户 ID
    pub boss_id: Uuid,
    /// 发放金额（元）
    #[sea_orm(column_type = "Decimal(Some((10, 2)))")]
    pub amount: Decimal,
    /// 支付凭证图片 URL
    pub payment_image: Option<String>,
    /// 备注说明
    pub note: Option<String>,
    /// 发放时间
    pub paid_at: DateTimeUtc,

    #[serde(skip)]
    #[sea_orm(belongs_to, from = "user_id", to = "id")]
    pub user: HasOne<super::user::Entity>,

    #[serde(skip)]
    #[sea_orm(has_many, via = "payroll_record")]
    pub price_records: HasMany<super::piece_record::Entity>,
}

impl ActiveModelBehavior for ActiveModel {}
