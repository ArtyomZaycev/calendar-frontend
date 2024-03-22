use std::cell::Ref;
use std::marker::PhantomData;

use calendar_lib::api::auth::types::AccessLevel;
use calendar_lib::api::{
    event_templates::types::EventTemplate, events::types::Event, schedules::types::Schedule,
};
use serde::de::DeserializeOwned;

use crate::config::Config;
use crate::tables::DbTableItem;

use super::db_connector::DbConnector;
use super::request::RequestId;
use super::requests_holder::RequestsHolder;
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

pub struct State {
    db_connector: DbConnector,
    pub access_levels: StateTable<AccessLevel>,
    pub events: StateTable<Event>,
    pub event_templates: StateTable<EventTemplate>,
    pub schedules: StateTable<Schedule>,
    requests: RequestsHolder,
}

impl State {
    pub fn new(config: &Config) -> Self {
        State {
            db_connector: DbConnector::new(config),
            access_levels: StateTable::new(),
            events: StateTable::new(),
            schedules: StateTable::new(),
            event_templates: StateTable::new(),
            requests: RequestsHolder::new(),
        }
    }

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
}

pub trait GetStateTable<T: DbTableItem> {
    fn get_table(&self) -> &StateTable<T>;
    fn get_table_mut(&mut self) -> &mut StateTable<T>;
}

impl GetStateTable<AccessLevel> for State {
    fn get_table(&self) -> &StateTable<AccessLevel> {
        &self.access_levels
    }

    fn get_table_mut(&mut self) -> &mut StateTable<AccessLevel> {
        &mut self.access_levels
    }
}

impl GetStateTable<Event> for State {
    fn get_table(&self) -> &StateTable<Event> {
        &self.events
    }

    fn get_table_mut(&mut self) -> &mut StateTable<Event> {
        &mut self.events
    }
}

impl GetStateTable<Schedule> for State {
    fn get_table(&self) -> &StateTable<Schedule> {
        &self.schedules
    }

    fn get_table_mut(&mut self) -> &mut StateTable<Schedule> {
        &mut self.schedules
    }
}

impl GetStateTable<EventTemplate> for State {
    fn get_table(&self) -> &StateTable<EventTemplate> {
        &self.event_templates
    }

    fn get_table_mut(&mut self) -> &mut StateTable<EventTemplate> {
        &mut self.event_templates
    }
}
