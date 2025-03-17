use ethers::types::Signature;
use serde::{Deserialize, Serialize};
use ethers::types::Address;
use crate::error::{Entity, Error};
use crate::prelude::Result as MyResult;
use super::bid::Bid;
use super::signature::Signature as SigString;
use super::bid_eip712::Bid as BidEIP712;
use std::str::FromStr;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settlement {
    pub bid: Bid,
    pub signature: SigString
}

impl Settlement {
    pub fn verify_signature(&self) -> MyResult<()> {
        self.bid.verify_signature()?;
        let signature: Signature = self.signature.clone().try_into().map_err(|_| Error::InvalidSignature(Entity::Settlement))?;
        let bid_eip712: BidEIP712 = self.bid.clone().try_into().map_err(|_| Error::InvalidSignature(Entity::Bid))?;
        let recovered = signature.recover_typed_data(&bid_eip712);
        let owner_result = Address::from_str(&self.bid.listing.owner);
        match (owner_result, recovered) {
            (Ok(owner), Ok(recovered)) if owner == recovered => Ok(()),
            _ => Err(Error::InvalidSignature(Entity::Settlement))
        }
    }
}

#[cfg(test)]
mod test {
    use crate::models::samples::test::get_settlement;

    use super::*;

    #[test]
    pub fn check_signature_ok() {
        let settlement = get_settlement();
        settlement.verify_signature().expect("Error");
    }

    #[test]
    fn check_sig_incorrect() {
        let settlement = get_settlement();
        let settlement_incorrect_sig = Settlement {
            signature: SigString {
                v: 29,
                ..settlement.signature
            },
            ..settlement
        };
        assert!(matches!(settlement_incorrect_sig.verify_signature(), Err(Error::InvalidSignature(Entity::Settlement))));
    }
}