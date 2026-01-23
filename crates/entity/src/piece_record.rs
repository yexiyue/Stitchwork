use rust_decimal::Decimal;
use schemars::JsonSchema;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};

/// 计件记录状态
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
    EnumString,
    Display,
    DeriveValueType,
    JsonSchema,
)]
#[serde(rename_all = "camelCase")]
#[strum(serialize_all = "camelCase")]
#[sea_orm(value_type = "String")]
pub enum PieceRecordStatus {
    /// 待审核，员工提交后等待老板审核
    Pending,
    /// 已通过，老板审核通过
    Approved,
    /// 已拒绝，老板审核拒绝
    Rejected,
    /// 已结算，已计入工资发放
    Settled,
}

/// 计件记录者
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
    EnumString,
    Display,
    DeriveValueType,
    JsonSchema,
)]
#[serde(rename_all = "camelCase")]
#[strum(serialize_all = "camelCase")]
#[sea_orm(value_type = "String")]
pub enum RecordedBy {
    /// 员工自己录入
    BySelf,
    /// 老板代为录入
    ByBoss,
}

/// 计件记录，记录员工完成某道工序的数量
#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
#[sea_orm(table_name = "piece_record")]
pub struct Model {
    /// 计件记录唯一标识符
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    /// 所属工序 ID
    pub process_id: Uuid,
    /// 完成该计件的员工用户 ID
    pub user_id: Uuid,
    /// 计件记录所属老板的用户 ID
    pub boss_id: Uuid,
    /// 完成数量
    pub quantity: i32,
    /// 计件金额（元），数量 × 工序单价
    #[sea_orm(column_type = "Decimal(Some((10, 2)))")]
    pub amount: Decimal,
    /// 计件记录状态
    pub status: PieceRecordStatus,
    /// 记录者：员工自己或老板代录
    pub recorded_by: RecordedBy,
    /// 录入时间
    pub recorded_at: DateTimeUtc,
    /// 记录最后更新时间（如审核状态变更）
    #[sea_orm(auto_update, default_expr = "Expr::current_timestamp()")]
    pub updated_at: DateTimeUtc,

    #[serde(skip)]
    #[sea_orm(belongs_to, from = "process_id", to = "id")]
    pub process: HasOne<super::process::Entity>,

    #[serde(skip)]
    #[sea_orm(belongs_to, from = "user_id", to = "id")]
    pub user: HasOne<super::user::Entity>,

    #[serde(skip)]
    #[sea_orm(has_one, via = "payroll_record")]
    pub payroll: HasOne<super::payroll::Entity>,
}

impl ActiveModelBehavior for ActiveModel {}
