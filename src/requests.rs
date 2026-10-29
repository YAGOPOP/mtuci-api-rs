use chrono::{Datelike, FixedOffset, Month, Utc};
use serde::Serialize;

use crate::{
    responses::SearchItem,
    types::{SearchItemKind, SearchValue},
};

#[derive(Debug, Clone, Copy, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) enum Action {
    GetItemsListBySearch,
    GetTimetableByValue,
}

#[derive(Debug, Serialize)]
pub(crate) struct RequestQuery {
    c: &'static str,
    action: Action,
    mode: &'static str,
}

impl RequestQuery {
    pub(crate) fn new(action: Action) -> Self {
        Self {
            c: "mtuci:timetable",
            action,
            mode: "class",
        }
    }
}

/// Параметры поиска группы, преподавателя или аудитории.
///
/// Хранит заимствованную строку поиска и сериализуется в формат запроса
/// API.
///
/// Запрос не проверяет строку на корректность и не гарантирует,
/// что API найдёт совпадения.
#[derive(Debug, Serialize)]
#[serde(rename_all = "UPPERCASE")]
pub struct SearchRequest<'a> {
    name: &'a str,
}

impl<'a> SearchRequest<'a> {
    /// Создаёт запрос поиска по заданной строке.
    ///
    /// # Аргументы
    ///
    /// * `name` — текст для поиска.
    pub fn new(name: &'a str) -> Self {
        Self { name }
    }
}

/// Параметры запроса расписания.
///
/// Содержит значение объекта, месяц и тип объекта. При сериализации
/// используются поля `VALUE`, `MONTH` и `TYPE`.
///
/// Нумерация месяцев в запросе начинается с нуля: январь — `0`,
/// декабрь — `11`. Для создания запроса используется [`chrono::Month`],
/// поэтому вручную преобразовывать месяц обычно не требуется.
///
/// Значение объекта можно взять из [`crate::SearchItem::value`].
#[derive(Debug, Serialize)]
#[serde(rename_all = "UPPERCASE")]
pub struct ScheduleRequest<'a> {
    value: &'a SearchValue,
    month: u8,

    #[serde(rename = "TYPE")]
    kind: SearchItemKind,
}

/// Параметры запроса расписания.
impl<'a> ScheduleRequest<'a> {
    /// Создаёт запрос расписания для объекта на указанный месяц.
    ///
    /// # Аргументы
    ///
    /// * `search_value` — значение объекта, используемое API для поиска его расписания.
    /// * `month` — месяц, для которого требуется расписание.
    /// * `kind` — тип объекта: группа, преподаватель или аудитория.
    pub fn new(search_value: &'a SearchValue, month: Month, kind: SearchItemKind) -> Self {
        Self {
            value: search_value,
            month: (month.number_from_month() - 1) as u8,
            kind,
        }
    }

    /// Создаёт запрос расписания для текущего месяца.
    ///
    /// Текущий месяц определяется по фиксированному смещению московского времени
    /// UTC+3.
    ///
    /// # Аргументы
    ///
    /// * `search_value` — значение объекта, используемое API для поиска расписания.
    /// * `kind` — тип объекта.
    pub fn new_with_current_month(search_value: &'a SearchValue, kind: SearchItemKind) -> Self {
        let moscow_time_offset = FixedOffset::east_opt(3 * 3600).expect("valid Moscow time offset");
        let currrent_month = Utc::now().with_timezone(&moscow_time_offset).month0();
        Self {
            value: search_value,
            month: currrent_month as u8,
            kind,
        }
    }

    /// Создаёт запрос расписания для результата поиска на указанный месяц.
    ///
    /// Тип объекта и значение для запроса берутся из [`SearchItem`].
    ///
    /// # Аргументы
    ///
    /// * `item` — результат поиска объекта.
    /// * `month` — месяц, для которого требуется расписание.
    pub fn new_for_search_item(item: &'a SearchItem, month: Month) -> Self {
        Self::new(item.value(), month, item.kind())
    }

    /// Создаёт запрос расписания для результата поиска на текущий месяц.
    ///
    /// # Аргументы
    ///
    /// * `item` — результат поиска объекта.
    pub fn new_for_search_item_with_current_month(item: &'a SearchItem) -> Self {
        Self::new_with_current_month(item.value(), item.kind())
    }
}
