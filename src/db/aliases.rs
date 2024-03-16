pub use calendar_lib::api::*;
pub use event_templates::types::*;
pub use events::types::*;
use itertools::Itertools;
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
    // TODO: Move to Connector?
    pub jwt: String,
}

impl UserInfo {
    pub fn is_admin(&self) -> bool {
        self.has_role(Role::Admin) || self.has_role(Role::SuperAdmin)
    }

    pub fn has_role(&self, role: Role) -> bool {
        self.user.roles.iter().contains(&role)
    }
}
