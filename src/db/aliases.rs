pub use calendar_lib::api::*;
pub use event_templates::types::*;
pub use events::types::*;
use itertools::Itertools;
pub use roles::types::*;
pub use schedules::types::*;
use serde::Deserialize;
pub use utils::User;

#[derive(Debug, Deserialize)]
pub struct EchoStruct {
    pub echo: String,
}

pub trait UserUtils {
    fn is_admin(&self) -> bool;
    fn has_role(&self, role: Role) -> bool;
}

impl UserUtils for User {
    fn is_admin(&self) -> bool {
        self.has_role(Role::Admin) || self.has_role(Role::SuperAdmin)
    }

    fn has_role(&self, role: Role) -> bool {
        self.roles.iter().contains(&role)
    }
}
