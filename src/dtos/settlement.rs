use serde::{Deserialize, Serialize};
use crate::models::settlement::Settlement;
use crate::dtos::listing::ParsingError as ListingParsingError;
use crate::dtos::bid::ParsingError as BidParsingError;
use crate::dtos::signature::ParsingError as SigParsingError;

use super::{bid::BidDTO, listing::ListingDTO, signature::SignatureDTO};

#[derive(Debug, Serialize, Deserialize)]
pub struct SettlementDTO {
    pub listing: ListingDTO,
    pub bid: BidDTO,
    pub signature: SignatureDTO
}


#[derive(Debug, Clone, strum_macros::AsRefStr)]
pub enum ParsingError {
    Listing(ListingParsingError),
    Bid(BidParsingError),
    Signature(SigParsingError)
}

impl TryInto<Settlement> for SettlementDTO {
    type Error = ParsingError;

    fn try_into(self) -> Result<Settlement, Self::Error> {
        Ok(Settlement {
            listing: self.listing.try_into().map_err(ParsingError::Listing)?,
            bid: self.bid.try_into().map_err(ParsingError::Bid)?,
            signature: self.signature.try_into().map_err(ParsingError::Signature)?
        })
    }
}

#[cfg(test)]
mod test {
    use insta::assert_debug_snapshot;

    use crate::models::samples::test::get_raw_settlement;

    use super::*;

    #[test]
    fn convert() {
        let raw_settlement = get_raw_settlement();
        let settlement_dto: SettlementDTO = serde_json::from_value(raw_settlement).expect("Cannot convert json to SettlementDTO");
        let settlement: Settlement = settlement_dto.try_into().expect("Cannot convert SettlementDTO to settlement");
        assert_debug_snapshot!(settlement);
    }
}