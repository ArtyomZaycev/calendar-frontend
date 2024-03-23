use calendar_lib::api::auth::*;

use super::{db_connector::DbConnectorData, main_state::{RequestType, State}};


/* TODO:
    load_state
    admin requests:
        load_user_memory_usage
        user_roles
        load_users
*/

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
        state.me = Some(response.user);
        state.current_access_level = response.access_level.level;
        state.access_levels.get_table_mut().replace_all(vec![response.access_level]);

        state.load_state();
    }
}

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
        state.me = Some(response.user);
        state.current_access_level = response.access_level.level;
        state.access_levels.get_table_mut().replace_all(vec![response.access_level]);

        state.load_state();
    }
}

pub struct RegisterRequest {}
impl RequestType for RegisterRequest {
    const URL: &'static str = register::PATH;
    const IS_AUTHORIZED: bool = false;
    const METHOD: reqwest::Method = register::METHOD;

    type Query = register::Args;
    type Body = register::Body;
    type Response = register::Response;

    type Info = ();

    fn push_to_state(response: Self::Response, info: Self::Info, state: &mut State) {
        
    }
}

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
        state.access_levels.load_all();
    }
}