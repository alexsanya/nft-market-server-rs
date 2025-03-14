use crate::{models::listing::Listing, repositories::listing::{get_all, save_listing}, services::onchain_provider::ensure_owner_has_nft};
use crate::prelude::*;

pub async fn create_listing(listing: &Listing) -> Result<()> {
    listing.verify_signature()?;
    ensure_owner_has_nft(listing.owner.clone(), listing.nft_contract.clone(), listing.token_id.clone()).await?;
    save_listing(listing).await?;
    Ok(())
}

pub fn get_listings() -> Result<Vec<Listing>> {
    get_all()
}