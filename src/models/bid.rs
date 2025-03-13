use ethers::types::{Address, U256};
use num_bigint::BigInt;
use std::str::FromStr;
use serde::{Deserialize, Serialize};
use super::signature::Signature as SigString;
use super::bid_eip712::Bid as BidEIP712;
use super::listing_eip712::Listing as ListingEIP712;
use ethers::types::transaction::eip712::Eip712;
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


#[cfg(test)]
mod tests {
    use std::str::FromStr;
    use ethers::types::Signature;
    use ethers::types::Address;
    use serde_json::json;
    use crate::dtos::bid::BidDTO;
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

        let raw_bid = json!({
            "bidder": "0xE98D94496aB9084f597a69978b593EBf83147335",
            "listing": raw_listing,
            "token_address": "0xc29f6F8D639eF187DcFEfeFBaD989cF2C941a23A",
            "valid_until": "1735504160",
            "value": "250",
            "signature": {
                "v": 28,
                "r": "0x1469ac6f9636c24d2d8c3fb2cbef73708876e15f23f23b1d33863939c905a21c",
                "s": "0x7d9a7ea039465c928311bcb737b23153232028038beadba2a667aa720f17602b"
            }
        });
        let bid_dto: BidDTO = serde_json::from_value(raw_bid).expect("Cannot convert jsonBid to BidDTO");
        let bid: Bid = bid_dto.try_into().expect("Cannot convert BidBTo to bid");
        let signature: Signature = bid.signature.clone().try_into().expect("Cannot convert signature");
        let bidder = bid.bidder.clone();
        let bid_eip712: BidEIP712 = bid.try_into().expect("Cannot convert Bid into BidEIP712");
        let address = signature.recover_typed_data(&bid_eip712).expect("Cannot recover typed data");
        println!("Recovered address: {:?}", address);
        assert_eq!(address, Address::from_str(&bidder).unwrap());

    }
}