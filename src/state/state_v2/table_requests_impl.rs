use calendar_lib::api::{
    auth::types::AccessLevel, event_templates::types::{EventTemplate, NewEventTemplate, UpdateEventTemplate}, events::types::{Event, NewEvent, UpdateEvent},
    schedules::types::{NewSchedule, Schedule, UpdateSchedule},
};

use super::table_requests::{
    TableItemDelete, TableItemInsert, TableItemLoadAll, TableItemLoadById, TableItemUpdate,
};

// TODO: Move to lib

pub trait TableItemApiPath {
    const API_PATH: &'static str;
    const API_PATH_PLURAL: &'static str;
}

impl TableItemApiPath for AccessLevel {
    const API_PATH: &'static str = "access_level";
    const API_PATH_PLURAL: &'static str = "access_levels";
}
impl TableItemApiPath for Event {
    const API_PATH: &'static str = "event";
    const API_PATH_PLURAL: &'static str = "events";
}
impl TableItemApiPath for EventTemplate {
    const API_PATH: &'static str = "event_template";
    const API_PATH_PLURAL: &'static str = "event_templates";
}
impl TableItemApiPath for Schedule {
    const API_PATH: &'static str = "schedule";
    const API_PATH_PLURAL: &'static str = "schedules";
}

impl TableItemLoadById for Event {
    const LOAD_BY_ID_PATH: &'static str = Self::API_PATH;
}
impl TableItemLoadAll for Event {
    const LOAD_ALL_PATH: &'static str = Self::API_PATH_PLURAL;
}
impl TableItemInsert for Event {
    type NewItem = NewEvent;
    const INSERT_PATH: &'static str = Self::API_PATH;
}
impl TableItemUpdate for Event {
    type UpdItem = UpdateEvent;
    const UPDATE_PATH: &'static str = Self::API_PATH;
}
impl TableItemDelete for Event {
    const DELETE_PATH: &'static str = Self::API_PATH;
}

impl TableItemLoadById for EventTemplate {
    const LOAD_BY_ID_PATH: &'static str = Self::API_PATH;
}
impl TableItemLoadAll for EventTemplate {
    const LOAD_ALL_PATH: &'static str = Self::API_PATH_PLURAL;
}
impl TableItemInsert for EventTemplate {
    type NewItem = NewEventTemplate;
    const INSERT_PATH: &'static str = Self::API_PATH;
}
impl TableItemUpdate for EventTemplate {
    type UpdItem = UpdateEventTemplate;
    const UPDATE_PATH: &'static str = Self::API_PATH;
}
impl TableItemDelete for EventTemplate {
    const DELETE_PATH: &'static str = Self::API_PATH;
}

impl TableItemLoadById for Schedule {
    const LOAD_BY_ID_PATH: &'static str = Self::API_PATH;
}
impl TableItemLoadAll for Schedule {
    const LOAD_ALL_PATH: &'static str = Self::API_PATH_PLURAL;
}
impl TableItemInsert for Schedule {
    type NewItem = NewSchedule;
    const INSERT_PATH: &'static str = Self::API_PATH;
}
impl TableItemUpdate for Schedule {
    type UpdItem = UpdateSchedule;
    const UPDATE_PATH: &'static str = Self::API_PATH;
}
impl TableItemDelete for Schedule {
    const DELETE_PATH: &'static str = Self::API_PATH;
}
