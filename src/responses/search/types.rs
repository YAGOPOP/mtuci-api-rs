use serde::Deserialize;
use crate::types::{SearchItemKind, SearchValue};

#[derive(Debug, Deserialize)]
pub struct SearchItem {
    #[serde(rename = "UF_NAME")]
    pub name: String,

    #[serde(rename = "type")]
    pub kind: SearchItemKind,

    pub value: SearchValue,
}
