use std::collections::HashMap;

use calendar_lib::api::{user_state, users, utils::User};

use crate::tables::{table::Table, DbTable};

use super::UserState;

pub struct AdminState {
    // TODO: Lazy table
    pub users: Table<User>,
    users_data: HashMap<i32, UserState>,
}

impl AdminState {
    pub fn new() -> Self {
        Self {
            users: Table::default(),
            users_data: HashMap::default(),
        }
    }
}

impl AdminState {
    pub(super) fn parse_load_user_error(
        &mut self,
        user_id: i32,
        response: users::load::BadRequestResponse,
    ) {
        match response {
            users::load::BadRequestResponse::NotFound => {
                self.users.remove_one(user_id);
                self.users_data.remove(&user_id);
            }
        };
    }

    pub(super) fn parse_load_users(&mut self, response: users::load_array::Response) {
        *self.users.get_mut() = response.array;
    }

    pub(super) fn parse_load_state(&mut self, user_id: i32, response: user_state::load::Response) {
        self.users_data.insert(user_id, response.into());
    }

    pub(super) fn parse_load_state_error(
        &mut self,
        user_id: i32,
        response: user_state::load::BadRequestResponse,
    ) {
        match response {
            user_state::load::BadRequestResponse::UserNotFound => {
                self.users.remove_one(user_id);
                self.users_data.remove(&user_id);
            }
        };
    }
}
