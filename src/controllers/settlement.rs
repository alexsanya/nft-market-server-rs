use tokio::join;
use crate::{models::settlement::Settlement, prelude::Result, repositories::settlement::{get_all, save_settlement}, services::onchain_provider::{ensure_bidder_has_tokens, ensure_erc20_allowance, ensure_owner_has_nft, esnure_erc712_allowance}};

pub async fn create_settlement(settlement: &Settlement) -> Result<()> {
    settlement.verify_signature()?;
    let (nft_ownership_check, nft_allowance_check, tokens_check, allowance_check)  = join!(
        ensure_owner_has_nft(&settlement.bid.listing.owner, &settlement.bid.listing.nft_contract, &settlement.bid.listing.token_id),
        esnure_erc712_allowance(&settlement.bid.listing.owner, &settlement.bid.listing.nft_contract),
        ensure_bidder_has_tokens(&settlement.bid.bidder, &settlement.bid.token_address, &settlement.bid.value),
        ensure_erc20_allowance(&settlement.bid.bidder, &settlement.bid.token_address, &settlement.bid.value)
    );
    nft_ownership_check?;
    nft_allowance_check?;
    tokens_check?;
    allowance_check?;
    save_settlement(settlement).await?;
    Ok(())
}

pub fn get_settlements() -> Result<Vec<Settlement>> {
    get_all()
}
