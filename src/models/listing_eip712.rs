use ethers::{abi::Address, contract::{Eip712, EthAbiType}, types::U256};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, EthAbiType, Eip712)]
#[eip712(
    name = "NFT Marketplace",
    version = "1",
    chain_id = 11155111, //Sepolia
    verifying_contract = "0x42d2C93839ED64b73Baa59A8ceB1C464287C8113"
)]
pub struct Listing {
    #[serde(rename = "nftContract")]
    pub nft_contract: Address,
    #[serde(rename = "tokenId")]
    pub token_id: U256,
    #[serde(rename = "minPriceCents")]
    pub min_price_cents: U256,
    #[serde(rename = "nonce")]
    pub nonce: U256
}
