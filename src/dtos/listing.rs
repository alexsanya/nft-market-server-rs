use serde::{Serialize, Deserialize};
use crate::{models::listing::Listing, utils::patterns::Patterns};
use super::signature::{self, SignatureDTO};

#[derive(Debug)]
pub enum ParsingError {
    Owner,
    ChainId,
    NftContract,
    TokenId,
    Nonce,
    Signature(signature::ParsingError)
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
        let patterns = Patterns::new();
        let listing = Listing{
            owner: patterns.test_address(&self.owner).map_err(|_| ParsingError::Owner)?.to_owned(),
            chain_id: self.chain_id.parse().map_err(|_| ParsingError::ChainId)?,
            min_price_cents: self.min_price_cents,
            nft_contract: patterns.test_address(&self.nft_contract).map_err(|_| ParsingError::NftContract)?.to_owned(),
            token_id: self.token_id.parse().map_err(|_| ParsingError::TokenId)?,
            nonce: self.nonce.parse().map_err(|_| ParsingError::Nonce)?,
            signature: self.signature.try_into().map_err(ParsingError::Signature)?
        };
        Ok(listing)
    }
}