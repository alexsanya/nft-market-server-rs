use serde_json;
use tracing::debug;
use crate::datasource::{set_value, get_all as fetch};
use crate::dtos::listing::ListingDTO;
use crate::models::listing::Listing;
use crate::error::Error;
use crate::prelude::Result as MyResult;

pub async fn save_listing(listing: &Listing) -> MyResult<()> {
    let key = format!("listings:{}", &listing.get_hash()?);
    let value = serde_json::to_string(listing).map_err(|_| Error::SaveData)?;
    debug!("Serrialized listing: {}", value);
    set_value(&key, &value).await?;
    Ok(())
}

pub async fn get_all() -> MyResult<Vec<Listing>> {
    let values = fetch("listings:*").await?;
    let listings = values.iter().filter_map(|value| {
        let listing_dto: Result<ListingDTO, _> = serde_json::from_str(&value);
        if let Ok(listing_dto) = listing_dto {
            match listing_dto.try_into() {
                Ok(listing) => Some(listing),
                _ => None
            }
        } else {
            None
        }
    }).collect();
    Ok(listings)
}