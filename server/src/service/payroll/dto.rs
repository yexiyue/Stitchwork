use rust_decimal::Decimal;
use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreatePayrollDto {
    pub user_id: Uuid,
    pub amount: Decimal,
    pub note: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdatePayrollDto {
    pub amount: Option<Decimal>,
    pub note: Option<String>,
}
