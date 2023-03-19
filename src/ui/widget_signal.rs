use calendar_lib::api::{
    auth::types::NewPassword,
    event_templates::types::NewEventTemplate,
    events::types::{NewEvent, UpdateEvent},
    schedules::types::{NewSchedule, UpdateSchedule},
};

#[derive(Debug, Clone)]
pub enum AppSignal {
    StateSignal(StateSignal),

    ChangeEvent(i32),
    ChangeSchedule(i32),

    AddPassword,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum StateSignal {
    ChangeAccessLevel(i32),

    /// (email, password)
    Login(String, String),
    /// (name, email, password)
    Register(String, String, String),

    InsertEvent(NewEvent),
    UpdateEvent(UpdateEvent),
    DeleteEvent(i32),

    InsertEventTemplate(NewEventTemplate),
    DeleteEventTemplate(i32),

    InsertSchedule(NewSchedule),
    UpdateSchedule(UpdateSchedule),
    DeleteSchedule(i32),

    InsertPassword(i32, Option<NewPassword>, Option<NewPassword>),
}

impl Into<AppSignal> for StateSignal {
    fn into(self) -> AppSignal {
        AppSignal::StateSignal(self)
    }
}
