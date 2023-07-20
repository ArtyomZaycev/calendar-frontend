use crate::db::request_parser::FromResponse;
use calendar_lib::api::*;
use derive_is_enum_variant::is_enum_variant;
use reqwest::StatusCode;

#[derive(Clone, Debug, is_enum_variant)]
pub enum AppRequestResponse {
    Login(auth::login::Response),
    LoginError(auth::login::BadRequestResponse),

    LoginByKey(auth::login_by_key::Response),

    Register(auth::register::Response),
    RegisterError(auth::register::BadRequestResponse),

    NewPassword(auth::new_password::Response),

    LoadUserIds(users::load_ids::Response),
    LoadUser(users::load::Response),
    LoadUserError(users::load::BadRequestResponse),
    LoadUsers(users::load_array::Response),

    LoadUserState(user_state::load::Response),
    LoadUserStateError(user_state::load::BadRequestResponse),

    LoadAccessLevels(auth::load_access_levels::Response),
    LoadUserRoles(user_roles::load_array::Response),

    LoadEvent(events::load::Response),
    LoadEventError(events::load::BadRequestResponse),
    LoadEvents(events::load_array::Response),
    InsertEvent(events::insert::Response),
    UpdateEvent(events::update::Response),
    DeleteEvent(events::delete::Response),

    LoadEventTemplate(event_templates::load::Response),
    LoadEventTemplateError(event_templates::load::BadRequestResponse),
    LoadEventTemplates(event_templates::load_array::Response),
    InsertEventTemplate(event_templates::insert::Response),
    UpdateEventTemplate(event_templates::update::Response),
    DeleteEventTemplate(event_templates::delete::Response),

    LoadSchedule(schedules::load::Response),
    LoadScheduleError(schedules::load::BadRequestResponse),
    LoadSchedules(schedules::load_array::Response),
    InsertSchedule(schedules::insert::Response),
    UpdateSchedule(schedules::update::Response),
    DeleteSchedule(schedules::delete::Response),

    /// For debug only
    #[allow(dead_code)]
    None,
    Error(StatusCode, String),
}

/// Lightweight request info
#[derive(Clone, Debug, Default, is_enum_variant)]
pub enum AppRequestInfo {
    LoadUser(i32),
    LoadUserState {
        user_id: i32,
    },
    LoadEvent(i32),
    UpdateEvent(i32),
    //UpdateEvents(Vec<i32>),
    DeleteEvent(i32),
    //DeleteEvents(Vec<i32>),
    LoadEventTemplate(i32),
    UpdateEventTemplate(i32),
    //UpdateEventTemplates(Vec<i32>),
    DeleteEventTemplate(i32),
    //DeleteEventTemplates(Vec<i32>),
    LoadSchedule(i32),
    UpdateSchedule(i32),
    //UpdateSchedules(Vec<i32>),
    DeleteSchedule(i32),
    //DeleteSchedules(Vec<i32>),
    #[allow(dead_code)]
    #[default]
    None,
}

/// Lightweight request response info
#[derive(Clone, Debug, Default, is_enum_variant)]
pub enum AppRequestResponseInfo {
    LoginError(auth::login::BadRequestResponse),
    RegisterError(auth::register::BadRequestResponse),
    LoadUserStateError(user_state::load::BadRequestResponse),
    LoadEventError(events::load::BadRequestResponse),
    LoadEventTemplateError(event_templates::load::BadRequestResponse),
    LoadScheduleError(schedules::load::BadRequestResponse),

    #[allow(dead_code)]
    #[default]
    None,
    Error(StatusCode, String),
}

impl FromResponse<AppRequestResponse> for AppRequestResponseInfo {
    fn from_response(response: &AppRequestResponse) -> Self {
        match response {
            AppRequestResponse::LoginError(r) => Self::LoginError(r.clone()),
            AppRequestResponse::RegisterError(r) => Self::RegisterError(r.clone()),
            AppRequestResponse::LoadUserStateError(r) => Self::LoadUserStateError(r.clone()),
            AppRequestResponse::LoadEventError(r) => Self::LoadEventError(r.clone()),
            AppRequestResponse::LoadEventTemplateError(r) => {
                Self::LoadEventTemplateError(r.clone())
            }
            AppRequestResponse::LoadScheduleError(r) => Self::LoadScheduleError(r.clone()),
            AppRequestResponse::Error(status_code, error) => {
                Self::Error(*status_code, error.clone())
            }
            _ => Self::None,
        }
    }
}
