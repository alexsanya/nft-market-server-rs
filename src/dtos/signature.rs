
use serde::{Serialize, Deserialize};

use crate::{models::signature::Signature, utils::patterns::Patterns};

#[derive(Debug, Serialize, Deserialize)]
pub struct SignatureDTO {
    pub v: u64,
    pub r: String,
    pub s: String
}

#[derive(Debug, Clone)]
pub enum ParsingError {
    RisInvalid,
    SisInvalid
}

impl TryInto<Signature> for SignatureDTO {
    type Error = ParsingError;

    fn try_into(self) -> Result<Signature, Self::Error> {
        let patterns = Patterns::new();
        Ok(
            Signature{
                v: self.v,
                r: patterns.test_bytes32(&self.r).map_err(|_| ParsingError::RisInvalid)?.to_owned(),
                s: patterns.test_bytes32(&self.s).map_err(|_| ParsingError::SisInvalid)?.to_owned()
            }
        )
    }
}