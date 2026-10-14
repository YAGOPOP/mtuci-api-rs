use std::{collections::HashMap, sync::LazyLock};
use serde::Deserialize;

use url::Url;

const API_ENDPOINT: LazyLock<Url> = LazyLock::new(|| Url::parse("https://mtuci.ru/bitrix/services/main/ajax.php").expect("Invalid API endpoint"));

#[derive(Debug, Deserialize)]
pub struct ScheduleResponse {
    status: String,
    data: ScheduleResponseData
}

#[derive(Debug, Deserialize)]
pub struct ScheduleResponseData {
    days: HashMap<String, Vec<ScheduleClass>>
}

#[derive(Debug, Deserialize)]
pub struct ScheduleClass {
    #[serde(rename = "ID")]
    id: String,
    #[serde(rename = "UF_DAY")]
    day: String,
    #[serde(rename = "UF_NUMBER")]
    number: String,
    #[serde(rename = "UF_DISCIPLINE")]
    discipline: String,
    #[serde(rename = "UF_TIME_START")]
    time_start: String,
    #[serde(rename = "UF_TIME_END")]
    time_end: String,
    #[serde(rename = "UF_TEACHER")]
    teacher: Vec<String>,
    #[serde(rename = "UF_AUDIENCE")]
    audience: Vec<String>,
    #[serde(rename = "UF_TYPE")]
    class_type: String,
    #[serde(rename = "UF_IS_RETAKE")]
    is_retake: String,
    #[serde(rename = "UF_GROUP")]
    group: Vec<String>,
    #[serde(rename = "UF_IS_ONLINE")]
    is_online: String,
    #[serde(rename = "UF_DATE")]
    date: String,
    #[serde(rename = "UF_IS_FORM_CONTROL")]
    is_form_control: String,
}
