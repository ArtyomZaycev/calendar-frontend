use calendar_lib::api::{sharing::SharedPermissions, utils::User};

use super::main_state::UserState;

pub struct SharedUserState {
    pub user: User, // no roles
    pub state: UserState,
    pub permissions: SharedPermissions,
}
