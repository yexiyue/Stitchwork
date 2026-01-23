use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateThreadDto {
    pub title: Option<String>,
    pub archived: Option<bool>,
}
