use num_bigint::BigInt;
use super::signature::Signature;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Listing {
    pub owner: String,
    pub chain_id: BigInt,
    pub min_price_cents: u16,
    pub nft_contract: String,
    //#[serde(with = "num_bigint::serde::bigint")]
    pub token_id: BigInt,
    //#[serde(with = "num_bigint::serde::bigint")]
    pub nonce: BigInt,
    //#[serde(with = "num_bigint::serde::bigint")]
    pub signature: Signature
}
