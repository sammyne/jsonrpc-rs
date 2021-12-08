pub struct Metadata {
    pub id: Option<String>,
}

mod request;
mod response;

pub use request::*;
pub use response::*;
