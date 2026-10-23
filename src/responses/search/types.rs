use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct SearchItem {
    #[serde(rename = "UF_NAME")]
    pub name: String,

    #[serde(rename = "type")]
    pub kind: SearchItemKind,

    pub value: String,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SearchItemKind {
    Teacher,
    Group,
    Audience,
}
