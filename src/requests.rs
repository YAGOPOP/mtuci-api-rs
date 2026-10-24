use chrono::{Datelike, Local, Month};
use serde::{Serialize, de::DeserializeOwned};

use crate::{
    errors::Error,
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

#[derive(Debug, Serialize)]
#[serde(rename_all = "UPPERCASE")]
pub struct SearchRequest<'a> {
    name: &'a str,
}

impl<'a> SearchRequest<'a> {
    pub fn new(name: &'a str) -> Self {
        Self { name }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "UPPERCASE")]
pub struct ScheduleRequest<'a> {
    value: &'a SearchValue,
    month: u8,

    #[serde(rename = "TYPE")]
    kind: &'a SearchItemKind,
}

impl<'a> ScheduleRequest<'a> {
    pub fn new(search_value: &'a SearchValue, month: Month, kind: &'a SearchItemKind) -> Self {
        Self {
            value: search_value,
            month: (month.number_from_month() - 1) as u8,
            kind,
        }
    }

    pub fn new_with_current_month(search_value: &'a SearchValue, kind: &'a SearchItemKind) -> Self {
        Self {
            value: search_value,
            month: Local::now().month0() as u8,
            kind,
        }
    }

    pub fn for_item(item: &'a SearchItem, month: Month) -> Self {
        Self::new(&item.value, month, &item.kind)
    }

    pub fn for_item_current_month(item: &'a SearchItem) -> Self {
        Self::new_with_current_month(&item.value, &item.kind)
    }
}
