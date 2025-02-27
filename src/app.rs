use axum::Router;

pub async fn create_app() -> Router {
    Router::new()
        .merge(crate::routes::status::create_route())
        .merge(crate::routes::listing::create_route())
}