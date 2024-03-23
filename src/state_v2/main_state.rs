use std::cell::Ref;
use std::collections::HashMap;
use std::marker::PhantomData;

use calendar_lib::api::auth::types::AccessLevel;
use calendar_lib::api::user_state;
use calendar_lib::api::utils::User;
use calendar_lib::api::{
    event_templates::types::EventTemplate, events::types::Event, schedules::types::Schedule,
};
use serde::de::DeserializeOwned;

use crate::config::Config;
use crate::tables::DbTableItem;

use super::db_connector::{DbConnector, DbConnectorData};
use super::request::RequestId;
use super::requests_holder::{RequestData, RequestsHolder};
use super::state_table::StateTable;

pub trait RequestType {
    const URL: &'static str;
    const IS_AUTHORIZED: bool;
    const METHOD: reqwest::Method;

    type Query;
    type Body = ();
    type Response: DeserializeOwned;
    type BadResponse: DeserializeOwned = ();

    /// e.g. update request item.id
    type Info: Clone = ();

    // TODO: Separate into different trait, move struct to request.rs
    fn push_to_state(response: Self::Response, info: Self::Info, state: &mut State);
    #[allow(unused_variables)]
    fn push_bad_to_state(response: Self::BadResponse, info: Self::Info, state: &mut State) {}
}

pub struct RequestIdentifier<T: RequestType> {
    id: RequestId,
    info: T::Info,
    _data: PhantomData<T>,
}

impl<T: RequestType> RequestIdentifier<T> {
    pub(super) fn new(request_id: RequestId, info: T::Info) -> Self {
        Self {
            id: request_id,
            info,
            _data: PhantomData::default(),
        }
    }
}

pub struct UserState {
    pub access_levels: StateTable<AccessLevel>,
    pub events: StateTable<Event>,
    pub event_templates: StateTable<EventTemplate>,
    pub schedules: StateTable<Schedule>,
    pub(super) requests: RequestsHolder,
}

impl UserState {
    fn new() -> Self {
        Self {
            access_levels: StateTable::new(),
            events: StateTable::new(),
            schedules: StateTable::new(),
            event_templates: StateTable::new(),
            requests: RequestsHolder::new(),
        }
    }
}

impl From<user_state::load::Response> for UserState {
    fn from(value: user_state::load::Response) -> Self {
        Self {
            access_levels: StateTable::from_vec(value.access_levels),
            events: StateTable::from_vec(value.events),
            schedules: StateTable::from_vec(value.schedules),
            event_templates: StateTable::from_vec(value.event_templates),
            requests: RequestsHolder::new(),
        }
    }
}

pub struct AdminState {
    pub users: StateTable<User>,
    pub users_data: HashMap<i32, UserState>,
    pub(super) requests: RequestsHolder,
}

impl AdminState {
    fn new() -> Self {
        Self {
            users: StateTable::new(),
            users_data: HashMap::default(),
            requests: RequestsHolder::new(),
        }
    }
}

pub struct State {
    db_connector: DbConnector,
    pub(super) requests: RequestsHolder,

    pub(super) me: User,
    pub(super) current_access_level: i32,

    pub user_state: UserState,
    pub admin_state: AdminState,
}

impl State {
    pub fn new(config: &Config) -> Self {
        State {
            db_connector: DbConnector::new(config),
            requests: RequestsHolder::new(),
            me: User::default(),
            current_access_level: -1,
            user_state: UserState::new(),
            admin_state: AdminState::new(),
        }
    }

    // TODO
    //pub fn is_failed(&self) -> bool;

    // TODO: Option<reqwest::Error<T::Response>>, to find out about failder requests
    pub fn get_response<'a, T: RequestType + 'static>(
        &'a self,
        identifier: RequestIdentifier<T>,
    ) -> Option<Result<Ref<'a, T::Response>, Ref<'a, T::BadResponse>>> {
        self.db_connector
            .convert_response::<T::Response, T::BadResponse>(identifier.id);
        self.db_connector
            .get_response::<T::Response, T::BadResponse>(identifier.id)
            .and_then(|r| r.ok())
    }

    pub fn take_response<T: RequestType + 'static>(
        &mut self,
        identifier: RequestIdentifier<T>,
    ) -> Option<Result<Box<T::Response>, Box<T::BadResponse>>> {
        self.db_connector
            .convert_response::<T::Response, T::BadResponse>(identifier.id);
        self.db_connector
            .take_response::<T::Response, T::BadResponse>(identifier.id)
            .and_then(|r| r.ok())
    }

    fn send_requests(&mut self) {
        let mut requests = self.requests.take();
        requests.extend(self.user_state.access_levels.requests.take());
        requests.extend(self.user_state.events.requests.take());
        requests.extend(self.user_state.event_templates.requests.take());
        requests.extend(self.user_state.schedules.requests.take());

        requests.into_iter().for_each(|request| {
            self.db_connector.request(request);
        });
    }

    pub fn update(&mut self) {
        self.db_connector.pull();
        self.send_requests();
    }
}

pub trait GetStateTable<T: DbTableItem> {
    fn get_table(&self) -> &StateTable<T>;
    fn get_table_mut(&mut self) -> &mut StateTable<T>;
}

impl GetStateTable<AccessLevel> for State {
    fn get_table(&self) -> &StateTable<AccessLevel> {
        &self.user_state.access_levels
    }

    fn get_table_mut(&mut self) -> &mut StateTable<AccessLevel> {
        &mut self.user_state.access_levels
    }
}

impl GetStateTable<Event> for State {
    fn get_table(&self) -> &StateTable<Event> {
        &self.user_state.events
    }

    fn get_table_mut(&mut self) -> &mut StateTable<Event> {
        &mut self.user_state.events
    }
}

impl GetStateTable<Schedule> for State {
    fn get_table(&self) -> &StateTable<Schedule> {
        &self.user_state.schedules
    }

    fn get_table_mut(&mut self) -> &mut StateTable<Schedule> {
        &mut self.user_state.schedules
    }
}

impl GetStateTable<EventTemplate> for State {
    fn get_table(&self) -> &StateTable<EventTemplate> {
        &self.user_state.event_templates
    }

    fn get_table_mut(&mut self) -> &mut StateTable<EventTemplate> {
        &mut self.user_state.event_templates
    }
}
