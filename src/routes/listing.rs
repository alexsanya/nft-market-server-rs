use axum::http::StatusCode;
use axum::response::IntoResponse;
use tracing::debug;
use crate::dtos::listing::ParsingError;
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

async fn create(Json(payload): Json<ListingDTO>) -> Result<impl IntoResponse> {
    let result = payload.try_into();

    if let Ok(listing) = result {
        create_listing(&listing).await?;
        debug!("Success");
        debug!("{:?}", listing);
        Ok(StatusCode::CREATED)
    } else {
        let err = result.unwrap_err();
        debug!("Error {:?}", err);
        match &err {
            ParsingError::Signature(signature_err) => Err(
                Error::InvalidInput(
                    Entity::Listing,
                    format!("{}.{}", err.as_ref(), signature_err.as_ref())
                )
            ),
            _ => Err(
                Error::InvalidInput(
                    Entity::Listing,
                    err.as_ref().to_owned()
                )
            )
        }
    }
}

async fn get_all()-> Result<Json<Vec<Listing>>> {
    let listings = get_listings().await?;
    Ok(Json(listings))
}