use calendar_lib::api::{auth, events, user_roles};
use reqwest::{Method, RequestBuilder, StatusCode};
use serde::de::DeserializeOwned;

use crate::{config::Config, ui::widget_signal::StateSignal};

use super::{
    aliases::*,
    connector::{Connector, RequestDescriptor},
    request_parser::RequestParser,
    state_action::StateAction,
};

pub struct State {
    connector: Connector<StateAction>,

    // Should not be modified manually, use requests
    pub me: Option<UserInfo>,
    pub users: Vec<User>,
    pub events: Vec<Event>,

    pub errors: Vec<()>,
}

impl State {
    pub fn new(config: &Config) -> Self {
        Self {
            connector: Connector::new(config),
            me: None,
            users: Vec::default(),
            events: Vec::default(),
            errors: Vec::default(),
        }
    }
}

impl State {
    pub fn parse_signal(&mut self, signal: StateSignal) {
        match signal {
            StateSignal::Login(email, password) => self.login(&email, &password),
            StateSignal::Register(name, email, password) => self.register(&name, &email, &password),
            StateSignal::InsertEvent(new_event) => self.insert_event(new_event),
            StateSignal::UpdateEvent(upd_event) => self.update_event(upd_event),
            StateSignal::DeleteEvent(id) => self.delete_event(id),
        }
    }
}

impl State {
    pub fn make_request(&self, method: Method, op: &str) -> RequestBuilder {
        self.connector.make_request(method, op)
    }

    pub fn make_request_authorized(&self, method: Method, op: &str) -> RequestBuilder {
        if let Some(me) = &self.me {
            self.connector.make_request(method, op).basic_auth(
                me.user.id,
                Some(std::str::from_utf8(&me.key).expect("parse error")),
            )
        } else {
            todo!()
        }
    }

    // Use for testing only
    #[cfg(debug_assertions)]
    #[allow(dead_code)]
    fn make_empty_parser() -> RequestParser<StateAction> {
        RequestParser::new_split(|_| StateAction::None, |_, _| StateAction::None)
    }

    fn make_parser<U, F>(on_success: F) -> RequestParser<StateAction>
    where
        U: DeserializeOwned,
        F: FnOnce(U) -> StateAction + 'static,
    {
        RequestParser::new_complex(on_success, |code, s| StateAction::Error(code, s))
    }

    fn make_bad_request_parser<T, F1, F2>(
        on_success: F1,
        on_bad_request: F2,
    ) -> RequestParser<StateAction>
    where
        T: DeserializeOwned,
        F1: FnOnce(T) -> StateAction + 'static,
        F2: FnOnce(String) -> StateAction + 'static,
    {
        RequestParser::new_complex(on_success, |code, msg| {
            if code == StatusCode::BAD_REQUEST {
                on_bad_request(msg)
            } else {
                StateAction::Error(code, msg)
            }
        })
    }

    fn make_typed_bad_request_parser<T, U, F1, F2>(
        on_success: F1,
        on_bad_request: F2,
    ) -> RequestParser<StateAction>
    where
        T: DeserializeOwned,
        U: DeserializeOwned,
        F1: FnOnce(T) -> StateAction + 'static,
        F2: FnOnce(U) -> StateAction + 'static,
    {
        RequestParser::new_complex(on_success, |code, msg| {
            if code == StatusCode::BAD_REQUEST {
                on_bad_request(serde_json::from_str(&msg).unwrap())
            } else {
                StateAction::Error(code, msg)
            }
        })
    }
}

impl State {
    pub fn load_access_levels(&self) {
        use auth::load_access_levels::*;

        let request = self
            .make_request_authorized(METHOD.clone(), PATH)
            .query(&Args {})
            .build()
            .unwrap();

        let parser = Self::make_parser(|r| StateAction::LoadAccessLevels(r));
        self.connector
            .request(request, RequestDescriptor::new(parser));
    }

    pub fn load_user_roles(&self) {
        use user_roles::load_array::*;

        let request = self
            .make_request_authorized(METHOD.clone(), PATH)
            .query(&Args { user_id: None })
            .build()
            .unwrap();

        let parser = Self::make_parser(|r| StateAction::LoadUserRoles(r));
        self.connector
            .request(request, RequestDescriptor::new(parser));
    }

