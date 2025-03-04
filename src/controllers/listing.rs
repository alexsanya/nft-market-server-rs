use crate::{models::listing::Listing, repositories::listing::{get_all, save_listing}};
use crate::prelude::*;

pub async fn create_listing(listing: &Listing) -> Result<()> {
    save_listing(listing).await?;
    Ok(())
}

pub fn get_listings() -> Result<Vec<Listing>> {
    get_all()
}