use std::collections::BTreeMap;

use chrono::{DateTime, FixedOffset, NaiveDate, NaiveTime, Utc};
use serde::{Deserialize, Deserializer, de};
use serde_with::{DeserializeAs, DisplayFromStr, serde_as};

/// Расписание, возвращённое API за запрошенный месяц.
///
/// Содержит занятия, сгруппированные по датам, сведения о днях недели,
/// последовательность дат текущей недели и время обновления, указанное API.
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

impl ScheduleData {
    /// Возвращает занятия, сгруппированные по датам.
    ///
    /// Ключи — даты без времени (`NaiveDate`), значения — списки занятий
    /// в соответствующий день. Используется `BTreeMap`, поэтому даты
    /// расположены в порядке возрастания.
    pub fn days(&self) -> &BTreeMap<NaiveDate, Vec<ScheduleClass>> {
        &self.days
    }

    /// Возвращает сведения о днях недели, предоставленные API.
    pub fn week(&self) -> &[WeekDay] {
        &self.week
    }

    /// Возвращает последовательность дат недели, предоставленную API.
    pub fn week_dates(&self) -> &[NaiveDate] {
        &self.week_dates
    }

    /// Возвращает время обновления расписания, указанное API.
    ///
    /// Время представлено в UTC. Оно не гарантирует, что API действительно
    /// обновил расписание в этот момент: значение определяется ответом сервера.
    pub fn updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }
}

/// Одно занятие из расписания.
///
/// Содержит идентификаторы занятия и дня недели, номер пары, дисциплину,
/// время проведения, преподавателей, аудитории, тип занятия и дополнительные
/// признаки, предоставленные API.
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

impl ScheduleClass {
    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn week_day(&self) -> u8 {
        self.week_day
    }

    pub fn class_number(&self) -> u8 {
        self.class_number
    }

    pub fn discipline(&self) -> &str {
        &self.discipline
    }

    pub fn time_start(&self) -> NaiveTime {
        self.time_start
    }

    pub fn time_end(&self) -> NaiveTime {
        self.time_end
    }

    pub fn teacher(&self) -> &[String] {
        &self.teacher
    }

    pub fn audience(&self) -> &[String] {
        &self.audience
    }

    pub fn class_type(&self) -> &str {
        &self.class_type
    }

    pub fn is_retake(&self) -> bool {
        self.is_retake
    }

    pub fn group(&self) -> &[String] {
        &self.group
    }

    pub fn is_online(&self) -> bool {
        self.is_online
    }

    pub fn date(&self) -> DateTime<FixedOffset> {
        self.date
    }

    pub fn is_form_control(&self) -> bool {
        self.is_form_control
    }
}

/// Сведения об одном дне недели, возвращённые API.
#[serde_as]
#[derive(Debug, Deserialize)]
pub struct WeekDay {

    /// Возвращает календарную дату.
    #[serde_as(deserialize_as = "NaiveDateFromStr")]
    date: NaiveDate,

    /// Возвращает числовой идентификатор дня недели из API.
    ///
    /// Соответствие числовых значений дням недели определяется форматом ответа.
    weekday: u8,
}

impl WeekDay {
    pub fn date(&self) -> NaiveDate {
        self.date
    }

    pub fn weekday(&self) -> u8 {
        self.weekday
    }
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
