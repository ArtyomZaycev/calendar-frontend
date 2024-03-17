use std::cell::Ref;
use std::marker::PhantomData;

use calendar_lib::api::{
    event_templates::types::EventTemplate, events::types::Event, schedules::types::Schedule,
};
use serde::de::DeserializeOwned;

use crate::config::Config;

use super::db_connector::DbConnector;
use super::request::RequestId;
use super::requests_holder::RequestsHolder;
use super::state_table::StateTable;

pub trait RequestType {
    const URL: String;
    const IS_AUTHORIZED: bool;
    const METHOD: reqwest::Method;

    type Query;
    type Body = ();
    type Response: DeserializeOwned;

    // TODO
    //type BadResponse: DeserializeOwned = ();

    /// e.g. update request item.id
    type Info: Clone = ();

    fn push_to_state(response: Self::Response, info: Self::Info, state: &mut State);
}

pub struct RequestIdentifier<T: RequestType> {
    id: RequestId,
    info: T::Info,
    _data: PhantomData<T>,
}

pub struct State {
    db_connector: DbConnector,
    pub events: StateTable<Event>,
    pub schedules: StateTable<Schedule>,
    pub event_templates: StateTable<EventTemplate>,
    requests: RequestsHolder,
}

impl State {
    pub fn new(config: &Config) -> Self {
        State {
            db_connector: DbConnector::new(config),
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
    ) -> Option<Ref<'a, T::Response>> {
        self.db_connector
            .convert_response::<T::Response>(identifier.id);
        self.db_connector.get_response::<T::Response>(identifier.id)
    }

    pub fn take_response<T: RequestType + 'static>(
        &mut self,
        identifier: RequestIdentifier<T>,
    ) -> Option<Box<T::Response>> {
        self.db_connector
            .convert_response::<T::Response>(identifier.id);
        self.db_connector
            .take_response::<T::Response>(identifier.id)
    }
}
