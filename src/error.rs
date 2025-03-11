use axum::{http::StatusCode, response::{Response, IntoResponse}};

#[derive(Clone, Debug, strum_macros::AsRefStr)]
pub enum Entity {
    Listing
}

#[derive(thiserror::Error, Clone, Debug)]
pub enum Error {
    #[error("Generic {0}")]
    Generic(String),

    #[error("SaveData")]
    SaveDataError,

    #[error("InvalidInput")]
    InvalidInput(Entity, String),

    #[error("MissingNFT")]
    MissingNFT
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let mut response = StatusCode::INTERNAL_SERVER_ERROR.into_response();
        response.extensions_mut().insert(self);
        response
    }
}

impl Error {
    pub fn client_status_and_errors(&self) -> (StatusCode, ClientError, Option<String>) {
        match self {
            Self::InvalidInput(entity, field) => (StatusCode::BAD_REQUEST, ClientError::CLIENT_ERROR, Some(format!("{}.{}", entity.as_ref(), field))),
            Self::MissingNFT => (StatusCode::BAD_REQUEST, ClientError::CLIENT_ERROR, Some("Owner doesnt posess NFT".to_owned())),
            _ => (StatusCode::INTERNAL_SERVER_ERROR, ClientError::SERVER_ERROR, None)
        }
    }
}

#[derive(Debug, strum_macros::AsRefStr)]
#[allow(non_camel_case_types)]
pub enum ClientError {
    CLIENT_ERROR,
    SERVER_ERROR
}