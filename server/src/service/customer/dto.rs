use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateCustomerDto {
    pub name: String,
    pub phone: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateCustomerDto {
    pub name: Option<String>,
    pub phone: Option<String>,
    pub description: Option<String>,
}
