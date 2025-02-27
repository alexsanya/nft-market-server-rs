
use once_cell::sync::Lazy;
use regex::Regex;

pub struct Patterns {
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

    pub fn test_bytes32<'a>(&self, text: &'a str) -> Result<&'a str, ()> {
        if self.bytes32.is_match(text) {
            Ok(text)
        } else {
            Err(())
        }
    }
}