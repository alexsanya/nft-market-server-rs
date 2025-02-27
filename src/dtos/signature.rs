
use serde::{Serialize, Deserialize};

use crate::models::signature::Signature;

#[derive(Debug, Serialize, Deserialize)]
pub struct SignatureDTO {
    pub v: u8,
    pub r: String,
    pub s: String
}

pub enum ParsingError {
    rIsInvalid,
    sIsInvalid
}

impl TryInto<Signature> for SignatureDTO {
    type Error = ParsingError;

    fn try_into(self) -> Result<Signature, Self::Error> {
        Ok(
            Signature{
                v: self.v,
                r: self.r,
                s: self.s
            }
        )
    }
}