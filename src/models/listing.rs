use num_bigint::BigInt;
use super::signature::Signature;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Listing {
    pub owner: String,
    pub chain_id: BigInt,
    pub min_price_cents: u16,
    pub nft_contract: String,
    pub token_id: BigInt,
    pub nonce: BigInt,
    pub signature: Signature
}
