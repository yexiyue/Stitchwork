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
