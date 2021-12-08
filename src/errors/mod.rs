use std::io;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Error {
    pub code: i32,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<u8>>,
}

pub type Result<T> = std::result::Result<T, Error>;

impl Error {
    pub fn internal_error() -> Self {
        Self::new_reserved(-32603, "Internal error", None)
    }

    pub fn invalid_params() -> Self {
        Self::new_reserved(-32602, "Invalid params", None)
    }

    pub fn invalid_request() -> Self {
        Self::new_reserved(-32600, "Invalid Request", None)
    }

    pub fn method_not_found() -> Self {
        Self::new_reserved(-32601, "Method not found", None)
    }

    pub fn new<T>(code: i32, message: T, data: Option<Vec<u8>>) -> Self
    where
        T: ToString,
    {
        // TODO: check reserved
        Self {
            code,
            message: message.to_string(),
            data,
        }
    }

    pub fn parse_error() -> Self {
        Self::new_reserved(-32700, "Parse error", None)
    }

    pub fn wrap(self, msg: &str) -> Self {
        let mut out = self;
        out.message = format!("{}\n  -> {}", msg, &out.message);

        out
    }

    fn new_reserved(code: i32, message: &'static str, data: Option<Vec<u8>>) -> Self {
        Self {
            code,
            message: message.to_string(),
            data,
        }
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        let err_string = err.to_string();
        Self::internal_error().wrap(&err_string)
    }
}

impl From<serde_json::error::Error> for Error {
    fn from(err: serde_json::error::Error) -> Self {
        let err_string = err.to_string();
        Self::parse_error().wrap(&err_string)
    }
}

pub fn is_reserved_error_code(c: i32) -> bool {
    c >= -32768 && c <= -32000
}
