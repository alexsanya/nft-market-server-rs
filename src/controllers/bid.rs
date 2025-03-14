use crate::{models::bid::Bid, prelude::Result, repositories::bids::{get_all, save_bid}, services::onchain_provider::{ensure_bidder_has_tokens, ensure_owner_has_nft}};

pub async fn create_bid(bid: &Bid) -> Result<()> {
    bid.verify_signature()?;
    //#TODO run in parralel
    check_owner_has_nft(&bid.listing.owner, &bid.listing.nft_contract, &bid.listing.token_id).await?;
    check_bidder_has_tokens(&bid.bidder, &bid.token_address, &bid.value).await?;
    save_bid(bid).await?;
    //#TODO check if tokens approved
    Ok(())
}

pub fn get_bids() -> Result<Vec<Bid>> {
    get_all()
}
