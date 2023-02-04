use calendar_lib::api::{auth::login, user_roles, events};
use reqwest::{Method, RequestBuilder, Response};

use super::{
    aliases::*,
    connector::Connector,
    request::{self, AppRequest},
};

enum StateAction {
    Login(login::Response),
    LoadUserRoles(user_roles::load_array::Response),
    LoadEvents(events::load_array::Response),

    Error(Response),
}

pub struct State {
    connector: Connector<StateAction>,

    pub me: Option<UserInfo>,

    // Should not be modified manually, use requests
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
    fn make_request(&self, method: Method, op: &str) -> RequestBuilder {
        self.connector.make_request(method, op)
    }

    fn make_request_authorized(&self, method: Method, op: &str) -> RequestBuilder {
        if let Some(me) = &self.me {
            self.connector
                .make_request_authorized(method, op, me.user.id, &me.key)
        } else {
            todo!()
        }
    }
}

impl State {
    pub fn load_user_roles(&mut self) {
        let on_success: request::OnSuccess<StateAction, user_roles::load_array::Response> =
            Box::new(|response| StateAction::LoadUserRoles(response));
        let on_error: request::OnError<StateAction> = Box::new(|e| StateAction::Error(e));

        let req = self
            .make_request_authorized(Method::GET, "user_roles")
            .build()
            .unwrap();

        self.connector
            .request(AppRequest::new(req, on_success, on_error));
    }

    pub fn login(&mut self, email: &str, pass: &str) {
        let on_success: request::OnSuccess<StateAction, login::Response> =
            Box::new(|response| StateAction::Login(response));
        let on_error: request::OnError<StateAction> = Box::new(|e| StateAction::Error(e));
        let req = self
            .make_request(Method::POST, "auth/login")
            .json(&login::Body {
                email: email.to_string(),
                password: pass.to_string(),
            })
            .build()
            .unwrap();
        self.connector
            .request(AppRequest::new(req, on_success, on_error));
    }

    pub fn load_events(&mut self) {
        let on_success: request::OnSuccess<StateAction, events::load_array::Response> =
            Box::new(|response| StateAction::LoadEvents(response));
        let on_error: request::OnError<StateAction> = Box::new(|e| StateAction::Error(e));
        let req = self
            .make_request_authorized(Method::GET, "events")
            .build()
            .unwrap();
        self.connector
            .request(AppRequest::new(req, on_success, on_error));
    }

    pub fn insert_event(&mut self, new_event: &events::insert::Body) {
        let on_error: request::OnError<StateAction> = Box::new(|e| StateAction::Error(e));
        let req = self
            .make_request_authorized(Method::POST, "event")
            .json(new_event)
            .build()
            .unwrap();
        self.connector
            .request(AppRequest::new_ignore(req, on_error));
    }

    pub fn delete_event(&mut self, id: i32) {
        let on_error: request::OnError<StateAction> = Box::new(|e| StateAction::Error(e));
        let req = self
            .make_request_authorized(Method::DELETE, "event")
            .query(&events::delete::Args { id })
            .json(&events::delete::Body {})
            .build()
            .unwrap();
        self.connector
            .request(AppRequest::new_ignore(req, on_error));
    }
}

impl State {
    pub fn poll(&mut self) {
        let actions = self.connector.poll();

        for action in actions {
            match action {
                StateAction::Login(res) => {
                    self.me = Some(UserInfo {
                        user: res.user,
                        access_level: res.access_level,
                        edit_rights: res.edit_rights,
                        key: res.key,
                        roles: vec![],
                    });
                    self.load_events();
                    self.load_user_roles();
                }
                StateAction::LoadUserRoles(res) => {
                    if let Some(me) = &mut self.me {
                        me.roles = res.array;
                    }
                }
                StateAction::LoadEvents(res) => {
                    self.events = res.array;
                }
                StateAction::Error(res) => {
                    println!("smth went wrong: {res:?}");
                }
            }
        }
    }
}
