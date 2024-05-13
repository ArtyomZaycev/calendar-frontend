use calendar_lib::api::{auth::*, user_state, utils::User};

use crate::{
    db::{aliases::UserUtils, request::RequestIdentifier},
    tables::TableId,
};

use super::{
    custom_requests::*,
    main_state::{AdminState, State, UserState},
    request::make_state_request,
};

impl State {
    pub fn logout(&mut self) -> RequestIdentifier<LogoutRequest> {
        self.user_state = UserState::new(-1);
        self.admin_state = AdminState::new();
        self.me = User::default();
        make_state_request((), |connector| {
            connector
                .make_request::<LogoutRequest>()
                .json(&logout::Body {})
        })
    }

    pub fn login(&self, email: String, password: String) -> RequestIdentifier<LoginRequest> {
        make_state_request(
            LoginInfo {
                email: email.clone(),
                password: password.clone(),
            },
            |connector| {
                connector
                    .make_request::<LoginRequest>()
                    .json(&login::Body { email, password })
            },
        )
    }

    pub fn login_by_jwt(&self, key: String) -> RequestIdentifier<LoginByKeyRequest> {
        make_state_request((), |connector| {
            connector
                .make_request::<LoginByKeyRequest>()
                .json(&login_by_key::Body {})
                .bearer_auth(key)
        })
    }

    pub fn register(
        &self,
        name: String,
        email: String,
        password: String,
    ) -> RequestIdentifier<RegisterRequest> {
        make_state_request((), |connector| {
            connector
                .make_request::<RegisterRequest>()
                .json(&register::Body {
                    name,
                    email,
                    password,
                })
        })
    }

    pub fn load_state(&self) {
        if self.me.is_admin() {
            self.admin_state.load_state();
        } else {
            self.user_state.load_state();
        }
    }
}

impl UserState {
    pub fn load_state(&self) -> RequestIdentifier<LoadStateRequest> {
        make_state_request(self.user_id, |connector| {
            connector
                .make_request::<LoadStateRequest>()
                .query(&user_state::load::Args {
                    user_id: self.user_id,
                })
        })
    }
}

impl AdminState {
    pub fn load_state(&self) {
        self.users.load_all();
    }

    pub fn load_user_state(&self, user_id: TableId) -> RequestIdentifier<LoadStateRequest> {
        make_state_request(user_id, |connector| {
            connector
                .make_request::<LoadStateRequest>()
                .query(&user_state::load::Args { user_id })
        })
    }
}
