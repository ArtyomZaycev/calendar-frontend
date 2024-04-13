use calendar_lib::api::{
    auth::{types::NewPassword, *},
    user_state,
    utils::User,
};

use crate::{db::aliases::UserUtils, tables::TableId};

use super::{
    custom_requests::*,
    db_connector::DbConnectorData,
    main_state::{AdminState, State, UserState},
    request::{RequestId, RequestIdentifier, RequestType, StateRequestType},
    requests_holder::{RequestData, RequestsHolder},
    state_updater::{StateChecker, StateExecutor, StateUpdater},
};

impl State {
    pub(super) fn make_request<T, F>(info: T::Info, make_request: F) -> RequestIdentifier<T>
    where
        T: StateRequestType,
        F: FnOnce(&DbConnectorData) -> reqwest::RequestBuilder,
    {
        let rinfo = info.clone();
        let make_checker = |request_id| {
            let checker: StateChecker = Box::new(move |state| {
                if state.db_connector.is_request_completed(request_id) {
                    state
                        .db_connector
                        .convert_response::<T::Response, T::BadResponse>(request_id);

                    let info = info.clone();
                    let identifier: RequestIdentifier<T> =
                        RequestIdentifier::new(request_id, info.clone());
                    let executor: StateExecutor = Box::new(move |state: &mut State| {
                        let response = state.take_response(&identifier);
                        if let Some(response) = response {
                            match response {
                                Ok(response) => T::push_to_state(*response, info, state),
                                Err(response) => T::push_bad_to_state(*response, info, state),
                            }
                        }
                    });
                    Some(executor)
                } else {
                    None
                }
            });
            checker
        };
        Self::make_request_custom(rinfo, make_request, make_checker)
    }

    pub(super) fn make_request_custom<T, F, G>(
        info: T::Info,
        make_request: F,
        make_checker: G,
    ) -> RequestIdentifier<T>
    where
        T: RequestType,
        F: FnOnce(&DbConnectorData) -> reqwest::RequestBuilder,
        G: FnOnce(RequestId) -> StateChecker,
    {
        let connector = DbConnectorData::get();
        let request_id = connector.next_request_id();
        let request = make_request(connector);
        RequestsHolder::get().push(RequestData::new(request_id, request.build().unwrap()));
        let identifier: RequestIdentifier<T> = RequestIdentifier::new(request_id, info.clone());
        StateUpdater::get().push_checker(make_checker(request_id));
        identifier
    }

    pub fn logout(&mut self) -> RequestIdentifier<LogoutRequest> {
        self.user_state = UserState::new();
        self.admin_state = AdminState::new();
        self.me = User::default();
        State::make_request((), |connector| {
            connector
                .make_request::<LogoutRequest>()
                .json(&logout::Body {})
        })
    }

    pub fn login(&self, email: String, password: String) -> RequestIdentifier<LoginRequest> {
        State::make_request((), |connector| {
            connector
                .make_request::<LoginRequest>()
                .json(&login::Body { email, password })
        })
    }

    pub fn login_by_jwt(&self, key: String) -> RequestIdentifier<LoginByKeyRequest> {
        State::make_request((), |connector| {
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
        State::make_request((), |connector| {
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
        self.user_state.load_state();
        if self.me.is_admin() {
            self.admin_state.load_state();
        }
    }
}

impl UserState {
    pub fn load_state(&self) -> RequestIdentifier<LoadStateRequest> {
        State::make_request(None, |connector| {
            connector
                .make_request::<LoadStateRequest>()
                .query(&user_state::load::Args { user_id: None })
        })
    }

    pub fn insert_password(
        &self,
        user_id: i32,
        access_level: i32,
        viewer_password: Option<NewPassword>,
        editor_password: Option<NewPassword>,
    ) -> RequestIdentifier<NewPasswordRequest> {
        State::make_request((), |connector| {
            connector
                .make_request::<NewPasswordRequest>()
                .json(&new_password::Body {
                    user_id,
                    access_level,
                    viewer_password,
                    editor_password,
                })
        })
    }
}

impl AdminState {
    pub fn load_state(&self) {
        self.users.load_all();
    }

    pub fn load_user_state(&self, user_id: TableId) -> RequestIdentifier<LoadStateRequest> {
        State::make_request(Some(user_id), |connector| {
            connector
                .make_request::<LoadStateRequest>()
                .query(&user_state::load::Args {
                    user_id: Some(user_id),
                })
        })
    }
}
