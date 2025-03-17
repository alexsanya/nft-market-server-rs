use tokio::join;
use crate::{models::listing::Listing, repositories::listing::{get_all, save_listing}, services::onchain_provider::{ensure_owner_has_nft, esnure_erc712_allowance}};
use crate::prelude::*;

pub async fn create_listing(listing: &Listing) -> Result<()> {
    listing.verify_signature()?;
    let (owner_posess_nft, owner_has_allowance) = join!(
        ensure_owner_has_nft(&listing.owner, &listing.nft_contract, &listing.token_id),
        esnure_erc712_allowance(&listing.owner, &listing.nft_contract)
    );
    owner_posess_nft?;
    owner_has_allowance?;
    save_listing(listing).await?;
    Ok(())
}

pub async fn get_listings() -> Result<Vec<Listing>> {
    get_all().await
}