use serde::{Deserialize, Serialize};

use crate::errors::Error;

#[derive(Deserialize, Serialize)]
pub struct Response<T> {
    jsonrpc: String,

    pub result: Option<T>,
    pub id: Option<String>,

    pub error: Option<Error>,
}

impl<'de, T> Response<T>
where
    T: Deserialize<'de> + Serialize,
{
    pub fn version(&self) -> &str {
        &self.jsonrpc
    }

    pub fn new_err(error: Error) -> Self {
        Self {
            jsonrpc: "2.0".to_string(),
            result: None,
            error: Some(error),
            id: None,
        }
    }
    pub fn new_ok(result: T, id: String) -> Self {
        Self {
            jsonrpc: "2.0".to_string(),
            result: Some(result),
            error: None,
            id: Some(id),
        }
    }
}
