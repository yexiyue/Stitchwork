use schemars::JsonSchema;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};

/// 用户角色
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
pub enum Role {
    /// 老板，工坊所有者，可管理订单、工序、员工和工资
    Boss,
    /// 员工，工坊成员，可查看自己的计件记录和工资
    Staff,
}

/// 系统用户，包括老板和员工
#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
#[sea_orm(table_name = "user")]
pub struct Model {
    /// 用户唯一标识符
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    /// 用户名，用于登录，全局唯一
    #[sea_orm(unique)]
    pub username: String,
    #[serde(skip)]
    pub password_hash: String,
    /// 用户角色：boss（老板）或 staff（员工）
    pub role: Role,
    /// 显示名称，用于界面展示
    pub display_name: Option<String>,
    /// 手机号码，全局唯一，用于登录和联系
    #[sea_orm(unique)]
    pub phone: String,
    /// 用户头像 URL
    pub avatar: Option<String>,
    /// 所属工坊 ID，员工必须属于某个工坊
    pub workshop_id: Option<Uuid>,
    /// 是否为超级管理员
    #[sea_orm(default_value = "false")]
    pub is_super_admin: bool,
    /// 用户创建时间
    #[sea_orm(default_expr = "Expr::current_timestamp()")]
    pub created_at: DateTimeUtc,
    /// 用户信息最后更新时间
    #[sea_orm(auto_update, default_expr = "Expr::current_timestamp()")]
    pub updated_at: DateTimeUtc,

    #[serde(skip)]
    #[sea_orm(has_many)]
    pub customers: HasMany<super::customer::Entity>,

    #[serde(skip)]
    #[sea_orm(has_many)]
    pub piece_records: HasMany<super::piece_record::Entity>,

    #[serde(skip)]
    #[sea_orm(has_many)]
    pub payrolls: HasMany<super::payroll::Entity>,

    #[serde(skip)]
    #[sea_orm(
        belongs_to,
        relation_enum = "WorkshopMember",
        from = "workshop_id",
        to = "id"
    )]
    pub workshop: HasOne<super::workshop::Entity>,

    #[serde(skip)]
    #[sea_orm(has_many, relation_enum = "WorkshopOwner", via_rel = "HasOwner")]
    pub workshops: HasMany<super::workshop::Entity>,

    #[serde(skip)]
    #[sea_orm(has_many)]
    pub threads: HasMany<super::chat_thread::Entity>,
}

impl ActiveModelBehavior for ActiveModel {}
