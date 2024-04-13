use calendar_lib::api::{auth::*, user_state};

use crate::tables::TableId;

use super::{
    db_connector::DbConnectorData,
    main_state::State,
    request::{RequestType, StateRequestType},
};

/* TODO:
    admin requests:
        load_user_memory_usage
        user_roles
        load_users
*/

#[derive(Clone, Copy)]
pub struct LogoutRequest {}
impl RequestType for LogoutRequest {
    const URL: &'static str = logout::PATH;
    const IS_AUTHORIZED: bool = true;
    const METHOD: reqwest::Method = logout::METHOD;

    type Query = logout::Args;
    type Body = logout::Body;
    type Response = logout::Response;

    type Info = ();
}
#[allow(unused_variables)]
impl StateRequestType for LogoutRequest {
    fn push_to_state(response: Self::Response, info: Self::Info, state: &mut State) {
        // We should clear all data as soon as request is made, not after it's done
    }

    fn push_bad_to_state(response: Self::BadResponse, info: Self::Info, state: &mut State) {}
}

#[derive(Clone, Copy)]
pub struct LoginRequest {}
impl RequestType for LoginRequest {
    const URL: &'static str = login::PATH;
    const IS_AUTHORIZED: bool = false;
    const METHOD: reqwest::Method = login::METHOD;

    type Query = login::Args;
    type Body = login::Body;
    type Response = login::Response;
    type BadResponse = login::BadRequestResponse;

    type Info = ();
}
#[allow(unused_variables)]
impl StateRequestType for LoginRequest {
    fn push_to_state(response: Self::Response, info: Self::Info, state: &mut State) {
        DbConnectorData::get().push_jwt(response.jwt);
        state.me = response.user;
        state.current_access_level = response.access_level.level;
        state
            .user_state
            .access_levels
            .get_table_mut()
            .replace_all(vec![response.access_level]);

        state.load_state();
    }

    fn push_bad_to_state(response: Self::BadResponse, info: Self::Info, state: &mut State) {}
}

#[derive(Clone, Copy)]
pub struct LoginByKeyRequest {}
impl RequestType for LoginByKeyRequest {
    const URL: &'static str = login_by_key::PATH;
    const IS_AUTHORIZED: bool = false;
    const METHOD: reqwest::Method = login_by_key::METHOD;

    type Query = login_by_key::Args;
    type Body = login_by_key::Body;
    type Response = login_by_key::Response;

    type Info = ();
}
#[allow(unused_variables)]
impl StateRequestType for LoginByKeyRequest {
    fn push_to_state(response: Self::Response, info: Self::Info, state: &mut State) {
        DbConnectorData::get().push_jwt(response.jwt);
        state.me = response.user;
        state.current_access_level = response.access_level.level;
        state
            .user_state
            .access_levels
            .get_table_mut()
            .replace_all(vec![response.access_level]);

        state.load_state();
    }

    fn push_bad_to_state(response: Self::BadResponse, info: Self::Info, state: &mut State) {}
}

#[derive(Clone, Copy)]
pub struct RegisterRequest {}
impl RequestType for RegisterRequest {
    const URL: &'static str = register::PATH;
    const IS_AUTHORIZED: bool = false;
    const METHOD: reqwest::Method = register::METHOD;

    type Query = register::Args;
    type Body = register::Body;
    type Response = register::Response;
    type BadResponse = register::BadRequestResponse;

    type Info = ();
}
#[allow(unused_variables)]
impl StateRequestType for RegisterRequest {
    fn push_to_state(response: Self::Response, info: Self::Info, state: &mut State) {}
    fn push_bad_to_state(response: Self::BadResponse, info: Self::Info, state: &mut State) {}
}

#[derive(Clone, Copy)]
pub struct NewPasswordRequest {}
impl RequestType for NewPasswordRequest {
    const URL: &'static str = new_password::PATH;
    const IS_AUTHORIZED: bool = false;
    const METHOD: reqwest::Method = new_password::METHOD;

    type Query = new_password::Args;
    type Body = new_password::Body;
    type Response = new_password::Response;

    type Info = ();
}
#[allow(unused_variables)]
impl StateRequestType for NewPasswordRequest {
    fn push_to_state(response: Self::Response, info: Self::Info, state: &mut State) {
        state.user_state.access_levels.load_all();
    }

    fn push_bad_to_state(response: Self::BadResponse, info: Self::Info, state: &mut State) {}
}

#[derive(Clone, Copy)]
pub struct LoadStateRequest {}
impl RequestType for LoadStateRequest {
    const URL: &'static str = user_state::load::PATH;
    const IS_AUTHORIZED: bool = true;
    const METHOD: reqwest::Method = user_state::load::METHOD;

    type Query = user_state::load::Args;
    type Response = user_state::load::Response;
    type BadResponse = user_state::load::BadRequestResponse;

    /// None for loading own state, Some(user_id) for admin request
    type Info = Option<TableId>;
}
#[allow(unused_variables)]
impl StateRequestType for LoadStateRequest {
    fn push_to_state(response: Self::Response, info: Self::Info, state: &mut State) {
        match info {
            Some(user_id) => {
                state
                    .admin_state
                    .users_data
                    .insert(user_id, response.into());
            }
            None => {
                state.user_state.replace_data(response);
                state.clear_events();
            }
        }
    }

    fn push_bad_to_state(response: Self::BadResponse, info: Self::Info, state: &mut State) {
        match info {
            Some(user_id) => match response {
                user_state::load::BadRequestResponse::UserNotFound => {
                    state.admin_state.users.get_table_mut().remove_one(user_id);
                    state.admin_state.users_data.remove(&user_id);
                }
            },
            None => {
                println!("Failed loading state");
                state.clear_events();
            }
        }
    }
}
