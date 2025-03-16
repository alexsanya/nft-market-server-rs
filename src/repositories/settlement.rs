use serde_json;
use std::sync::Mutex;
use crate::datasource::set_value;
use crate::models::settlement::Settlement;
use crate::prelude::*;

static STORAGE: Mutex<Vec<String>> = Mutex::new(Vec::new());

pub async fn save_settlement(settlement: &Settlement) -> Result<()> {
    let key = format!("settlements:{}", &settlement.bid.get_hash()?);
    let value = serde_json::to_string(settlement).map_err(|_| Error::SaveData)?;
    set_value(&key, &value).await?;
    Ok(())
}

pub fn get_all() -> Result<Vec<Settlement>> {
    let storage = STORAGE.lock().map_err(|_| Error::FetchData)?;
    Ok(storage.iter().map(|v| serde_json::from_str(v).unwrap()).collect())
}