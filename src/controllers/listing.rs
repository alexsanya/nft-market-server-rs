use axum::Json;

use crate::{models::listing::Listing, repositories::listing::{get_all, save_listing}};


pub async fn create_listing(listing: &Listing) -> Result<(), ()> {
    save_listing(listing).await?;
    Ok(())
}

pub fn get_listings() -> Vec<String> {
    get_all().unwrap()
}