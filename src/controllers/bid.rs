use tokio::join;
use crate::{models::bid::Bid, prelude::Result, repositories::bids::{get_all, save_bid}, services::onchain_provider::{ensure_bidder_has_tokens, ensure_erc20_allowance, ensure_owner_has_nft, esnure_erc712_allowance}};

pub async fn create_bid(bid: &Bid) -> Result<()> {
    bid.verify_signature()?;
    let (nft_ownership_check, nft_allowance_check, tokens_check, allowance_check)  = join!(
        ensure_owner_has_nft(&bid.listing.owner, &bid.listing.nft_contract, &bid.listing.token_id),
        esnure_erc712_allowance(&bid.listing.owner, &bid.listing.nft_contract),
        ensure_bidder_has_tokens(&bid.bidder, &bid.token_address, &bid.value),
        ensure_erc20_allowance(&bid.bidder, &bid.token_address, &bid.value)
    );
    nft_ownership_check?;
    nft_allowance_check?;
    tokens_check?;
    allowance_check?;
    save_bid(bid).await?;
    Ok(())
}

pub fn get_bids() -> Result<Vec<Bid>> {
    get_all()
}
