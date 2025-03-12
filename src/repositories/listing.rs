use serde_json;
use std::sync::Mutex;
use crate::models::listing::Listing;
use crate::prelude::*;

static STORAGE: Mutex<Vec<String>> = Mutex::new(Vec::new());

pub async fn save_listing(listing: &Listing) -> Result<()> {
    let mut storage = STORAGE.lock().map_err(|_| Error::SaveData)?;
    storage.push(serde_json::to_string(listing).unwrap());
    Ok(())
}

pub fn get_all() -> Result<Vec<Listing>> {
    let storage = STORAGE.lock().map_err(|_| Error::SaveData)?;
    Ok(storage.iter().map(|v| serde_json::from_str(v).unwrap()).collect())
}