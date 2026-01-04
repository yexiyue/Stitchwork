use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::Serialize;
use uuid::Uuid;

/// Boss overview data
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BossOverview {
    pub pending_count: i64,
    pub processing_order_count: i64,
    pub today_quantity: i64,
    pub today_amount: Decimal,
    pub month_quantity: i64,
    pub month_amount: Decimal,
    pub staff_count: i64,
}

/// Staff overview data
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StaffOverview {
    pub month_quantity: i64,
    pub month_amount: Decimal,
}

/// Activity type
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ActivityType {
    Submit,
    Approve,
    Reject,
}

/// Activity item
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Activity {
    pub id: Uuid,
    #[serde(rename = "type")]
    pub activity_type: ActivityType,
    pub user_name: String,
    pub order_name: String,
    pub process_name: String,
    pub quantity: i32,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivityList {
    pub list: Vec<Activity>,
}
