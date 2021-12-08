use serde::{Deserialize, Serialize};

// https://github.com/rust-lang/rust-clippy/issues/1689: Warn about trait bounds on struct and enum
//  type parameters
// https://github.com/serde-rs/serde/issues/890: Deserialize lifetime in generic context
// https://github.com/serde-rs/serde/issues/964: How to write a generic type bounds for deserialize
//  trait?
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Request<T>
//where
//    T: DeserializeOwned + Serialize,
{
    jsonrpc: String,

    pub method: String,
    //#[serde(bound = "")]
    pub params: T,
    pub id: Option<String>,
}

impl<'de, T> Request<T>
where
    T: Deserialize<'de> + Serialize,
{
    pub fn new<S>(method: S, params: T, id: Option<String>) -> Self
    where
        S: ToString,
    {
        Self {
            jsonrpc: "2.0".to_string(),
            method: method.to_string(),
            params,
            id,
        }
    }

    pub fn version(&self) -> &str {
        &self.jsonrpc
    }
}
