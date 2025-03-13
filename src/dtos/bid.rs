use serde::{Deserialize, Serialize};
use crate::models::bid::Bid;
use crate::utils::patterns::Patterns;
use super::listing::ListingDTO;
use super::signature::SignatureDTO;
use super::listing::ParsingError as ParsingListingError;
use super::signature::ParsingError as ParsingSignatureError;

#[derive(Debug, strum_macros::AsRefStr)]
pub enum ParsingError {
    Bidder,
    TokenAddress,
    ValidUntil,
    Value,
    Listing(ParsingListingError),
    Signature(ParsingSignatureError)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BidDTO {
    pub bidder: String,
    pub listing: ListingDTO,
    pub token_address: String,
    pub valid_until: String,
    pub value: String,
    pub signature: SignatureDTO
}

impl TryInto<Bid> for BidDTO {
    type Error = ParsingError;

    fn try_into(self) -> Result<Bid, Self::Error> {
        let patterns = Patterns::new();
        let bid = Bid {
            bidder: patterns.test_address(&self.bidder).map_err(|_| ParsingError::Bidder)?.to_owned(),
            listing: self.listing.try_into().map_err(ParsingError::Listing)?,
            token_address: patterns.test_address(&self.token_address).map_err(|_| ParsingError::TokenAddress)?.to_owned(),
            valid_until: self.valid_until.parse().map_err(|_| ParsingError::ValidUntil)?,
            value: self.value.parse().map_err(|_| ParsingError::Value)?,
            siganture: self.signature.try_into().map_err(ParsingError::Signature)?
        };
        Ok(bid)
    }
}

#[cfg(test)]
mod test {
    use serde_json::json;
    use insta::assert_debug_snapshot;

    use super::*;

    #[test]
    fn convert() {
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
        assert_debug_snapshot!(bid);
    }
}