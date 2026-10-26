use serde::Deserialize;
use serde_json::Value;
use serde_with::{DisplayFromStr, serde_as};

#[derive(Debug, Deserialize)]
pub struct RoomsInfo {
    rooms: Vec<Room>,
}

impl RoomsInfo {
    pub fn rooms(&self) -> &[Room] {
        &self.rooms
    }
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

impl Room {
    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn internal_id(&self) -> u32 {
        self.internal_id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn fullname(&self) -> &str {
        &self.fullname
    }

    pub fn desc(&self) -> &str {
        &self.desc
    }

    pub fn table(&self) -> &RoomTable {
        &self.table
    }
}

#[derive(Debug, Deserialize)]
pub struct RoomTable {
    links: Vec<String>,
    time: Vec<String>,
    phones: Vec<String>,
    mails: Vec<String>,
    funcs: Vec<Value>,
}

impl RoomTable {
    pub fn links(&self) -> &[String] {
        &self.links
    }

    pub fn time(&self) -> &[String] {
        &self.time
    }

    pub fn phones(&self) -> &[String] {
        &self.phones
    }

    pub fn mails(&self) -> &[String] {
        &self.mails
    }

    pub fn funcs(&self) -> &[Value] {
        &self.funcs
    }
}
