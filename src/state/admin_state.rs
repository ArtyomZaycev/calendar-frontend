use std::collections::HashMap;

use calendar_lib::api::utils::User;

use super::{state_table::StateTable, user_state::UserState};

pub struct AdminState {
    pub users: StateTable<User>,
    pub users_data: HashMap<i32, UserState>,
}

impl AdminState {
    pub(super) fn new() -> Self {
        Self {
            users: StateTable::new(),
            users_data: HashMap::default(),
        }
    }
}
