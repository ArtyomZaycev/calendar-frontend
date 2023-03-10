use calendar_lib::api::{
    event_templates::types::NewEventTemplate,
    events::types::{NewEvent, UpdateEvent},
    schedules::types::{NewSchedule, UpdateSchedule},
};

#[derive(Debug, Clone)]
pub enum AppSignal {
    StateSignal(StateSignal),

    ChangeEvent(i32),
    ChangeSchedule(i32),
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum StateSignal {
    Login(String, String),            // email, password
    Register(String, String, String), // name, email, password

    InsertEvent(NewEvent),
    UpdateEvent(UpdateEvent),
    DeleteEvent(i32),

    InsertEventTemplate(NewEventTemplate),
    DeleteEventTemplate(i32),

    InsertSchedule(NewSchedule),
    UpdateSchedule(UpdateSchedule),
    DeleteSchedule(i32),
}

impl Into<AppSignal> for StateSignal {
    fn into(self) -> AppSignal {
        AppSignal::StateSignal(self)
    }
}
