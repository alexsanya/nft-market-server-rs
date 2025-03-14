use crate::{models::bid::Bid, prelude::Result, services::onchain_provider::{check_bidder_has_tokens, check_owner_has_nft}};

pub async fn create_bid(bid: &Bid) -> Result<()> {
    bid.verify_signature()?;
    //#TODO run in parralel
    check_owner_has_nft(&bid.listing.owner, &bid.listing.nft_contract, &bid.listing.token_id).await?;
    check_bidder_has_tokens(&bid.bidder, &bid.token_address, &bid.value).await?;
    //#TODO check if tokens approved
    Ok(())
}