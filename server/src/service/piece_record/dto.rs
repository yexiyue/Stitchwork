use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreatePieceRecordDto {
    pub process_id: Uuid,
    pub user_id: Uuid,
    pub quantity: i32,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdatePieceRecordDto {
    pub quantity: Option<i32>,
}
