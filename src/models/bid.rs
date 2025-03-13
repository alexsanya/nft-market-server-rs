use num_bigint::BigInt;
use serde::{Deserialize, Serialize};
use super::signature::Signature as SigString;

use super::listing::Listing;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bid {
    pub bidder: String,
    pub listing: Listing,
    pub token_address: String,
    pub valid_until: BigInt,
    pub value: BigInt,
    pub siganture: SigString
}