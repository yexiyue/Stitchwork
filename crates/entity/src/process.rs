use rust_decimal::Decimal;
use schemars::JsonSchema;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

/// 加工工序，订单中的一道加工步骤
#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
#[sea_orm(table_name = "process")]
pub struct Model {
    /// 工序唯一标识符
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    /// 所属订单 ID
    pub order_id: Uuid,
    /// 工序所属老板的用户 ID
    pub boss_id: Uuid,
    /// 工序名称，如"裁剪"、"缝纫"、"熨烫"等
    pub name: String,
    /// 工序描述/备注
    pub description: Option<String>,
    /// 计件单价（元），员工每完成一件可获得的报酬
    #[sea_orm(column_type = "Decimal(Some((10, 2)))")]
    pub piece_price: Decimal,
    /// 工序最后更新时间
    #[sea_orm(auto_update, default_expr = "Expr::current_timestamp()")]
    pub updated_at: DateTimeUtc,

    #[serde(skip)]
    #[sea_orm(belongs_to, from = "order_id", to = "id")]
    pub order: HasOne<super::order::Entity>,

    #[serde(skip)]
    #[sea_orm(has_many)]
    pub piece_records: HasMany<super::piece_record::Entity>,
}

impl ActiveModelBehavior for ActiveModel {}
