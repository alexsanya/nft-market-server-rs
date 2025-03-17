use tracing::debug;
use axum::{routing::{get, post}, Json, Router};
use crate::{controllers::bid::{create_bid, get_bids}, dtos::bid::{BidDTO, ParsingError}, error::{Entity, Error}, models::bid::Bid, prelude::Result};

pub fn create_route() -> Router {
    Router::new()
        .route("/bids", post(create))
        .route("/bids", get(get_all))
}

async fn create(Json(payload): Json<BidDTO>) -> Result<()> {
    let result = payload.try_into();

    if let Ok(bid) = result {
        create_bid(&bid).await?;
        debug!("Success");
        debug!("{:?}", bid);
        Ok(())
    } else {
        let err = result.unwrap_err();
        debug!("Error {:?}", err);
        if let ParsingError::Listing(listing_err) = err.clone() {
            Err(Error::InvalidInput(Entity::Listing, format!("{}.{}", err.as_ref(), listing_err.as_ref())))
        } else {
            Err(Error::InvalidInput(Entity::Bid, err.as_ref().to_owned()))
        }
    }
}

async fn get_all()-> Result<Json<Vec<Bid>>> {
    let bids = get_bids().await?;
    Ok(Json(bids))
}