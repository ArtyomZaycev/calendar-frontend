use std::collections::HashMap;

use calendar_lib::api::{utils::User, user_state, users};

use super::UserState;

pub struct AdminState {
    // user_ids could be very big, updated only on load_user_ids request
    user_ids: Vec<i32>,
    users: HashMap<i32, User>,
    users_data: HashMap<i32, UserState>,
}

impl AdminState {
    pub fn new() -> Self {
        Self {
            user_ids: Vec::default(),
            users: HashMap::default(),
            users_data: HashMap::default(),
        }
    }
}

impl AdminState {
    pub(super) fn parse_load_user_ids(&mut self, response: users::load_ids::Response) {
        self.user_ids = response.array;
    }

    pub(super) fn parse_load_user(&mut self, response: users::load::Response) {
        self.users.insert(response.value.id, response.value);
    }

    pub(super) fn parse_load_user_error(&mut self, user_id: i32, response: users::load::BadRequestResponse) {
        match response {
            users::load::BadRequestResponse::NotFound => {
                self.users.remove(&user_id);
                self.users_data.remove(&user_id);
            },
        };
    }

    pub(super) fn parse_load_users(&mut self, response: users::load_array::Response) {
        self.users.extend(response.array.into_iter().map(|user| (user.id, user)));
    }

    pub(super) fn parse_load_state(&mut self, user_id: i32, response: user_state::load::Response) {
        self.users_data.insert(user_id, response.into());
    }

    pub(super) fn parse_load_state_error(&mut self, user_id: i32, response: user_state::load::BadRequestResponse) {
        match response {
            user_state::load::BadRequestResponse::UserNotFound => {
                self.users.remove(&user_id);
                self.users_data.remove(&user_id);
            },
        };
    }
}