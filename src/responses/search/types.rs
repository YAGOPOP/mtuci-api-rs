use crate::types::{SearchItemKind, SearchValue};
use serde::Deserialize;

/// Элемент, найденный при поиске в API МТУСИ.
///
/// Содержит отображаемое название объекта, его тип и значение,
/// которое используется для запроса расписания.
#[derive(Debug, Deserialize)]
pub struct SearchItem {
    #[serde(rename = "UF_NAME")]
    name: String,

    #[serde(rename = "type")]
    kind: SearchItemKind,

    value: SearchValue,
}

impl SearchItem {
    /// Возвращает название найденного объекта.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Возвращает тип найденного объекта.
    pub fn kind(&self) -> SearchItemKind {
        self.kind
    }

    /// Возвращает значение объекта для запроса расписания.
    ///
    /// Полученное значение можно передать в [`crate::ScheduleRequest`].
    pub fn value(&self) -> &SearchValue {
        &self.value
    }
}
