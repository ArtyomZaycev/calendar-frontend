use calendar_lib::api::utils;
pub use calendar_lib::api::*;
use serde::{Deserialize, Serialize};

pub use events::types::*;
pub use roles::types::*;
pub use utils::User;

#[derive(Debug, Deserialize)]
pub struct EchoStruct {
    pub echo: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserInfo {
    pub user: User,
    pub access_level: i32,
    pub edit_rights: bool,
    pub key: Vec<u8>,
    pub roles: Vec<Role>,
}
