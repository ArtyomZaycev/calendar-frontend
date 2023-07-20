use calendar_lib::api::{
    auth::types::AccessLevel, event_templates::types::EventTemplate, events::types::Event,
    schedules::types::Schedule, user_state,
};

pub struct UserState {
    pub(super) access_levels: Vec<AccessLevel>,
    pub(super) event_templates: Vec<EventTemplate>,
    pub(super) events: Vec<Event>,
    pub(super) schedules: Vec<Schedule>,
}

impl Into<UserState> for user_state::load::Response {
    fn into(self) -> UserState {
        UserState {
            access_levels: self.access_levels,
            event_templates: self.event_templates,
            events: self.events,
            schedules: self.schedules,
        }
    }
}

impl UserState {
    pub fn new() -> Self {
        Self {
            access_levels: Vec::default(),
            event_templates: Vec::default(),
            events: Vec::default(),
            schedules: Vec::default(),
        }
    }

    pub fn clear(&mut self) {
        self.access_levels = vec![];
        self.event_templates = vec![];
        self.events = vec![];
        self.schedules = vec![];
    }
}

impl UserState {
    pub fn get_access_levels(&self) -> &Vec<AccessLevel> {
        &self.access_levels
    }
    pub fn get_event_templates(&self) -> &Vec<EventTemplate> {
        &self.event_templates
    }
    pub fn get_events(&self) -> &Vec<Event> {
        &self.events
    }
    pub fn get_schedules(&self) -> &Vec<Schedule> {
        &self.schedules
    }

    pub(super) fn get_access_levels_mut(&mut self) -> &mut Vec<AccessLevel> {
        &mut self.access_levels
    }
    pub(super) fn get_event_templates_mut(&mut self) -> &mut Vec<EventTemplate> {
        &mut self.event_templates
    }
    pub(super) fn get_events_mut(&mut self) -> &mut Vec<Event> {
        &mut self.events
    }
    pub(super) fn get_schedules_mut(&mut self) -> &mut Vec<Schedule> {
        &mut self.schedules
    }
}
