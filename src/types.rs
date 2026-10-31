use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub(crate) enum ApiStatus {
    Success,
    Error,
}

/// Ошибка, возвращённая API МТУСИ.
///
/// Содержит сообщение, код ошибки и дополнительные данные,
/// если они присутствовали в ответе.
#[derive(Debug, Deserialize)]
pub struct ApiError {
    /// Текстовое описание ошибки, предоставленное API.
    pub message: String,

    /// Числовой код ошибки, предоставленный API.
    pub code: i64,

    /// Дополнительные данные ошибки из поля `customData`.
    ///
    /// Формат содержимого не исследован.
    #[serde(rename = "customData")]
    pub custom_data: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct ApiResponse<T> {
    pub status: ApiStatus,
    pub data: Option<T>,
    pub errors: Vec<ApiError>,
}

/// Тип объекта, найденного при поиске.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SearchItemKind {
    Teacher,
    Group,
    Audience,
}

/// Значение объекта поиска, используемое API при запросе расписания.
///
/// Обёртка над строкой, которую API возвращает в поле `value` результата
/// поиска. Значение предназначено для передачи в [`crate::ScheduleRequest`].
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(transparent)]
pub struct SearchValue(String);

impl SearchValue {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for SearchValue {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
