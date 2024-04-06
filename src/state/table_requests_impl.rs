use calendar_lib::api::{
    auth::types::AccessLevel,
    event_templates::{
        self,
        types::{EventTemplate, NewEventTemplate, UpdateEventTemplate},
    },
    events::{
        self,
        types::{Event, NewEvent, UpdateEvent},
    },
    roles::{self, types::Role},
    schedules::{
        self,
        types::{NewSchedule, Schedule, UpdateSchedule},
    },
    users,
    utils::User,
};

use super::table_requests::{
    TableItemDelete, TableItemInsert, TableItemLoadAll, TableItemLoadById, TableItemUpdate,
};

impl TableItemLoadAll for AccessLevel {
    const LOAD_ALL_PATH: &'static str = "auth/load_access_levels";
}

impl TableItemLoadById for Event {
    const LOAD_BY_ID_PATH: &'static str = events::load::PATH;
}
impl TableItemLoadAll for Event {
    const LOAD_ALL_PATH: &'static str = events::load_array::PATH;
}
impl TableItemInsert for Event {
    type NewItem = NewEvent;
    const INSERT_PATH: &'static str = events::insert::PATH;
}
impl TableItemUpdate for Event {
    type UpdItem = UpdateEvent;
    const UPDATE_PATH: &'static str = events::update::PATH;
}
impl TableItemDelete for Event {
    const DELETE_PATH: &'static str = events::delete::PATH;
}

impl TableItemLoadById for EventTemplate {
    const LOAD_BY_ID_PATH: &'static str = event_templates::load::PATH;
}
impl TableItemLoadAll for EventTemplate {
    const LOAD_ALL_PATH: &'static str = event_templates::load_array::PATH;
}
impl TableItemInsert for EventTemplate {
    type NewItem = NewEventTemplate;
    const INSERT_PATH: &'static str = event_templates::insert::PATH;
}
impl TableItemUpdate for EventTemplate {
    type UpdItem = UpdateEventTemplate;
    const UPDATE_PATH: &'static str = event_templates::update::PATH;
}
impl TableItemDelete for EventTemplate {
    const DELETE_PATH: &'static str = event_templates::delete::PATH;
}

impl TableItemLoadById for Schedule {
    const LOAD_BY_ID_PATH: &'static str = schedules::load::PATH;
}
impl TableItemLoadAll for Schedule {
    const LOAD_ALL_PATH: &'static str = schedules::load_array::PATH;
}
impl TableItemInsert for Schedule {
    type NewItem = NewSchedule;
    const INSERT_PATH: &'static str = schedules::insert::PATH;
}
impl TableItemUpdate for Schedule {
    type UpdItem = UpdateSchedule;
    const UPDATE_PATH: &'static str = schedules::update::PATH;
}
impl TableItemDelete for Schedule {
    const DELETE_PATH: &'static str = schedules::delete::PATH;
}

impl TableItemLoadAll for Role {
    const LOAD_ALL_PATH: &'static str = roles::load_array::PATH;
}

impl TableItemLoadAll for User {
    const LOAD_ALL_PATH: &'static str = users::load_array::PATH;
}
