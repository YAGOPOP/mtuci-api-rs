use chrono::{Datelike, Local, Month};
use serde::{Serialize, de::DeserializeOwned};

use crate::{
    errors::Error,
    responses::{ScheduleData, SearchItem},
    types::{ApiResponse, ApiStatus},
};

use crate::requests::{Action, RequestQuery, ScheduleRequest, SearchRequest};

const API_URL: &str = "https://mtuci.ru/bitrix/services/main/ajax.php";

#[derive(Clone, Debug)]
pub struct Client {
    http: reqwest::Client,
}

impl Default for Client {
    fn default() -> Self {
        Self::new()
    }
}

impl Client {
    pub fn new() -> Self {
        Self {
            http: reqwest::Client::new(),
        }
    }

    pub fn from_client(http: reqwest::Client) -> Self {
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
