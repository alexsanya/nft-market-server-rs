use crate::{models::bid::Bid, prelude::Result};

pub async fn create_bid(bid: &Bid) -> Result<()> {
    bid.verify_signature()?;
    Ok(())
}