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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SearchItemKind {
    Teacher,
    Group,
    Audience,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(transparent)]
pub struct SearchValue(String);

impl SearchValue {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for SearchValue {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
