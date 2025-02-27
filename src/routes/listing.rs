
use tracing::debug;
use axum::{Router, routing::post, Json};

use crate::{dtos::listing::{ListingDTO, ParsingError}, models::listing::Listing};

pub fn create_route() -> Router {
    Router::new().route("/listings", post(create))
}

async fn create(Json(payload): Json<ListingDTO>) {
    debug!("{:?}", payload);


    let result: Result<Listing, ParsingError> = payload.try_into();

    if let Ok(listing) = result {
        debug!("Success");
        debug!("{:?}", listing);
    } else {
        debug!("Error");
        debug!("{:?}", result.unwrap_err());
    }
}