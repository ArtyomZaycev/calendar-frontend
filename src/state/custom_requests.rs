use calendar_lib::api::{auth::*, sharing, user_state};

use crate::{db::aliases::UserUtils, tables::TableId};

use super::{main_state::State, request::*, shared_state::GrantedUserState};

/* TODO:
    admin requests:
        load_user_memory_usage
        user_roles
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

#[derive(Debug, Clone)]
pub struct LoginInfo {
    pub email: String,
    pub password: String,
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

    type Info = LoginInfo;
}
#[allow(unused_variables)]
impl StateRequestType for LoginRequest {
    fn push_to_state(response: Self::Response, info: Self::Info, state: &mut State) {
        state.on_logged_in(response.user, response.jwt);
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
        state.on_logged_in(response.user, response.jwt);
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
pub struct LoadStateRequest {}
impl RequestType for LoadStateRequest {
    const URL: &'static str = user_state::load::PATH;
    const IS_AUTHORIZED: bool = true;
    const METHOD: reqwest::Method = user_state::load::METHOD;

    type Query = user_state::load::Args;
    type Response = user_state::load::Response;
    type BadResponse = user_state::load::BadRequestResponse;

    /// user_id
    type Info = TableId;
}
#[allow(unused_variables)]
impl StateRequestType for LoadStateRequest {
    fn push_to_state(response: Self::Response, info: Self::Info, state: &mut State) {
        let user_id = info;
        state.get_user_state_mut(user_id).replace_data(response);
        state.clear_events(user_id);
    }

    fn push_bad_to_state(response: Self::BadResponse, info: Self::Info, state: &mut State) {
        let user_id = info;
        if state.me.is_admin() {
            match response {
                user_state::load::BadRequestResponse::UserNotFound => {
                    state.admin_state.users.get_table_mut().remove_one(user_id);
                    state.admin_state.users_data.remove(&user_id);
                }
            }
        } else {
            println!("Failed loading state");
            state.clear_events(user_id);
        }
    }
}

#[derive(Clone, Copy)]
pub struct LoadGrantedPermissionsRequest {}
impl RequestType for LoadGrantedPermissionsRequest {
    const URL: &'static str = sharing::load_granted_permissions::PATH;
    const IS_AUTHORIZED: bool = true;
    const METHOD: reqwest::Method = sharing::load_granted_permissions::METHOD;

    type Query = sharing::load_granted_permissions::Args;
    type Response = sharing::load_granted_permissions::Response;

    /// user_id
    type Info = TableId;
}
#[allow(unused_variables)]
impl StateRequestType for LoadGrantedPermissionsRequest {
    fn push_to_state(response: Self::Response, info: Self::Info, state: &mut State) {
        let user_id = info;

        if user_id == state.me.id {
            state.granted_states = response
                .iter()
                .cloned()
                .map(|(user, permission)| {
                    let granted = GrantedUserState::new(user, permission.permissions);
                    granted.state.load_state();
                    granted
                })
                .collect();
        }

        state
            .get_user_state_mut(user_id)
            .granted_permissions
            .default_push_from_load_all(
                response
                    .into_iter()
                    .map(|(_, permission)| permission)
                    .collect(),
            );
    }

    fn push_bad_to_state(response: Self::BadResponse, info: Self::Info, state: &mut State) {
        let user_id = info;
        state.granted_states.clear();
        state.get_user_state_mut(user_id).granted_permissions.default_push_bad_from_load_all();
    }
}
