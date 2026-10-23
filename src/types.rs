use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ApiStatus {
    Success,
    Error,
}

#[derive(Debug, Deserialize)]
pub struct ApiError {
    pub message: String,
    pub code: i64,

    #[serde(rename = "customData")]
    pub custom_data: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize)]
pub struct ApiResponse<T> {
    pub status: ApiStatus,
    pub data: Option<T>,
    pub errors: Vec<ApiError>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum SearchItemKind {
    Teacher,
    Group,
    Audience,
}
