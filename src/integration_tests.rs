use axum::{body::{self, Body}, http::{header::CONTENT_TYPE, Method, Request, StatusCode}};
use dotenv::dotenv;
use serde::de::DeserializeOwned;
use tower::Service;
use crate::{datasource::init_redis, dtos::listing::ListingDTO, models::samples::test::{get_listing_dto, get_raw_listing}, routes::listing::create_route};


fn create_listing_request() -> Request<Body> {
    let body_str = get_raw_listing().to_string();
    let body = body::Body::from(body_str);
    Request::builder()
        .method(Method::POST)
        .uri("/listings")
        .header(CONTENT_TYPE, "application/json")
        .body(body)
        .expect("Failed to construct create listings request")
}

fn get_listings_request() -> Request<Body> {
    Request::builder()
        .method(Method::GET)
        .uri("/listings")
        .body(Body::empty())
        .expect("Failed to construct get listings request")
}

async fn deserialize_body<T: DeserializeOwned>(response_body: body::Body) -> T {
    let bytes = body::to_bytes(response_body, usize::MAX)
        .await
        .expect("Could not read data from response body!");

    serde_json::from_slice(&bytes).unwrap_or_else(|err| {
        panic!(
            "Could not parse body content into data structure! Error: {}, Received body: {:?}",
            err, bytes
        )
    })
}

#[tokio::test]
#[cfg_attr(not(feature = "integration_test"), ignore)]
async fn create_listing_success() {
    dotenv().ok();
    // logger::setup();
    init_redis().await;
    let mut router = create_route();
    let response = router.call(create_listing_request()).await.unwrap();
    let (res_parts, _) = response.into_parts();
    assert_eq!(StatusCode::CREATED, res_parts.status);
    let response = router.call(get_listings_request()).await.unwrap();
    let (res_parts, res_body) = response.into_parts();
    assert_eq!(StatusCode::OK, res_parts.status);
    let listings_list: Vec<ListingDTO> = deserialize_body(res_body).await;
    assert_eq!(listings_list[0], get_listing_dto());
}
