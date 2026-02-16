use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// 创建分享请求
#[derive(Debug, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateShareRequest {
    /// 分享标题
    pub title: String,
    /// 分享描述
    pub description: Option<String>,
    /// 关联的订单ID列表
    pub order_ids: Vec<Uuid>,
    /// 关联的工序ID列表
    pub process_ids: Vec<Uuid>,
}

/// 更新分享请求
#[derive(Debug, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct UpdateShareRequest {
    /// 分享标题
    pub title: Option<String>,
    /// 分享描述
    pub description: Option<String>,
    /// 关联的订单ID列表
    pub order_ids: Option<Vec<Uuid>>,
    /// 关联的工序ID列表
    pub process_ids: Option<Vec<Uuid>>,
    /// 是否启用
    pub is_active: Option<bool>,
}

/// 公开分享页面响应（无需登录）
#[derive(Debug, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct PublicShareResponse {
    /// 分享标题
    pub title: String,
    /// 分享描述
    pub description: Option<String>,
    /// 工坊名称
    pub workshop_name: Option<String>,
    /// 工坊描述
    pub workshop_desc: Option<String>,
    /// 工坊地址
    pub workshop_address: Option<String>,
    /// 工坊图片URL
    pub workshop_image: Option<String>,
    /// 计件单位
    pub piece_unit: String,
    /// 老板联系电话
    pub boss_phone: Option<String>,
    /// 老板头像URL
    pub avatar: Option<String>,
    /// 可接工序列表
    pub processes: Vec<PublicProcessInfo>,
}

/// 公开工序信息
#[derive(Debug, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct PublicProcessInfo {
    /// 工序ID
    pub id: Uuid,
    /// 工序名称
    pub name: String,
    /// 工序描述
    pub description: Option<String>,
    /// 计件单价（元）
    pub piece_price: rust_decimal::Decimal,
    /// 订单产品名称
    pub order_product_name: String,
    /// 订单产品图片URL列表
    pub order_images: Vec<String>,
    /// 剩余可接数量
    pub remaining_quantity: i32,
}
