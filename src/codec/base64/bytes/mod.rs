use serde::de::Error;
use serde::{Deserialize, Deserializer};
use serde::{Serialize, Serializer};

pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<u8>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    base64::decode(s).map_err(|err| D::Error::custom(err.to_string()))
}

pub fn serialize<S>(v: &[u8], serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    base64::encode(v).serialize(serializer)
}

pub mod optionals;

#[cfg(test)]
mod tests;
