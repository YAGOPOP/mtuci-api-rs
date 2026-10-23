use crate::types::ApiError;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Http(#[from] reqwest::Error),

    #[error("Bitrix API errors: {0:?}")]
    Api(Vec<ApiError>),

    #[error("API returned success without data")]
    MissingData,

    #[error("invalid month: {0}, expected 1..=12")]
    InvalidMonth(u8),

    // #[error("search query cannot be empty")]
    // EmptySearchQuery,
}
