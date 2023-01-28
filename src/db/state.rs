use calendar_lib::api_types::{auth::login, roles::load_user_roles};
use reqwest::{Method, Response};

use super::{
    aliases::*,
    connector::Connector,
    request::{self, AppRequest},
};

enum StateAction {
    Login(login::Response),
    LoginError(Response),

    LoadUserRoles(roles::load_user_roles::Response),
    LoadUserRolesError(Response),
}

pub struct State {
    connector: Connector<StateAction>,

    pub me: Option<UserInfo>,

    pub users: Vec<()>,
    pub events: Vec<Event>,

    pub errors: Vec<()>,
}

impl State {
    pub fn new() -> Self {
        Self {
            connector: Connector::new(),
            me: None,
            users: Vec::default(),
            events: Vec::default(),
            errors: Vec::default(),
        }
    }
}

impl State {
    pub fn load_user_roles(&self) {

        if let Some(me) = &self.me {
            let on_success: request::OnSuccess<StateAction, load_user_roles::Response> =
                Box::new(|response| StateAction::LoadUserRoles(response));
            let on_error: request::OnError<StateAction> =
                Box::new(|e| StateAction::LoadUserRolesError(e));

            self.connector.request(AppRequest::new(
                self.connector
                    .client
                    .request(Method::GET, "http://127.0.0.1:8080/user_roles")
                    .basic_auth(
                        me.user.user_id,
                        Some(std::str::from_utf8(&me.user.key).expect("parse error")),
                    )
                    .build()
                    .unwrap(),
                on_success,
                on_error,
            ));
        } else {
            // TODO
            println!("No auth");
        }
    }

    pub fn login(&self, email: &str, pass: &str) {

        let on_success: request::OnSuccess<StateAction, login::Response> =
            Box::new(|response| StateAction::Login(response));
        let on_error: request::OnError<StateAction> = Box::new(|e| StateAction::LoginError(e));
        self.connector.request(AppRequest::new(
            self.connector
                .client
                .request(Method::POST, "http://127.0.0.1:8080/auth/login")
                .json(&login::Body {
                    email: email.to_string(),
                    password: pass.to_string(),
                })
                .build()
                .unwrap(),
            on_success,
            on_error,
        ));
    }

    pub fn poll(&mut self) {
        let actions = self.connector.poll();

        for action in actions {
            match action {
                StateAction::Login(res) => {
                    self.me = Some(UserInfo {
                        user: res,
                        roles: vec![],
                    })
                }
                StateAction::LoginError(res) => {
                    println!("smth went wrong: {res:?}");
                }
                StateAction::LoadUserRoles(res) => {
                    if let Some(me) = &mut self.me {
                        me.roles = res.array;
                    }
                }
                StateAction::LoadUserRolesError(res) => {
                    println!("smth went wrong: {res:?}");
                }
            }
        }
    }
}
