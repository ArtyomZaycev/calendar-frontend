use calendar_lib::api::{sharing::GrantedPermissions, utils::User};

use super::main_state::UserState;

pub struct GrantedUserState {
    pub user: User, // no roles
    pub state: UserState,
    pub permissions: GrantedPermissions,
}
