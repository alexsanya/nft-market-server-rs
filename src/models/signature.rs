
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Signature {
    pub v: u8,
    pub r: String,
    pub s: String
}