use serde::Deserialize;
use serde_json::Value;
use serde_with::{DisplayFromStr, serde_as};

#[derive(Debug, Deserialize)]
pub struct RoomsInfo {
    rooms: Vec<Room>,
}

#[serde_as]
#[derive(Debug, Deserialize)]
pub struct Room {
    #[serde_as(deserialize_as = "DisplayFromStr")]
    id: u32,

    #[serde_as(deserialize_as = "DisplayFromStr")]
    internal_id: u32,

    name: String,
    fullname: String,
    desc: String,
    table: RoomTable,
}

#[derive(Debug, Deserialize)]
pub struct RoomTable {
    links: Vec<String>,
    time: Vec<String>,
    phones: Vec<String>,
    mails: Vec<String>,
    funcs: Vec<Value>,
}
