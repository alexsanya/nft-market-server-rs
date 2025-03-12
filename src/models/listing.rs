use ethers::types::U256;
use num_bigint::BigInt;
use ethers::abi::Address;
use std::str::FromStr;
use serde::{Serialize, Deserialize};
use super::listing_eip712::Listing as ListingEIP712;
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

#[derive(Debug)]
pub enum Eip712Error {
    MinPrice,
    NftContract,
    TokenId,
    Nonce
}

impl TryInto<ListingEIP712> for Listing {
    type Error = Eip712Error;

    fn try_into(self) -> Result<ListingEIP712, Self::Error> {
        let listing_eip712 = ListingEIP712 {
            min_price_cents: U256::from_dec_str(&self.min_price_cents.to_string()).map_err(|_| Eip712Error::MinPrice)?,
            nft_contract: Address::from_str(&self.nft_contract).map_err(|_| Eip712Error::NftContract)?,
            token_id: U256::from_dec_str(&self.token_id.to_string()).map_err(|_| Eip712Error::TokenId)?,
            nonce: U256::from_dec_str(&self.nonce.to_string()).map_err(|_| Eip712Error::Nonce)?,
        };
        Ok(listing_eip712)
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


        let listing_dto: ListingDTO = serde_json::from_value(raw_listing).unwrap();

        let listing: Listing = listing_dto.try_into().unwrap();




        let r = U256::from_str(&listing.signature.r).unwrap();
        let s = U256::from_str(&listing.signature.s).unwrap();
        let signature = Signature{ r, s, v: listing.signature.v };
        println!("Signature: {}", signature);


        let listing_eip712: ListingEIP712 = listing.try_into().expect("Failed to convert listing into EIP712");
        println!("listing_eip721: {:?}", listing_eip712);
        //listing_eip712.encode_eip712().expect("Error encoding into EIP712");

        //let address = signature.recover(hash).unwrap();
        let address = signature.recover_typed_data(&listing_eip712).expect("Cannot recover typed data");
        println!("Recovered address: {:?}", address);

        assert_eq!(address, Address::from_str("0x3897326ceda92b3da2c27a224d6fdcfefcacf57a").unwrap());

    }
}