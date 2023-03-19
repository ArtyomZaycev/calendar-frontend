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
        let levels = self
            .access_levels
            .iter()
            .filter(|l| l.level == self.current_access_level)
            .collect::<Vec<_>>();
        if levels.len() == 0 {
            self.access_levels.last().unwrap().clone()
        } else if levels.len() == 1 {
            levels[0].clone()
        } else {
            (*levels.iter().find(|v| v.edit_rights).unwrap_or(&levels[0])).clone()
        }
    }
}
