use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use schemars::JsonSchema;
use serde::Serialize;
use uuid::Uuid;

/// 老板首页概览数据
#[derive(Debug, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct BossOverview {
    /// 待审批计件记录数
    pub pending_count: i64,
    /// 进行中订单数
    pub processing_order_count: i64,
    /// 今日完成数量
    pub today_quantity: i64,
    /// 今日完成金额（元）
    pub today_amount: Decimal,
    /// 本月完成数量
    pub month_quantity: i64,
    /// 本月完成金额（元）
    pub month_amount: Decimal,
    /// 员工总数
    pub staff_count: i64,
}

/// 员工首页概览数据
#[derive(Debug, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct StaffOverview {
    /// 本月完成数量
    pub month_quantity: i64,
    /// 本月完成金额（元）
    pub month_amount: Decimal,
}

/// 动态类型
#[derive(Debug, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub enum ActivityType {
    /// 提交计件记录
    Submit,
    /// 审批通过
    Approve,
    /// 审批驳回
    Reject,
}

/// 动态项
#[derive(Debug, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct Activity {
    /// 动态ID（计件记录ID）
    pub id: Uuid,
    /// 动态类型
    pub activity_type: ActivityType,
    /// 用户姓名
    pub user_name: String,
    /// 订单/产品名称
    pub order_name: String,
    /// 订单图片URL
    pub order_image: Option<String>,
    /// 工序名称
    pub process_name: String,
    /// 计件数量
    pub quantity: i32,
    /// 创建时间
    pub created_at: DateTime<Utc>,
}

/// 动态列表
#[derive(Debug, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ActivityList {
    /// 动态列表
    pub list: Vec<Activity>,
}
