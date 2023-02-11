use calendar_lib::api::{auth::login, user_roles, events};
use reqwest::{Method, RequestBuilder, StatusCode};
use serde::de::DeserializeOwned;

use super::{
    aliases::*,
    connector::{Connector, RequestDescriptor},
    request_parser::RequestParser,
};

enum StateAction {
    Login(login::Response),
    LoadUserRoles(user_roles::load_array::Response),
    LoadEvents(events::load_array::Response),
    InsertEvent(events::insert::Response),
    DeleteEvent(events::delete::Response),

    #[allow(dead_code)]
    None,
    Error(StatusCode, String),
}

pub struct State {
    connector: Connector<StateAction>,

    pub me: Option<UserInfo>,

    // Should not be modified manually, use requests
    pub users: Vec<User>,
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
                .make_request(method, op)
                .basic_auth(me.user.id, Some(std::str::from_utf8(&me.key).expect("parse error")))
        } else {
            todo!()
        }
    }

    // Use for testing only
    #[cfg(debug_assertions)]
    #[allow(dead_code)]
    fn make_empty_parser(&mut self) -> RequestParser<StateAction> {
        RequestParser::new_split(
            |_| StateAction::None, 
            |_, _| StateAction::None
        )
    }

    fn make_parser<U, F>(&mut self, on_success: F) -> RequestParser<StateAction> where
        U: DeserializeOwned,
        F: FnOnce(U) -> StateAction + 'static
    {
        RequestParser::new_complex(
            on_success,
            |code, s| StateAction::Error(code, s)
        )
    }
}

impl State {
    pub fn load_user_roles(&mut self) {
        let request = self
            .make_request_authorized(Method::GET, "user_roles")
            .build()
            .unwrap();

        let parser = self.make_parser(|r| StateAction::LoadUserRoles(r));
        self.connector.request(request, RequestDescriptor::new(parser));
    }

    pub fn login(&mut self, email: &str, pass: &str) {
        let request = self
            .make_request(Method::POST, "auth/login")
            .json(&login::Body {
                email: email.to_string(),
                password: pass.to_string(),
            })
            .build()
            .unwrap();

        let parser = self.make_parser(|r| StateAction::Login(r));
        self.connector.request(request, RequestDescriptor::new(parser));
    }

    pub fn load_events(&mut self) {
        let request = self
            .make_request_authorized(Method::GET, "events")
            .build()
            .unwrap();

        let parser = self.make_parser(|r| StateAction::LoadEvents(r));
        self.connector.request(request, RequestDescriptor::new(parser));
    }

    pub fn insert_event(&mut self, new_event: NewEvent) {
        let request = self
            .make_request_authorized(Method::POST, "event")
            .json(&events::insert::Body { new_event })
            .build()
            .unwrap();

        let parser = self.make_parser(|r| StateAction::InsertEvent(r));
        self.connector.request(request, RequestDescriptor::new(parser));
    }

    pub fn delete_event(&mut self, id: i32) {
        let request = self
            .make_request_authorized(Method::DELETE, "event")
            .query(&events::delete::Args { id })
            .json(&events::delete::Body {})
            .build()
            .unwrap();

        let parser = self.make_parser(|r| StateAction::DeleteEvent(r));
        self.connector.request(request, RequestDescriptor::new(parser));
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
                StateAction::InsertEvent(_) => {
                    self.load_events();
                },
                StateAction::DeleteEvent(_) => {
                    self.load_events();
                },
                StateAction::None => {
                    println!("none");
                }
                StateAction::Error(status, s) => {
                    println!("smth went wrong: {status:?}=>{s:?}");
                }
            }
        }
    }
}
