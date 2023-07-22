use calendar_lib::api::{
    auth::types::AccessLevel, event_templates::types::*, events::types::*, schedules::types::*,
    user_state,
};

use crate::db::table::table::Table;

pub struct UserState {
    pub access_levels: Table<AccessLevel>,
    pub event_templates: Table<EventTemplate>,
    pub events: Table<Event>,
    pub schedules: Table<Schedule>,
}

impl Into<UserState> for user_state::load::Response {
    fn into(self) -> UserState {
        UserState {
            access_levels: Table::from_vec(self.access_levels),
            event_templates: Table::from_vec(self.event_templates),
            events: Table::from_vec(self.events),
            schedules: Table::from_vec(self.schedules),
        }
    }
}

impl UserState {
    pub fn new() -> Self {
        Self {
            access_levels: Default::default(),
            event_templates: Default::default(),
            events: Default::default(),
            schedules: Default::default(),
        }
    }

    pub fn clear(&mut self) {
        self.access_levels.clear();
        self.event_templates.clear();
        self.events.clear();
        self.schedules.clear();
    }
}
