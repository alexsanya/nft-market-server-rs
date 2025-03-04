use axum::{http::StatusCode, response::{Response, IntoResponse}};

#[derive(thiserror::Error, Clone, Debug)]
pub enum Error {
    #[error("Generic {0}")]
    Generic(String),

    #[error("SaveData")]
    SaveDataError,
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let mut response = StatusCode::INTERNAL_SERVER_ERROR.into_response();
        response.extensions_mut().insert(self);
        response
    }
}

impl Error {
    pub fn client_status_and_errors(&self) -> (StatusCode, ClientError) {
        (StatusCode::INTERNAL_SERVER_ERROR, ClientError::SERVER_ERROR)
    }
}

#[derive(Debug, strum_macros::AsRefStr)]
#[allow(non_camel_case_types)]
pub enum ClientError {
    SERVER_ERROR
}