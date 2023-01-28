pub use calendar_lib::api_types::*;
use serde::{Deserialize, Serialize};

pub type Event = events::load::Response;
pub type Role = roles::load::Response;

#[derive(Debug, Deserialize)]
pub struct EchoStruct {
    pub echo: String,
}

pub type User = auth::login::Response;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserInfo {
    pub user: User,
    pub roles: Vec<Role>,
}
