use crate::{models::listing::Listing, repositories::listing::{get_all, save_listing}, services::onchain_provider::check_owner_has_nft};
use crate::prelude::*;

pub async fn create_listing(listing: &Listing) -> Result<()> {
    check_owner_has_nft(&listing.owner, &listing.nft_contract, &listing.token_id).await?;
    save_listing(listing).await?;
    Ok(())
}

pub fn get_listings() -> Result<Vec<Listing>> {
    get_all()
}