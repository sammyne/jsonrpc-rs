use serde::de::Error;
use serde::{Deserialize, Deserializer};
use serde::{Serialize, Serializer};

pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<Vec<u8>>, D::Error>
where
    D: Deserializer<'de>,
{
    match Option::<String>::deserialize(deserializer)? {
        Some(v) => base64::decode(v).map(Some).map_err(D::Error::custom),
        None => Ok(None),
    }
}

pub fn serialize<S>(v: &Option<Vec<u8>>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    v.as_ref()
        .map(|vv| base64::encode(vv))
        .serialize(serializer)
}
