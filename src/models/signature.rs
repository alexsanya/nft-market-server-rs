
use ethers::contract::Eip712;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Signature {
    pub v: u64,
    pub r: String,
    pub s: String
}