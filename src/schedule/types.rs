use std::{collections::BTreeMap, sync::LazyLock};
// use serde::Deserialize;

use chrono::{DateTime, FixedOffset, NaiveDate, NaiveTime, Utc};
use serde::{Deserialize, Deserializer, de};
use serde_with::{DeserializeAs, DisplayFromStr, serde_as};

use url::Url;

const API_ENDPOINT: LazyLock<Url> = LazyLock::new(|| {
    Url::parse("https://mtuci.ru/bitrix/services/main/ajax.php").expect("Invalid API endpoint")
});

#[derive(Debug, Deserialize)]
pub struct ScheduleResponse {
    status: ResponseStatus,
    data: Option<ScheduleData>,
    errors: Vec<ErrorData>
}

    #[derive(Debug, Deserialize)]
pub enum ResponseStatus {
    #[serde(rename = "success")]
    Success,

    #[serde(rename = "error")]
    Error,
}

#[serde_as]
#[derive(Debug, Deserialize)]
pub struct ScheduleData {
    #[serde_as(deserialize_as = "BTreeMap<NaiveDateFromStr, _>")]
    days: BTreeMap<NaiveDate, Vec<ScheduleClass>>,

    week: Vec<WeekDay>,

    #[serde_as(deserialize_as = "Vec<NaiveDateFromStr>")]
    week_dates: Vec<NaiveDate>,

    #[serde(with = "chrono::serde::ts_seconds")]
    updated_at: DateTime<Utc>,
}

#[serde_as]
#[derive(Debug, Deserialize)]
pub struct ScheduleClass {
    #[serde_as(deserialize_as = "DisplayFromStr")]
    #[serde(rename = "ID")]
    id: u32,

    #[serde_as(deserialize_as = "DisplayFromStr")]
    #[serde(rename = "UF_DAY")]
    week_day: u8,

    #[serde_as(deserialize_as = "DisplayFromStr")]
    #[serde(rename = "UF_NUMBER")]
    class_number: u8,

    #[serde(rename = "UF_DISCIPLINE")]
    discipline: String,

    #[serde(rename = "UF_TIME_START")]
    time_start: NaiveTime,

    #[serde(rename = "UF_TIME_END")]
    time_end: NaiveTime,

    #[serde(rename = "UF_TEACHER")]
    teacher: Vec<String>,

    #[serde(rename = "UF_AUDIENCE")]
    audience: Vec<String>,

    #[serde(rename = "UF_TYPE")]
    class_type: String,

    #[serde(rename = "UF_IS_RETAKE", deserialize_with = "deser_bool")]
    is_retake: bool,

    #[serde(rename = "UF_GROUP")]
    group: Vec<String>,

    #[serde(rename = "UF_IS_ONLINE", deserialize_with = "deser_bool")]
    is_online: bool,

    #[serde(rename = "UF_DATE")]
    date: DateTime<FixedOffset>,

    #[serde(rename = "UF_IS_FORM_CONTROL", deserialize_with = "deser_bool")]
    is_form_control: bool,
}

#[serde_as]
#[derive(Debug, Deserialize)]
struct WeekDay {
    #[serde_as(deserialize_as = "NaiveDateFromStr")]
    date: NaiveDate,

    weekday: u8,
}

#[derive(Debug, Deserialize)]
pub struct ErrorData {
message: String,

/// seems like always 0????
code: i32,
// #[serde(rename = "customData")]
// custom_data: always null???
}

struct NaiveDateFromStr;

impl<'de> DeserializeAs<'de, NaiveDate> for NaiveDateFromStr {
    fn deserialize_as<D>(deserializer: D) -> Result<NaiveDate, D::Error>
    where
        D: Deserializer<'de>,
    {
        let date = String::deserialize(deserializer)?;
        NaiveDate::parse_from_str(&date, "%d.%m.%Y").map_err(de::Error::custom)
    }
}

fn deser_bool<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    match String::deserialize(deserializer)?.as_str() {
        "1" => Ok(true),
        "0" => Ok(false),
        other => Err(de::Error::custom(format!(
            "invalid boolean value: {other:?}"
        ))),
    }
}