    pub fn logout(&mut self) {
        use auth::logout::*;

        let request = self
            .make_request_authorized(METHOD.clone(), PATH)
            .query(&Args {})
            .json(&Body {})
            .build()
            .unwrap();

        self.me = None;
        self.users = vec![];
        self.events = vec![];

        let parser = Self::make_empty_parser();
        self.connector
            .request(request, RequestDescriptor::new(parser));
    }

    pub fn login(&self, email: &str, password: &str) {
        use auth::login::*;

        let request = self
            .make_request(METHOD.clone(), PATH)
            .query(&Args {})
            .json(&Body {
                email: email.to_owned(),
                password: password.to_owned(),
            })
            .build()
            .unwrap();

        let parser = Self::make_parser(|r| StateAction::Login(r));
        self.connector
            .request(request, RequestDescriptor::new(parser));
    }

    pub fn register(&self, name: &str, email: &str, password: &str) {
        use auth::register::*;

        let request = self
            .make_request(METHOD.clone(), PATH)
            .query(&Args {})
            .json(&Body {
                name: name.to_owned(),
                email: email.to_owned(),
                password: password.to_owned(),
            })
            .build()
            .unwrap();

        let parser = Self::make_typed_bad_request_parser(
            |r| StateAction::Register(r),
            |r| StateAction::RegisterError(r),
        );
        self.connector
            .request(request, RequestDescriptor::new(parser));
    }

    pub fn load_events(&self) {
        use events::load_array::*;

        let request = self
            .make_request_authorized(METHOD.clone(), PATH)
            .query(&Args {})
            .build()
            .unwrap();

        let parser = Self::make_parser(|r| StateAction::LoadEvents(r));
        self.connector
            .request(request, RequestDescriptor::new(parser));
    }

    pub fn insert_event(&self, new_event: NewEvent) {
        use events::insert::*;

        let request = self
            .make_request_authorized(METHOD.clone(), PATH)
            .query(&Args {})
            .json(&Body { new_event })
            .build()
            .unwrap();

        let parser = Self::make_parser(|r| StateAction::InsertEvent(r));
        self.connector
            .request(request, RequestDescriptor::new(parser));
    }

    pub fn update_event(&self, upd_event: UpdateEvent) {
        use events::update::*;

        let request = self
            .make_request_authorized(METHOD.clone(), PATH)
            .query(&Args {})
            .json(&Body { upd_event })
            .build()
            .unwrap();

        let parser = Self::make_parser(|r| StateAction::UpdateEvent(r));
        self.connector
            .request(request, RequestDescriptor::new(parser));
    }

    pub fn delete_event(&self, id: i32) {
        use events::delete::*;

        let request = self
            .make_request_authorized(METHOD.clone(), PATH)
            .query(&Args { id })
            .json(&Body {})
            .build()
            .unwrap();

        let parser = Self::make_parser(|r| StateAction::DeleteEvent(r));
        self.connector
            .request(request, RequestDescriptor::new(parser));
    }
}

impl State {
    fn parse_action(&mut self, action: StateAction) {
        match action {
            StateAction::Login(res) => {
                self.me = Some(UserInfo {
                    user: res.user,
                    current_access_level: res.access_level.level,
                    access_levels: vec![res.access_level],
                    key: res.key,
                    roles: vec![],
                });
                self.load_access_levels();
                self.load_events();
                self.load_user_roles();
            }
            StateAction::Register(_) => {}
            StateAction::RegisterError(_) => {}
            StateAction::LoadAccessLevels(mut r) => {
                if let Some(me) = &mut self.me {
                    r.array.sort_by(|a, b| a.level.cmp(&b.level));
                    me.access_levels = r.array;
                }
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
            }
            StateAction::UpdateEvent(_) => {
                self.load_events();
            }
            StateAction::DeleteEvent(_) => {
                self.load_events();
            }
            StateAction::None => {}
            StateAction::Error(status, s) => {
                println!("smth went wrong: {status:?}=>{s:?}");
            }
        }
    }

    pub fn poll(&mut self) -> Vec<StateAction> {
        let actions = self.connector.poll();
        actions
            .clone()
            .into_iter()
            .for_each(|a| self.parse_action(a));
        actions
    }

    pub fn get_active_requests_descriptions(&self) -> Vec<()> {
        self.connector.get_active_requests_descriptions()
    }
}
