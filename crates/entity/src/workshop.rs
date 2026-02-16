use schemars::JsonSchema;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

/// 工坊/车间，服装加工的工作单位
#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
#[sea_orm(table_name = "workshop")]
pub struct Model {
    /// 工坊唯一标识符
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    /// 工坊所有者（老板）的用户 ID
    pub owner_id: Uuid,
    /// 工坊名称
    pub name: String,
    /// 工坊描述
    pub desc: Option<String>,
    /// 工坊地址
    pub address: Option<String>,
    /// 工坊图片/Logo URL
    pub image: Option<String>,
    /// 计件单位，如"打"、"件"等
    #[sea_orm(default_value = "打")]
    pub piece_unit: String,
    /// 业务标签，如"工坊"、"车间"等
    #[sea_orm(default_value = "工坊")]
    pub business_label: String,
    /// 工坊创建时间
    #[sea_orm(default_expr = "Expr::current_timestamp()")]
    pub created_at: DateTimeUtc,
    /// 工坊信息最后更新时间
    #[sea_orm(auto_update, default_expr = "Expr::current_timestamp()")]
    pub updated_at: DateTimeUtc,

    #[serde(skip)]
    #[sea_orm(has_many, relation_enum = "HasStaffs", via_rel = "WorkshopMember")]
    pub staffs: HasMany<super::user::Entity>,
    #[serde(skip)]
    #[sea_orm(belongs_to, relation_enum = "HasOwner", from = "owner_id", to = "id")]
    pub boss: HasOne<super::user::Entity>,
}

impl ActiveModelBehavior for ActiveModel {}
