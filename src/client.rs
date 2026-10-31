use serde::{Serialize, de::DeserializeOwned};

use crate::{
    errors::Error,
    requests::{Action, RequestQuery, ScheduleRequest, SearchRequest},
    responses::{ScheduleData, SearchItem},
    types::{ApiResponse, ApiStatus},
};

const API_URL: &str = "https://mtuci.ru/bitrix/services/main/ajax.php";
const USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "-rs/", env!("CARGO_PKG_VERSION"));

/// Клиент для работы с API МТУСИ.
///
/// Для HTTP-запросов использует [`reqwest::Client`].
///
/// Все публичные методы клиента асинхронные.
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
    /// Создаёт клиент API с настройками HTTP по умолчанию.
    ///
    /// Для HTTP-запросов создаётся новый [`reqwest::Client`].
    ///
    /// # Panics
    ///
    /// Может паниковать, если не удаётся инициализировать HTTP-клиент.
    pub fn new() -> Self {
        Self {
            http: reqwest::Client::new(),
        }
    }

    /// Создаёт клиент API на основе готового HTTP-клиента.
    ///
    /// # Аргументы
    ///
    /// * `http` — предварительно настроенный HTTP-клиент.
    pub fn from_client(http: reqwest::Client) -> Self {
        Self { http }
    }

    async fn post<B, T>(&self, action: Action, body: B) -> Result<T, Error>
    where
        B: Serialize,
        T: DeserializeOwned,
    {
        let response: ApiResponse<T> = self
            .http
            .post(API_URL)
            .header(reqwest::header::USER_AGENT, USER_AGENT)
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

    /// Ищет группы, преподавателей и аудитории по заданной строке.
    ///
    /// Поиск выполняется через API МТУСИ. Возвращённые элементы содержат
    /// название объекта, его тип и значение, необходимое для запроса расписания.
    ///
    /// # Аргументы
    ///
    /// * `search_request` — запрос с текстом для поиска.
    pub async fn search(
        &self,
        search_request: SearchRequest<'_>,
    ) -> Result<Vec<SearchItem>, Error> {
        self.post(Action::GetItemsListBySearch, search_request)
            .await
    }

    /// Получает расписание для указанного объекта на месяц из запроса.
    ///
    /// # Аргументы
    ///
    /// * `schedule_request` — запрос с объектом, месяцем и типом объекта.
    pub async fn get_schedule(
        &self,
        schedule_request: ScheduleRequest<'_>,
    ) -> Result<ScheduleData, Error> {
        self.post(Action::GetTimetableByValue, schedule_request)
            .await
    }
}
