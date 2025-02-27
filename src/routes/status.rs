use serde::Serialize;
use axum::{Router, routing::get, Json};

pub fn create_route() -> Router {
    Router::new().route("/status", get(handler))
}

async fn handler() -> Result<Json<Status>, ()> {
    Ok(Json(Status { status: "OK".to_owned() }))
}

#[derive(Serialize, Debug)]
struct Status {
    status: String
}