use serde_json;
use std::sync::Mutex;
use crate::models::bid::Bid;
use crate::prelude::*;

static STORAGE: Mutex<Vec<String>> = Mutex::new(Vec::new());

pub async fn save_bid(bid: &Bid) -> Result<()> {
    let mut storage = STORAGE.lock().map_err(|_| Error::SaveData)?;
    storage.push(serde_json::to_string(bid).unwrap());
    Ok(())
}

pub fn get_all() -> Result<Vec<Bid>> {
    let storage = STORAGE.lock().map_err(|_| Error::FetchData)?;
    Ok(storage.iter().map(|v| serde_json::from_str(v).unwrap()).collect())
}