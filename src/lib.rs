//! Неофициальная обёртка для недокумментированного API МТУСИ.
//!
//! Библиотека позволяет искать группы, преподавателей и аудитории,
//! а также получать расписание найденного объекта на выбранный месяц.
//!
//! Для выполнения HTTP-запросов используется [`reqwest`]. Ответы API
//! преобразуются в типизированные структуры с помощью Serde.
//!
//! # Основные типы
//!
//! - [`Client`] — клиент для взаимодействия с API.
//! - [`SearchRequest`] — запрос поиска групп, преподавателей или аудиторий.
//! - [`ScheduleRequest`] — запрос расписания на определённый месяц.
//! - [`SearchItem`] — результат поиска, который можно использовать
//!   для получения расписания.
//! - [`ScheduleData`] и [`ScheduleClass`] — данные расписания.
//!
//! API не документирован официально, поэтому доступность методов и
//! структура ответов зависят от текущей реализации сайта.

mod client;
mod errors;
mod requests;
mod responses;
mod types;

pub use client::Client;
pub use errors::Error;
pub use requests::{ScheduleRequest, SearchRequest};
pub use responses::{Room, RoomTable, RoomsInfo};
pub use responses::{ScheduleClass, ScheduleData, SearchItem, WeekDay};
pub use types::{ApiError, SearchItemKind, SearchValue};
