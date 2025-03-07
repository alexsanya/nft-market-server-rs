use tracing::debug;
use crate::error::Entity;
use crate::Result;
use crate::Error;
use axum::{routing::{get, post}, Json, Router};

use crate::{controllers::listing::{create_listing, get_listings}, dtos::listing::ListingDTO, models::listing::Listing};

pub fn create_route() -> Router {
    Router::new()
        .route("/listings", post(create))
        .route("/listings", get(get_all))
}

async fn create(Json(payload): Json<ListingDTO>) -> Result<()> {
    let result = payload.try_into();

    if let Ok(listing) = result {
        create_listing(&listing).await?;
        debug!("Success");
        debug!("{:?}", listing);
        Ok(())
    } else {
        debug!("Error");
        let err = result.unwrap_err();
        Err(Error::InvalidInput(Entity::Listing, err.as_ref().to_owned()))
    }
}

async fn get_all()-> Result<Json<Vec<Listing>>> {
    let listings = get_listings()?;
    Ok(Json(listings))
}