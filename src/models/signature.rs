
use ethers::types::{Signature as EthSignature, U256};
use serde::{Serialize, Deserialize};
use std::str::FromStr;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Signature {
    pub v: u64,
    pub r: String,
    pub s: String
}


impl TryInto<EthSignature> for Signature {
    type Error = ();

    fn try_into(self) -> Result<EthSignature, Self::Error> {
        let r = U256::from_str(&self.r).unwrap();
        let s = U256::from_str(&self.s).unwrap();
        let signature = EthSignature{ r, s, v: self.v };
        Ok(signature)
    }
}