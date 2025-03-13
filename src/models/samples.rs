#[cfg(test)]
pub mod test {
    use std::os::macos::raw;

    use serde_json::{json, Value};

    use crate::{dtos::{bid::BidDTO, listing::ListingDTO}, models::{bid::Bid, listing::Listing}};

    pub fn get_raw_listing() -> Value {
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
        raw_listing
    }

    pub fn get_listing() -> Listing {
        let raw_listing = get_raw_listing();
        let listing_dto: ListingDTO = serde_json::from_value(raw_listing).expect("Cannot convert json to ListingDTO");
        listing_dto.try_into().expect("Cannot convert ListingDTO to Listing")
    }

    pub fn get_raw_bid() -> Value {
        let raw_bid = json!({
            "bidder": "0xE98D94496aB9084f597a69978b593EBf83147335",
            "listing": get_raw_listing(),
            "token_address": "0xc29f6F8D639eF187DcFEfeFBaD989cF2C941a23A",
            "valid_until": "1735504160",
            "value": "250",
            "signature": {
                "v": 28,
                "r": "0x1469ac6f9636c24d2d8c3fb2cbef73708876e15f23f23b1d33863939c905a21c",
                "s": "0x7d9a7ea039465c928311bcb737b23153232028038beadba2a667aa720f17602b"
            }
        });
        raw_bid
    }

    pub fn get_bid() -> Bid {
        let raw_bid = get_raw_bid();
        let bid_dto: BidDTO = serde_json::from_value(raw_bid).expect("Cannot convert json to BidDTO");
        bid_dto.try_into().expect("Cannot convert BidBTo to bid")
    }
}