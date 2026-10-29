use crate::types::ApiError;

/// Ошибки, возникающие при взаимодействии с API МТУСИ.
///
/// Объединяет ошибки HTTP-клиента, ошибки самого API и ситуацию,
/// когда API сообщил об успехе, но не вернул ожидаемые данные.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Ошибка выполнения HTTP-запроса, получения ответа или его разбора.
    ///
    /// Содержит исходную ошибку `reqwest`.
    #[error(transparent)]
    Http(#[from] reqwest::Error),

    /// API вернул статус ошибки и список описаний ошибок.
    #[error("Bitrix API errors: {0:?}")]
    Api(Vec<ApiError>),

    /// API сообщил об успешном выполнении запроса, но поле `data`
    /// отсутствует или не содержит ожидаемых данных.
    #[error("API returned success without data")]
    MissingData,
}
