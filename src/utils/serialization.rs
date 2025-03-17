use num_bigint::BigInt;
use serde::Serializer;

pub fn serialize_bigint_as_string<S>(value: &BigInt, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(&value.to_string())
}