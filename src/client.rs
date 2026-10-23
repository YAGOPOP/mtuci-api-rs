use chrono::{Datelike, Local, Month};
use serde::{Serialize, de::DeserializeOwned};
use serde_json::value;

use crate::{
    errors::Error,
    responses::{ScheduleData, SearchItem},
    types::{ApiResponse, ApiStatus, SearchItemKind, SearchValue},
};

const API_URL: &str = "https://mtuci.ru/bitrix/services/main/ajax.php";

#[derive(Debug, Clone, Copy, Serialize)]
#[serde(rename_all = "camelCase")]
enum Action {
    GetItemsListBySearch,
    GetTimetableByValue,
}

#[derive(Debug, Serialize)]
pub struct RequestQuery {
    c: &'static str,
    action: Action,
    mode: &'static str,
}

impl RequestQuery {
    pub fn new(action: Action) -> Self {
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

struct Client {
    http: reqwest::Client,
}

impl Client {
    pub fn new() -> Client {
        Self {
            http: reqwest::Client::new(),
        }
    }

    pub fn from_client(http: reqwest::Client) -> Client {
        Self { http }
    }

    async fn post<B, T>(&self, action: Action, body: B) -> Result<T, Error>
    where
        B: Serialize, // + ?Sized
        T: DeserializeOwned,
    {
        let response: ApiResponse<T> = self
            .http
            .post(API_URL)
            .query(&RequestQuery::new(action))
            .form(&body)
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?;

        match response.status {
            ApiStatus::Success => response.data.ok_or(Error::MissingData),
            ApiStatus::Error => Err(Error::Api(response.errors)),
        }
    }

    pub async fn search(
        &self,
        search_request: SearchRequest<'_>,
    ) -> Result<Vec<SearchItem>, Error> {
        self.post(Action::GetItemsListBySearch, search_request)
            .await
    }

    pub async fn get_schedule(
        &self,
        schedule_request: ScheduleRequest<'_>,
    ) -> Result<ScheduleData, Error> {
        self.post(Action::GetTimetableByValue, schedule_request)
            .await
    }
}
