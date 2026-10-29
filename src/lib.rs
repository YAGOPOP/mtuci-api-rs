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
