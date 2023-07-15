use std::collections::HashMap;

use calendar_lib::api::utils::User;

use super::UserState;

pub struct AdminState {
    pub(super) users: Vec<User>,
    pub(super) users_data: HashMap<i32, UserState>,
}

impl AdminState {
    pub fn new() -> Self {
        Self {
            users: Vec::default(),
            users_data: HashMap::default(),
        }
    }
}
