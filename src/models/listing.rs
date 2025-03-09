use num_bigint::BigInt;
use crate::utils::hash::Hashable;
use ethers::utils::keccak256;
use ethers::abi::AbiEncode;
use hex;

use serde::{Serialize, Deserialize};
use super::signature::Signature as SigString;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Listing {
    pub owner: String,
    pub chain_id: BigInt,
    pub min_price_cents: BigInt,
    pub nft_contract: String,
    pub token_id: BigInt,
    pub nonce: BigInt,
    pub signature: SigString
}

impl Hashable for Listing {
    fn hash(&self, domain_separator: String) -> [u8; 32] {
        let encoded_data = (
            keccak256(b"Listing(address nftContract,uint256 tokenId,uint256 minPriceCents,uint256 nonce)"),
            self.nft_contract.parse::<ethers::types::Address>().unwrap(),
            self.token_id.to_string().parse::<ethers::types::U256>().unwrap(),
            hex::encode(self.min_price_cents.to_signed_bytes_be()).parse::<ethers::types::U256>().unwrap(),
            self.nonce.to_string().parse::<ethers::types::U256>().unwrap()
        ).encode();


        let prefix = [0x19, 0x01];
        let result = [
            &prefix[..],
            &hex::decode(domain_separator).unwrap()[..],
            &keccak256(&encoded_data)[..]
        ].concat();

        //print!("hex_raw: {}", hex::encode(&result));
        keccak256(&result)
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;
    use ethers::types::Signature;

    use ethers::types::{Address, U256};
    use serde_json::json;

    use crate::dtos::listing::ListingDTO;

    use super::*;

    #[test]
    fn check_hash() {
        let raw_listing = json!({
            "owner": "0x3897326cEda92B3da2c27a224D6fDCFefCaCf57A",
            "chain_id": "11155111",
            "min_price_cents": "150000",
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

        assert_eq!(hex::encode(hash), "a065e3992869424e8464212057e58769f949c06184d8f472d58b4827c0c8fe5d");

        let r = U256::from_str(&listing.signature.r).unwrap();
        let s = U256::from_str(&listing.signature.s).unwrap();
        let signature = Signature{ r, s, v: listing.signature.v };
        print!(" | Signature: {}", signature.to_string());
        print!(" | hash: {:?}", hex::encode(&hash));

        let address = signature.recover(hash).unwrap();
        print!(" | Recovered address: {:?}", address);

        assert_eq!(address, Address::from_str("0x1d86d9c913934c5e1908c882f9de0fe8433fee79").unwrap());

    }
}