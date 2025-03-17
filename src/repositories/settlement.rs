use serde_json;
use tracing::debug;
use crate::datasource::{set_value, get_all as fetch};
use crate::dtos::settlement::SettlementDTO;
use crate::error::Error;
use crate::models::settlement::Settlement;
use crate::prelude::Result as MyResult;

pub async fn save_settlement(settlement: &Settlement) -> MyResult<()> {
    let key = format!("settlements:{}", &settlement.bid.get_hash()?);
    let value = serde_json::to_string(settlement).map_err(|_| Error::SaveData)?;
    debug!("Serrialized settlement: {}", value);
    set_value(&key, &value).await?;
    Ok(())
}

pub async fn get_all() -> MyResult<Vec<Settlement>> {
    let values = fetch("settlements:*").await?;
    let settlements: Vec<Settlement> = values.iter().filter_map(|value| {
        let settlement_dto: Result<SettlementDTO, _> = serde_json::from_str(value);
        debug!("SettlementDTO: {:?}", settlement_dto);
        if let Ok(settlement_dto) = settlement_dto {
            match settlement_dto.try_into() {
                Ok(settlement) => Some(settlement),
                _ => None
            }
        } else {
            None
        }
    }).collect();
    Ok(settlements)
}