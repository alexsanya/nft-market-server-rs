use serde_json;
use std::sync::Mutex;
use once_cell::sync::Lazy;
use crate::models::listing::Listing;

static STORAGE: Lazy<Mutex<Vec<String>>> = Lazy::new(|| Mutex::new(Vec::new()));

pub async fn save_listing(listing: &Listing) -> Result<(), ()> {
    if let Ok(mut storage) = STORAGE.lock() {
        storage.push(serde_json::to_string(listing).unwrap());
    }
    Ok(())
}

pub fn get_all() -> Result<Vec<String>, ()> {
    let storage = STORAGE.lock().unwrap();
    Ok(storage.iter().map(|v| v.clone()).collect())
}