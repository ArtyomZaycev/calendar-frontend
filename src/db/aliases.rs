pub use calendar_lib::api::*;
use calendar_lib::api::{auth::types::AccessLevel, utils};
use serde::{Deserialize, Serialize};

pub use event_templates::types::*;
pub use events::types::*;
pub use roles::types::*;
pub use schedules::types::*;
pub use utils::User;

#[derive(Debug, Deserialize)]
pub struct EchoStruct {
    pub echo: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserInfo {
    pub user: User,
    pub current_access_level: i32,
    pub access_levels: Vec<AccessLevel>,
    pub key: Vec<u8>,
    pub roles: Vec<Role>,
}

impl UserInfo {
    pub fn get_access_level(&self) -> AccessLevel {
        self.access_levels
            .iter()
            .find(|l| l.level == self.current_access_level)
            .cloned()
            .unwrap_or(self.access_levels.last().cloned().unwrap())
    }
}
