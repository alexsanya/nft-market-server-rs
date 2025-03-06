use axum::{middleware, response::{IntoResponse, Response}, Json, Router};
use serde_json::json;
use crate::Error;

pub async fn create_app() -> Router {
    Router::new()
        .merge(crate::routes::status::create_route())
        .merge(crate::routes::listing::create_route())
        .layer(middleware::map_response(map_response_mapper))
}

async fn map_response_mapper(res: Response) -> Response {
    let error = res.extensions().get::<Error>();

    if let Some(error) = error {
        let (status_code, client_error, description) = error.client_status_and_errors();
        let response = match description {
            Some(description) => json!({
                "error": client_error.as_ref(),
                "description": description
            }),
            None => json!({
                "error": client_error.as_ref()
            })
        };
        (status_code, Json(response)).into_response()
    } else {
        res
    }
}