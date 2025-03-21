use ethers::types::{Signature, U256};
use num_bigint::BigInt;
use ethers::abi::Address;
use std::str::FromStr;
use serde::{Serialize, Deserialize};
use crate::error::{Entity, Error};
use crate::prelude::Result as MyResult;
use ethers::types::transaction::eip712::Eip712;
use crate::utils::serialization::serialize_bigint_as_string;

use super::listing_eip712::Listing as ListingEIP712;
use super::signature::Signature as SigString;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Listing {
    pub owner: String,
    #[serde(serialize_with = "serialize_bigint_as_string")]
    pub chain_id: BigInt,
    #[serde(serialize_with = "serialize_bigint_as_string")]
    pub min_price_cents: BigInt,
    pub nft_contract: String,
    #[serde(serialize_with = "serialize_bigint_as_string")]
    pub token_id: BigInt,
    #[serde(serialize_with = "serialize_bigint_as_string")]
    pub nonce: BigInt,
    pub signature: SigString
}


impl Listing {
    pub fn verify_signature(&self) -> MyResult<()> {
        let signature: Signature = self.signature.clone().try_into().map_err(|_| Error::InvalidSignature(Entity::Listing))?;
        let owner_result = Address::from_str(&self.owner);
        let listing_eip721: ListingEIP712 = self.clone().try_into().map_err(|_| Error::InvalidSignature(Entity::Listing))?;
        match (signature.recover_typed_data(&listing_eip721), owner_result) {
            (Ok(recovered), Ok(owner)) if recovered == owner => Ok(()),
            _ => Err(Error::InvalidSignature(Entity::Listing))
        }
    }

    pub fn get_hash(&self) -> MyResult<String> {
        let listing_eip712: ListingEIP712 = self.clone().try_into().map_err(|_| Error::InvalidSignature(Entity::Listing))?;
        let hash = listing_eip712.encode_eip712().map_err(|_| Error::InvalidSignature(Entity::Listing))?;
        Ok(hex::encode(hash))
    }
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
    use ethers::types::Address;
    use ethers::types::transaction::eip712::Eip712;
    use crate::models::samples::test::get_listing;
    use super::*;

    #[test]
    fn check_hash() {
        let listing: Listing = get_listing();
        let signature: Signature = listing.signature.clone().try_into().expect("Cannot convert signature");
        println!("Signature: {}", signature);

        let owner = listing.owner.clone();
        let listing_eip712: ListingEIP712 = listing.try_into().expect("Failed to convert listing into EIP712");
        println!("listing_eip721: {:?}", listing_eip712);
        println!("EIP712 hash: {:?}", hex::encode(listing_eip712.encode_eip712().unwrap()));

        let address = signature.recover_typed_data(&listing_eip712).expect("Cannot recover typed data");
        println!("Recovered address: {:?}", address);

        assert_eq!(address, Address::from_str(&owner).unwrap());

    }

    #[test]
    fn check_signature() {
        let listing = get_listing();
        listing.verify_signature().expect("Error");
        let listing_incorrect_sig = Listing {
            signature: SigString {
                v: 29,
                ..listing.signature
            },
            ..listing
        };
        assert!(matches!(listing_incorrect_sig.verify_signature(), Err(Error::InvalidSignature(Entity::Listing))));
    }
}