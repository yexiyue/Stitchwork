use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProcessProgress {
    pub process_id: Uuid,
    pub name: String,
    pub completed_quantity: i64,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderStats {
    pub order_id: Uuid,
    pub total_quantity: i32,
    pub completed_quantity: i64,
    pub progress: f64,
    pub processes: Vec<ProcessProgress>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CustomerSummary {
    pub customer_id: Uuid,
    pub customer_name: String,
    pub total_orders: i64,
    pub pending_orders: i64,
    pub processing_orders: i64,
    pub completed_orders: i64,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CustomerSummaryList {
    pub list: Vec<CustomerSummary>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkerProduction {
    pub user_id: Uuid,
    pub user_name: String,
    pub total_quantity: i64,
    pub total_amount: Decimal,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkerProductionList {
    pub list: Vec<WorkerProduction>,
}

#[derive(Debug, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct WorkerStatsParams {
    pub start_date: Option<String>,
    pub end_date: Option<String>,
}

// Daily stats for trend chart
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DailyStat {
    pub date: String,
    pub total_quantity: i64,
    pub total_amount: Decimal,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DailyStatsList {
    pub list: Vec<DailyStat>,
}

// Group stats for stacked bar chart
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GroupStat {
    pub id: Uuid,
    pub name: String,
    pub total_quantity: i64,
    pub total_amount: Decimal,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GroupStatsList {
    pub list: Vec<GroupStat>,
}

// ============ Order Stats ============

#[derive(Debug, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct OrderStatsParams {
    pub start_date: Option<String>,
    pub end_date: Option<String>,
}

/// 订单概览统计
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderOverview {
    pub total_orders: i64,
    pub pending_orders: i64,
    pub processing_orders: i64,
    pub completed_orders: i64,
    pub delivered_orders: i64,
    pub cancelled_orders: i64,
    pub total_quantity: i64,
    pub total_amount: Decimal,
}

/// 月度订单趋势
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MonthlyOrderStat {
    pub month: String, // YYYY-MM
    pub order_count: i64,
    pub total_quantity: i64,
    pub total_amount: Decimal,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MonthlyOrderStatsList {
    pub list: Vec<MonthlyOrderStat>,
}

/// 客户贡献度
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CustomerContribution {
    pub customer_id: Uuid,
    pub customer_name: String,
    pub order_count: i64,
    pub total_quantity: i64,
    pub total_amount: Decimal,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CustomerContributionList {
    pub list: Vec<CustomerContribution>,
}

/// 订单进度概览
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderProgressItem {
    pub order_id: Uuid,
    pub product_name: String,
    pub customer_name: String,
    pub total_quantity: i32,
    pub completed_quantity: i64,
    pub progress: f64,
    pub status: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderProgressList {
    pub list: Vec<OrderProgressItem>,
}

/// 每日订单趋势
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DailyOrderStat {
    pub date: String, // YYYY-MM-DD
    pub order_count: i64,
    pub total_quantity: i64,
    pub total_amount: Decimal,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DailyOrderStatsList {
    pub list: Vec<DailyOrderStat>,
}
