use num_bigint::BigInt;
use crate::utils::hash::Hashable;
use ethers::utils::keccak256;
use ethers::abi::AbiEncode;
use tracing::debug;
use hex;

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

impl Hashable for Listing {
    fn hash(&self, domain_separator: String) -> String {
        let encoded_data = (
            keccak256(b"Listing(address nftContract,uint256 tokenId,uint256 minPriceCents,uint256 nonce)"),
            self.nft_contract.parse::<ethers::types::Address>().unwrap(),
            self.token_id.to_string().parse::<ethers::types::U256>().unwrap(),
            hex::encode(self.min_price_cents.to_be_bytes()).parse::<ethers::types::U256>().unwrap(),
            self.nonce.to_string().parse::<ethers::types::U256>().unwrap()
        ).encode();

        print!("hello hash: {}", hex::encode(keccak256(b"hello")));
        print!("encoded_data: {}", hex::encode(&encoded_data));
        print!("encoded_data hash: {:?}", hex::encode(keccak256(&encoded_data[..])));

        //let hex_raw = (
        //    ethers::types::Bytes::from(vec![0x19, 0x01]),
        //    domain_separator.parse::<ethers::types::Bytes>().unwrap(),
        //    ethers::types::Bytes::from(keccak256(encoded_data))
        //).encode();

        let prefix = vec![0x19, 0x01];
        let result = [&prefix[..], &hex::decode(domain_separator).unwrap()[..], &keccak256(&encoded_data)[..]].concat();

        print!("hex_raw: {}", hex::encode(&result));
        hex::encode(keccak256(&result))
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use crate::dtos::listing::ListingDTO;

    use super::*;

    #[test]
    fn check_hash() {
        let raw_listing = json!({
            "owner": "0x3897326cEda92B3da2c27a224D6fDCFefCaCf57A",
            "chain_id": "11155111",
            "min_price_cents": 137,
            "nft_contract": "0xf44b599a0aB6b8cb14E992994BEC0dc59dF883B2",
            "token_id": "1",
            "nonce": "0",
            "signature": {
                "v": 28,
                "r": "0x5ef4620f4b296763ff15209456d75e868f149a8d1c6821f1ff11fab70bca0ee0",
                "s": "0x337ddcb26ea919a2bf5ad6e1d49bd6951a27d1d2e940a5543a70eabc5dbe237e"
            }
        });

        let DOMAIN_SEPARATOR = "47720df067349d6e15f380966605c432d4f59c1ad2d55501d9d8ea139c7244d9".to_owned();

        let listing_dto: ListingDTO = serde_json::from_value(raw_listing).unwrap();

        let listing: Listing = listing_dto.try_into().unwrap();

        let hash = listing.hash(DOMAIN_SEPARATOR);

        assert_eq!(hash, "07e440212245f786b1ab066a629db7ef7b9e6f98b1ced217ed260b3d1e1a1fa3");

    }
}