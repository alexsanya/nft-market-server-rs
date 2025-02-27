use crate::{models::listing::Listing, repositories::listing::save_listing};


pub fn create_listing(listing: &Listing) -> Result<(), ()> {
    save_listing(listing)?;
    Ok(())
}