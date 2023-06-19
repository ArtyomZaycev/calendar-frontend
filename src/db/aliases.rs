use calendar_lib::api::utils;
pub use calendar_lib::api::*;
pub use event_templates::types::*;
pub use events::types::*;
pub use roles::types::*;
pub use schedules::types::*;
use serde::{Deserialize, Serialize};
pub use utils::User;

#[derive(Debug, Deserialize)]
pub struct EchoStruct {
    pub echo: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserInfo {
    pub user: User,
    pub jwt: String,
    pub roles: Vec<Role>,
}
