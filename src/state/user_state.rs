use calendar_lib::api::{auth::types::AccessLevel, events::types::*, user_state};

use crate::tables::{table::Table, *};

pub struct UserState {
    pub(super) access_levels: Vec<AccessLevel>,
    pub event_templates: EventTemplates,
    pub events: Table<Event>,
    pub schedules: Schedules,
}

impl Into<UserState> for user_state::load::Response {
    fn into(self) -> UserState {
        UserState {
            access_levels: self.access_levels,
            event_templates: EventTemplates::default(),
            events: Table::from_vec(self.events),
            schedules: Schedules::from(self.schedules),
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
        self.access_levels = vec![];
        self.event_templates.clear();
        self.events.clear();
        self.schedules.clear();
    }
}

impl UserState {
    pub fn get_access_levels(&self) -> &Vec<AccessLevel> {
        &self.access_levels
    }

    pub(super) fn get_access_levels_mut(&mut self) -> &mut Vec<AccessLevel> {
        &mut self.access_levels
    }
}
