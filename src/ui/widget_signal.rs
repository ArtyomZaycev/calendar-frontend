use calendar_lib::api::events::types::{NewEvent, UpdateEvent};

#[derive(Debug, Clone)]
pub enum AppSignal {
    StateSignal(StateSignal),

    ChangeEvent(i32),
}

#[derive(Debug, Clone)]
pub enum StateSignal {
    Login(String, String),            // email, password
    Register(String, String, String), // name, email, password
    InsertEvent(NewEvent),
    UpdateEvent(UpdateEvent),
    DeleteEvent(i32),
}

impl Into<AppSignal> for StateSignal {
    fn into(self) -> AppSignal {
        AppSignal::StateSignal(self)
    }
}
