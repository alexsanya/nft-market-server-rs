use ethers::types::{Address, Signature, U256};
use num_bigint::BigInt;
use std::str::FromStr;
use serde::{Deserialize, Serialize};
use super::signature::Signature as SigString;
use super::bid_eip712::Bid as BidEIP712;
use super::listing_eip712::Listing as ListingEIP712;
use ethers::types::transaction::eip712::Eip712;
use crate::error::{Entity, Error};
use crate::prelude::Result as MyResult;
use super::listing::Listing;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bid {
    pub bidder: String,
    pub listing: Listing,
    pub token_address: String,
    pub valid_until: BigInt,
    pub value: BigInt,
    pub signature: SigString
}

#[derive(Debug)]
pub enum Eip712Error {
    TokenAddress,
    ValidUntil,
    Value,
    Listing
}

impl TryInto<BidEIP712> for Bid {
    type Error = Eip712Error;
    fn try_into(self) -> Result<BidEIP712, Self::Error> {
        let listing_eip712: ListingEIP712 = self.listing.try_into().map_err(|_| Eip712Error::Listing)?;
        let hash = listing_eip712.encode_eip712().map_err(|_| Eip712Error::Listing)?;
        let bid_eip712 = BidEIP712 {
            token_contract: Address::from_str(&self.token_address).map_err(|_| Eip712Error::TokenAddress)?,
            value: U256::from_dec_str(&self.value.to_string()).map_err(|_| Eip712Error::Value)?,
            valid_until: U256::from_dec_str(&self.valid_until.to_string()).map_err(|_| Eip712Error::ValidUntil)?,
            listing_hash: hash.into()
        };
        Ok(bid_eip712)
    }
}

impl Bid {
    pub fn verify_signature(&self) -> MyResult<()> {
        self.listing.verify_signature()?;
        let signature: Signature = self.signature.clone().try_into().map_err(|_| Error::InvalidSignature(Entity::Bid))?;
        let bid_eip712: BidEIP712 = self.clone().try_into().map_err(|_| Error::InvalidSignature(Entity::Bid))?;
        let recover_result = signature.recover_typed_data(&bid_eip712);
        let bidder_result = Address::from_str(&self.bidder);
        match (recover_result, bidder_result) {
            (Ok(recovered), Ok(bidder)) if recovered == bidder => Ok(()),
            _ => Err(Error::InvalidSignature(Entity::Bid))
        }
    }
}


#[cfg(test)]
mod tests {
    use std::str::FromStr;
    use ethers::types::Signature;
    use ethers::types::Address;
    use crate::models::samples::test::get_bid;
    use super::*;

    #[test]
    fn check_hash() {
        let bid = get_bid();
        let signature: Signature = bid.signature.clone().try_into().expect("Cannot convert signature");
        let bidder = bid.bidder.clone();
        let bid_eip712: BidEIP712 = bid.try_into().expect("Cannot convert Bid into BidEIP712");
        let address = signature.recover_typed_data(&bid_eip712).expect("Cannot recover typed data");
        println!("Recovered address: {:?}", address);
        assert_eq!(address, Address::from_str(&bidder).unwrap());

    }

    #[test]
    fn check_signature_ok() {
        let bid = get_bid();
        bid.verify_signature().expect("Error");
    }

    #[test]
    fn check_listing_sig_incorrect() {
        let bid = get_bid();
        let bid_incorrect_sig = Bid {
            listing: Listing {
                signature: SigString {
                    v: 29,
                    ..bid.signature.clone()
                },
                ..bid.listing
            },
            ..bid
        };
        assert!(matches!(bid_incorrect_sig.verify_signature(), Err(Error::InvalidSignature(Entity::Listing))));
    }

    #[test]
    fn check_bid_sig_incorrect() {
        let bid = get_bid();
        let bid_incorrect_sig = Bid {
            signature: SigString {
                v: 29,
                ..bid.signature
            },
            ..bid
        };
        assert!(matches!(bid_incorrect_sig.verify_signature(), Err(Error::InvalidSignature(Entity::Bid))));
    }
}