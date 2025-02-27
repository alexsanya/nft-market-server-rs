
use axum::{Router, routing::post, Json};

use crate::{dtos::listing::{ListingDTO, ParsingError}, models::listing::Listing};

pub fn create_route() -> Router {
    Router::new().route("/listings", post(create))
}

async fn create(Json(payload): Json<ListingDTO>) {
    print!("{:?}", payload);


    let result: Result<Listing, ParsingError> = payload.try_into();

    if let Ok(listing) = result {
        print!("{:?}", listing);
    } else {
        print!("{:?}", result.unwrap_err())
    }
}