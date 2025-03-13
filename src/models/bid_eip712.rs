use ethers::{contract::{Eip712, EthAbiType}, types::{Address, H256, U256}};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, EthAbiType, Eip712)]
#[eip712(
    name = "NFT Marketplace",
    version = "1",
    chain_id = 11155111, //Sepolia
    verifying_contract = "0x42d2C93839ED64b73Baa59A8ceB1C464287C8113"
)]
pub struct Bid {
    #[serde(rename = "tokenContract")]
    pub token_contract: Address,
    #[serde(rename = "value")]
    pub value: U256,
    #[serde(rename = "validUntil")]
    pub valid_until: U256,
    #[serde(rename = "listingHash")]
    pub listing_hash: H256,
}