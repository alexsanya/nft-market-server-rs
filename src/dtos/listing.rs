use once_cell::sync::Lazy;
use regex::Regex;
use serde::{Serialize, Deserialize};

use crate::models::listing::Listing;

use super::signature::SignatureDTO;

#[derive(Debug)]
pub enum ParsingError {
    Owner,
    ChainId,
    NftContract,
    TokenId,
    Nonce,
    Signature
}


struct Patterns {
    address: Lazy<Regex>,
    bytes32: Lazy<Regex>
}

impl Patterns {
    pub fn new() -> Self {
        Patterns {
            address: Lazy::new(|| Regex::new(r"^(0x)?[0-9a-fA-F]{40}").unwrap()),
            bytes32: Lazy::new(|| Regex::new(r"^(0x)?[0-9a-fA-F]{64}").unwrap())
        }
    }

    pub fn test_address<'a>(&self, text: &'a str) -> Result<&'a str, ()> {
        if self.address.is_match(text) {
            Ok(text)
        } else {
            Err(())
        }
    }

    pub fn test_bytes32(self, text: &str) -> Result<&str, ()> {
        if self.bytes32.is_match(text) {
            Ok(text)
        } else {
            Err(())
        }
    }
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
            signature: self.signature.try_into().map_err(|_| ParsingError::Signature)?
        };
        Ok(listing)
    }
}