use calendar_lib::api::{auth::*, user_state};

use crate::tables::TableId;

use super::{
    db_connector::DbConnectorData,
    main_state::{RequestType, State},
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

    fn push_to_state(response: Self::Response, info: Self::Info, state: &mut State) {
        // We should clear all data as soon as request is made, not after it's done
    }
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

    fn push_to_state(response: Self::Response, info: Self::Info, state: &mut State) {
        DbConnectorData::get().push_jwt(response.jwt);
        state.me = response.user;
        state.current_access_level = response.access_level.level;
        state
            .user_state
            .access_levels
            .get_table_mut()
            .replace_all(vec![response.access_level]);

        state.user_state.load_state();
    }
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

    fn push_to_state(response: Self::Response, info: Self::Info, state: &mut State) {
        DbConnectorData::get().push_jwt(response.jwt);
        state.me = response.user;
        state.current_access_level = response.access_level.level;
        state
            .user_state
            .access_levels
            .get_table_mut()
            .replace_all(vec![response.access_level]);

        state.user_state.load_state();
    }
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

    type Info = ();

    fn push_to_state(response: Self::Response, info: Self::Info, state: &mut State) {}
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

    fn push_to_state(response: Self::Response, info: Self::Info, state: &mut State) {
        state.user_state.access_levels.load_all();
    }
}

#[derive(Clone, Copy)]
pub struct LoadStateRequest {}
impl RequestType for LoadStateRequest {
    const URL: &'static str = user_state::load::PATH;
    const IS_AUTHORIZED: bool = true;
    const METHOD: reqwest::Method = user_state::load::METHOD;

    type Query = user_state::load::Args;
    type Response = user_state::load::Response;

    /// None for loading own state, Some(user_id) for admin request
    type Info = Option<TableId>;

    fn push_to_state(response: Self::Response, info: Self::Info, state: &mut State) {
        match info {
            Some(user_id) => {
                state
                    .admin_state
                    .users_data
                    .insert(user_id, response.into());
            }
            None => {
                // TODO: Properly replace data, we are losing requests now
                state.user_state = response.into();
            }
        }
    }
}
