use crate::types::{SearchItemKind, SearchValue};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct SearchItem {
    #[serde(rename = "UF_NAME")]
    name: String,

    #[serde(rename = "type")]
    kind: SearchItemKind,

    value: SearchValue,
}

impl SearchItem {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn kind(&self) -> SearchItemKind {
        self.kind
    }

    pub fn value(&self) -> &SearchValue {
        &self.value
    }
}
