pub use calendar_lib::api_types::*;
use serde::Deserialize;

pub type Event = events::load::Response;
pub type Role = roles::load::Response;

#[derive(Debug, Deserialize)]
pub struct EchoStruct {
    pub echo: String,
}