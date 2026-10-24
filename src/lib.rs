mod client;
mod errors;
mod requests;
mod responses;
mod types;

pub use client::Client;

pub use requests::{ScheduleRequest, SearchRequest};

pub use errors::Error;

pub use types::{SearchItemKind, SearchValue, ApiError};

pub use responses::{
    ScheduleClass,
    ScheduleData,
    SearchItem,
    WeekDay
};
