use calendar_lib::api::{
    auth::types::NewPassword,
    event_templates::types::{NewEventTemplate, UpdateEventTemplate},
    events::types::{NewEvent, UpdateEvent},
    schedules::types::{NewSchedule, UpdateSchedule},
};
use chrono::NaiveDate;

#[derive(Debug, Clone)]
pub enum AppSignal {
    StateSignal(StateSignal),

    ChangeEvent(i32),
    ChangeEventTemplate(i32),
    ChangeSchedule(i32),

    AddPassword,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum StateSignal {
    ChangeAccessLevel(i32),

    // TODO: Named
    /// (email, password)
    Login(String, String),
    /// (name, email, password)
    Register(String, String, String),

    InsertEvent(NewEvent),
    UpdateEvent(UpdateEvent),
    DeleteEvent(i32),

    InsertEventTemplate(NewEventTemplate),
    UpdateEventTemplate(UpdateEventTemplate),
    DeleteEventTemplate(i32),

    InsertSchedule(NewSchedule),
    UpdateSchedule(UpdateSchedule),
    DeleteSchedule(i32),

    // TODO: Named
    /// (access_level, viewer_password, editor_password)
    InsertPassword(i32, Option<NewPassword>, Option<NewPassword>),
    /// (date, plan_id)
    AcceptScheduledEvent(NaiveDate, i32),
}

impl Into<AppSignal> for StateSignal {
    fn into(self) -> AppSignal {
        AppSignal::StateSignal(self)
    }
}
