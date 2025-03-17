use serde_json;
use tracing::debug;
use crate::datasource::{set_value, get_all as fetch};
use crate::dtos::bid::BidDTO;
use crate::error::Error;
use crate::models::bid::Bid;
use crate::prelude::Result as MyResult;

pub async fn save_bid(bid: &Bid) -> MyResult<()> {
    let key = format!("bids:{}", &bid.get_hash()?);
    let value = serde_json::to_string(bid).map_err(|_| Error::SaveData)?;
    debug!("Serrialized bid: {}", value);
    set_value(&key, &value).await?;
    Ok(())
}

pub async fn get_all() -> MyResult<Vec<Bid>> {
    let values = fetch("bids:*").await?;
    let bids = values.iter().filter_map(|value| {
        let bid_dto: Result<BidDTO, _> = serde_json::from_str(value);
        if let Ok(bid_dto) = bid_dto {
            match bid_dto.try_into() {
                Ok(bid) => Some(bid),
                _ => None
            }
        } else {
            None
        }
    }).collect();

    Ok(bids)
}