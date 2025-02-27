use serde::{Serialize, Deserialize};

use crate::models::listing::Listing;

use super::signature::SignatureDTO;

#[derive(Debug)]
pub enum ParsingError {
    ChainId,
    TokenId,
    Nonce,
    Signature
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListingDTO {
    pub owner: String,
    pub chain_id: String,
    pub min_price_cents: u16,
    pub nft_contract: String,
    pub token_id: String,
    pub nonce: String,
    pub signature: SignatureDTO
}

impl TryInto<Listing> for ListingDTO {
    type Error = ParsingError;

    fn try_into(self) -> Result<Listing, Self::Error> {
        let listing = Listing{
            owner: self.owner,
            chain_id: self.chain_id.parse().map_err(|_| ParsingError::ChainId)?,
            min_price_cents: self.min_price_cents,
            nft_contract: self.nft_contract,
            token_id: self.token_id.parse().map_err(|_| ParsingError::TokenId)?,
            nonce: self.nonce.parse().map_err(|_| ParsingError::Nonce)?,
            signature: self.signature.try_into().map_err(|_| ParsingError::Signature)?
        };
        Ok(listing)
    }
}