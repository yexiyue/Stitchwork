use rust_decimal::Decimal;
use schemars::JsonSchema;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};

/// 订单状态
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
pub enum OrderStatus {
    /// 待处理，刚创建的订单
    Pending,
    /// 进行中，已开始加工
    Processing,
    /// 已完成，加工完成待出货
    Completed,
    /// 已出货，订单已交付客户
    Delivered,
    /// 已取消
    Cancelled,
}

impl OrderStatus {
    pub fn can_transition_to(&self, next: Self) -> bool {
        matches!(
            (self, next),
            (Self::Pending, Self::Processing)
                | (Self::Pending, Self::Cancelled)
                | (Self::Processing, Self::Completed)
                | (Self::Processing, Self::Cancelled)
                | (Self::Completed, Self::Delivered)
        )
    }
}

/// 服装加工订单
#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
#[sea_orm(table_name = "order")]
pub struct Model {
    /// 订单唯一标识符
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    /// 客户 ID
    pub customer_id: Uuid,
    /// 订单所属老板的用户 ID
    pub boss_id: Uuid,
    /// 产品名称
    pub product_name: String,
    /// 产品描述/备注
    pub description: Option<String>,
    /// 产品图片 URL 数组
    #[sea_orm(column_type = "JsonBinary", nullable)]
    pub images: Option<Json>,
    /// 订单数量
    pub quantity: i32,
    /// 单价（元）
    #[sea_orm(column_type = "Decimal(Some((10, 2)))")]
    pub unit_price: Decimal,
    /// 订单状态
    pub status: OrderStatus,
    /// 收货/拿货时间
    pub received_at: DateTimeUtc,
    /// 出货时间
    pub delivered_at: Option<DateTimeUtc>,
    /// 订单最后更新时间
    #[sea_orm(auto_update, default_expr = "Expr::current_timestamp()")]
    pub updated_at: DateTimeUtc,

    #[serde(skip)]
    #[sea_orm(belongs_to, from = "customer_id", to = "id")]
    pub customer: HasOne<super::customer::Entity>,

    #[serde(skip)]
    #[sea_orm(has_many)]
    pub processes: HasMany<super::process::Entity>,
}

impl ActiveModelBehavior for ActiveModel {}
