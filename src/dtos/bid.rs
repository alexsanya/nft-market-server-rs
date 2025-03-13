use serde::{Deserialize, Serialize};
use crate::models::bid::Bid;
use crate::utils::patterns::Patterns;
use super::listing::ListingDTO;
use super::signature::SignatureDTO;
use super::listing::ParsingError as ParsingListingError;
use super::signature::ParsingError as ParsingSignatureError;

#[derive(Debug, Clone, strum_macros::AsRefStr)]
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
            signature: self.signature.try_into().map_err(ParsingError::Signature)?
        };
        Ok(bid)
    }
}

#[cfg(test)]
mod test {
    use insta::assert_debug_snapshot;

    use crate::models::samples::test::get_raw_bid;

    use super::*;

    #[test]
    fn convert() {
        let raw_bid = get_raw_bid();
        let bid_dto: BidDTO = serde_json::from_value(raw_bid).expect("Cannot convert jsonBid to BidDTO");
        let bid: Bid = bid_dto.try_into().expect("Cannot convert BidBTo to bid");
        assert_debug_snapshot!(bid);
    }
}