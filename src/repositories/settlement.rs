use serde_json;
use std::sync::Mutex;
use crate::models::settlement::Settlement;
use crate::prelude::*;

static STORAGE: Mutex<Vec<String>> = Mutex::new(Vec::new());

pub async fn save_settlement(settlement: &Settlement) -> Result<()> {
    let mut storage = STORAGE.lock().map_err(|_| Error::SaveData)?;
    storage.push(serde_json::to_string(settlement).unwrap());
    Ok(())
}

pub fn get_all() -> Result<Vec<Settlement>> {
    let storage = STORAGE.lock().map_err(|_| Error::FetchData)?;
    Ok(storage.iter().map(|v| serde_json::from_str(v).unwrap()).collect())
}