use rust_decimal::Decimal;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// 工序进度统计
#[derive(Debug, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ProcessProgress {
    /// 工序ID
    pub process_id: Uuid,
    /// 工序名称
    pub name: String,
    /// 已完成数量
    pub completed_quantity: i64,
}

/// 订单统计详情
#[derive(Debug, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct OrderStats {
    /// 订单ID
    pub order_id: Uuid,
    /// 订单总数量
    pub total_quantity: i32,
    /// 已完成数量
    pub completed_quantity: i64,
    /// 完成进度（0-100）
    pub progress: f64,
    /// 各工序进度列表
    pub processes: Vec<ProcessProgress>,
}

/// 客户订单汇总
#[derive(Debug, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct CustomerSummary {
    /// 客户ID
    pub customer_id: Uuid,
    /// 客户名称
    pub customer_name: String,
    /// 订单总数
    pub total_orders: i64,
    /// 待处理订单数
    pub pending_orders: i64,
    /// 进行中订单数
    pub processing_orders: i64,
    /// 已完成订单数
    pub completed_orders: i64,
}

/// 客户订单汇总列表
#[derive(Debug, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct CustomerSummaryList {
    /// 客户汇总列表
    pub list: Vec<CustomerSummary>,
}

/// 员工产量统计
#[derive(Debug, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct WorkerProduction {
    /// 员工ID
    pub user_id: Uuid,
    /// 员工姓名
    pub user_name: String,
    /// 总数量（件）
    pub total_quantity: i64,
    /// 总金额（元）
    pub total_amount: Decimal,
}

/// 员工产量统计列表
#[derive(Debug, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct WorkerProductionList {
    /// 员工产量列表
    pub list: Vec<WorkerProduction>,
}

/// 员工统计查询参数
#[derive(Debug, Deserialize, Default, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct WorkerStatsParams {
    /// 开始日期，格式 YYYY-MM-DD
    pub start_date: Option<String>,
    /// 结束日期，格式 YYYY-MM-DD
    pub end_date: Option<String>,
}

/// 每日统计（用于趋势图）
#[derive(Debug, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct DailyStat {
    /// 日期，格式 YYYY-MM-DD
    pub date: String,
    /// 当日总数量
    pub total_quantity: i64,
    /// 当日总金额
    pub total_amount: Decimal,
}

/// 每日统计列表
#[derive(Debug, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct DailyStatsList {
    /// 每日统计列表
    pub list: Vec<DailyStat>,
}

/// 分组统计（用于堆叠柱状图）
#[derive(Debug, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct GroupStat {
    /// 分组ID
    pub id: Uuid,
    /// 分组名称
    pub name: String,
    /// 总数量
    pub total_quantity: i64,
    /// 总金额
    pub total_amount: Decimal,
}

/// 分组统计列表
#[derive(Debug, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct GroupStatsList {
    /// 分组统计列表
    pub list: Vec<GroupStat>,
}

// ============ Order Stats ============

/// 订单统计查询参数
#[derive(Debug, Deserialize, Default, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct OrderStatsParams {
    /// 开始日期，格式 YYYY-MM-DD
    pub start_date: Option<String>,
    /// 结束日期，格式 YYYY-MM-DD
    pub end_date: Option<String>,
}

/// 订单概览统计
#[derive(Debug, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct OrderOverview {
    /// 订单总数
    pub total_orders: i64,
    /// 待处理订单数
    pub pending_orders: i64,
    /// 进行中订单数
    pub processing_orders: i64,
    /// 已完成订单数
    pub completed_orders: i64,
    /// 已出货订单数
    pub delivered_orders: i64,
    /// 已取消订单数
    pub cancelled_orders: i64,
    /// 总数量
    pub total_quantity: i64,
    /// 总金额
    pub total_amount: Decimal,
}

/// 月度订单趋势
#[derive(Debug, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct MonthlyOrderStat {
    /// 月份，格式 YYYY-MM
    pub month: String,
    /// 订单数量
    pub order_count: i64,
    /// 总数量
    pub total_quantity: i64,
    /// 总金额
    pub total_amount: Decimal,
}

/// 月度订单趋势列表
#[derive(Debug, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct MonthlyOrderStatsList {
    /// 月度统计列表
    pub list: Vec<MonthlyOrderStat>,
}

/// 客户贡献度
#[derive(Debug, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct CustomerContribution {
    /// 客户ID
    pub customer_id: Uuid,
    /// 客户名称
    pub customer_name: String,
    /// 订单数量
    pub order_count: i64,
    /// 总数量
    pub total_quantity: i64,
    /// 总金额
    pub total_amount: Decimal,
}

/// 客户贡献度列表
#[derive(Debug, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct CustomerContributionList {
    /// 客户贡献度列表
    pub list: Vec<CustomerContribution>,
}

/// 订单进度概览
#[derive(Debug, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct OrderProgressItem {
    /// 订单ID
    pub order_id: Uuid,
    /// 产品名称
    pub product_name: String,
    /// 客户名称
    pub customer_name: String,
    /// 订单总数量
    pub total_quantity: i32,
    /// 已完成数量
    pub completed_quantity: i64,
    /// 完成进度（0-100）
    pub progress: f64,
    /// 订单状态
    pub status: String,
}

/// 订单进度列表
#[derive(Debug, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct OrderProgressList {
    /// 订单进度列表
    pub list: Vec<OrderProgressItem>,
}

/// 每日订单趋势
#[derive(Debug, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct DailyOrderStat {
    /// 日期，格式 YYYY-MM-DD
    pub date: String,
    /// 订单数量
    pub order_count: i64,
    /// 总数量
    pub total_quantity: i64,
    /// 总金额
    pub total_amount: Decimal,
}

/// 每日订单趋势列表
#[derive(Debug, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct DailyOrderStatsList {
    /// 每日订单统计列表
    pub list: Vec<DailyOrderStat>,
}
