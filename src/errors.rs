use crate::types::ApiError;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Http(#[from] reqwest::Error),

    #[error("Bitrix API errors: {0:?}")]
    Api(Vec<ApiError>),

    #[error("API returned success without data")]
    MissingData,
}
