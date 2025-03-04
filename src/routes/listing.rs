
use tracing::debug;
use crate::Result;
use axum::{Router, routing::{get, post}, Json};

use crate::{controllers::listing::{create_listing, get_listings}, dtos::listing::{ListingDTO}, models::listing::Listing};

pub fn create_route() -> Router {
    Router::new()
        .route("/listings", post(create))
        .route("/listings", get(get_all))
}

async fn create(Json(payload): Json<ListingDTO>) {
    let result = payload.try_into();

    if let Ok(listing) = result {
        create_listing(&listing).await.unwrap();
        debug!("Success");
        debug!("{:?}", listing);
    } else {
        debug!("Error");
        debug!("{:?}", result.unwrap_err());
    }
}

async fn get_all()-> Result<Json<Vec<Listing>>> {
    let listings = get_listings()?;
    Ok(Json(listings))
}