use calendar_lib::api::{sharing::Permissions, utils::User};

use super::main_state::UserState;

pub struct GrantedUserState {
    pub user: User, // no roles
    pub state: UserState,
    pub permissions: Permissions,
}

impl GrantedUserState {
    pub fn new(user: User, permissions: Permissions) -> Self {
        Self {
            state: UserState::new(user.id),
            user,
            permissions,
        }
    }
}
