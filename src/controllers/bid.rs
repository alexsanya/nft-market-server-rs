use crate::{models::bid::Bid, prelude::Result, services::onchain_provider::{ensure_bidder_has_tokens, ensure_owner_has_nft}};

pub async fn create_bid(bid: &Bid) -> Result<()> {
    //bid.verify_signature()?;
    let owner = bid.listing.owner.clone();
    let nft_contract = bid.listing.nft_contract.clone();
    let token_id = bid.listing.token_id.clone();
    let t1 = tokio::spawn(ensure_owner_has_nft(owner, nft_contract, token_id));
    let t2 = tokio::spawn(ensure_bidder_has_tokens(bid.bidder.clone(), bid.token_address.clone(), bid.value.clone()));
    t1.await.unwrap()?;
    t2.await.unwrap()?;
    //#TODO check if tokens approved
    Ok(())
}