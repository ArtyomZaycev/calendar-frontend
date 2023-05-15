use calendar_lib::api::{
    auth::{self, login, new_password, register},
    event_templates, events, schedules, user_roles,
};
use derive_is_enum_variant::is_enum_variant;
use reqwest::StatusCode;

#[derive(Clone, Debug, is_enum_variant)]
pub enum AppRequest {
    Login(login::Response),
    LoginError(login::BadRequestResponse),

    Register(register::Response),
    RegisterError(register::BadRequestResponse),

    NewPassword(new_password::Response),

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

    #[allow(dead_code)]
    None, // for debug only
    Error(StatusCode, String),
}

#[derive(Clone, Debug, Default, is_enum_variant)]
pub enum AppRequestDescription {
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

// TODO: macro
pub trait HasStateAction {
    fn has_login(&self) -> bool;
    fn has_register(&self) -> bool;
    fn has_register_error(&self) -> bool;
    fn has_load_user_roles(&self) -> bool;
    fn has_load_events(&self) -> bool;
    fn has_insert_event(&self) -> bool;
    fn has_update_event(&self) -> bool;
    fn has_delete_events(&self) -> bool;
    fn has_insert_schedule(&self) -> bool;
    fn has_update_schedule(&self) -> bool;
    fn has_insert_event_template(&self) -> bool;
    fn has_none(&self) -> bool;
    fn has_error(&self) -> bool;
}

pub trait GetStateAction {
    fn get_login(&self) -> Option<&login::Response>;
    fn get_login_error(&self) -> Option<&login::BadRequestResponse>;
    fn get_register_error(&self) -> Option<&register::BadRequestResponse>;
    fn get_load_user_roles(&self) -> Option<&user_roles::load_array::Response>;
    fn get_load_events(&self) -> Option<&events::load_array::Response>;
    fn get_insert_event(&self) -> Option<&events::insert::Response>;
    fn get_delete_events(&self) -> Option<&events::delete::Response>;
    fn get_none(&self) -> Option<()>;
    fn get_error(&self) -> Option<(&StatusCode, &String)>;
}

pub trait GetMutStateAction {
    fn get_login_mut(&mut self) -> Option<&mut login::Response>;
    fn get_load_user_roles_mut(&mut self) -> Option<&mut user_roles::load_array::Response>;
    fn get_load_events_mut(&mut self) -> Option<&mut events::load_array::Response>;
    fn get_insert_event_mut(&mut self) -> Option<&mut events::insert::Response>;
    fn get_delete_events_mut(&mut self) -> Option<&mut events::delete::Response>;
    fn get_none_mut(&mut self) -> Option<()>;
    fn get_error_mut(&mut self) -> Option<(&mut StatusCode, &mut String)>;
}

impl HasStateAction for Vec<AppRequest> {
    fn has_login(&self) -> bool {
        self.iter().any(|x| x.is_login())
    }
    fn has_register(&self) -> bool {
        self.iter().any(|x| x.is_register())
    }
    fn has_register_error(&self) -> bool {
        self.iter().any(|x| x.is_register_error())
    }
    fn has_load_user_roles(&self) -> bool {
        self.iter().any(|x| x.is_load_user_roles())
    }
    fn has_load_events(&self) -> bool {
        self.iter().any(|x| x.is_load_events())
    }
    fn has_insert_event(&self) -> bool {
        self.iter().any(|x| x.is_insert_event())
    }
    fn has_update_event(&self) -> bool {
        self.iter().any(|x| x.is_update_event())
    }
    fn has_delete_events(&self) -> bool {
        self.iter().any(|x| x.is_delete_event())
    }
    fn has_insert_schedule(&self) -> bool {
        self.iter().any(|x| x.is_insert_schedule())
    }
    fn has_update_schedule(&self) -> bool {
        self.iter().any(|x| x.is_update_schedule())
    }
    fn has_insert_event_template(&self) -> bool {
        self.iter().any(|x| x.is_insert_event_template())
    }
    fn has_none(&self) -> bool {
        self.iter().any(|x| x.is_none())
    }
    fn has_error(&self) -> bool {
        self.iter().any(|x| x.is_error())
    }
}

impl GetStateAction for Vec<AppRequest> {
    fn get_login(&self) -> Option<&login::Response> {
        self.iter().find_map(|x| {
            if let AppRequest::Login(d) = x {
                Some(d)
            } else {
                None
            }
        })
    }

    fn get_login_error(&self) -> Option<&login::BadRequestResponse> {
        self.iter().find_map(|x| {
            if let AppRequest::LoginError(d) = x {
                Some(d)
            } else {
                None
            }
        })
    }

    fn get_register_error(&self) -> Option<&register::BadRequestResponse> {
        self.iter().find_map(|x| {
            if let AppRequest::RegisterError(d) = x {
                Some(d)
            } else {
                None
            }
        })
    }

    fn get_load_user_roles(&self) -> Option<&user_roles::load_array::Response> {
        self.iter().find_map(|x| {
            if let AppRequest::LoadUserRoles(d) = x {
                Some(d)
            } else {
                None
            }
        })
    }

    fn get_load_events(&self) -> Option<&events::load_array::Response> {
        self.iter().find_map(|x| {
            if let AppRequest::LoadEvents(d) = x {
                Some(d)
            } else {
                None
            }
        })
    }

    fn get_insert_event(&self) -> Option<&events::insert::Response> {
        self.iter().find_map(|x| {
            if let AppRequest::InsertEvent(d) = x {
                Some(d)
            } else {
                None
            }
        })
    }

    fn get_delete_events(&self) -> Option<&events::delete::Response> {
        self.iter().find_map(|x| {
            if let AppRequest::DeleteEvent(d) = x {
                Some(d)
            } else {
                None
            }
        })
    }

    fn get_none(&self) -> Option<()> {
        self.iter().find_map(|x| {
            if let AppRequest::None = x {
                Some(())
            } else {
                None
            }
        })
    }

    fn get_error(&self) -> Option<(&StatusCode, &String)> {
        self.iter().find_map(|x| {
            if let AppRequest::Error(d1, d2) = x {
                Some((d1, d2))
            } else {
                None
            }
        })
    }
}

impl GetMutStateAction for Vec<AppRequest> {
    fn get_login_mut(&mut self) -> Option<&mut login::Response> {
        self.iter_mut().find_map(|x| {
            if let AppRequest::Login(d) = x {
                Some(d)
            } else {
                None
            }
        })
    }

    fn get_load_user_roles_mut(&mut self) -> Option<&mut user_roles::load_array::Response> {
        self.iter_mut().find_map(|x| {
            if let AppRequest::LoadUserRoles(d) = x {
                Some(d)
            } else {
                None
            }
        })
    }

    fn get_load_events_mut(&mut self) -> Option<&mut events::load_array::Response> {
        self.iter_mut().find_map(|x| {
            if let AppRequest::LoadEvents(d) = x {
                Some(d)
            } else {
                None
            }
        })
    }

    fn get_insert_event_mut(&mut self) -> Option<&mut events::insert::Response> {
        self.iter_mut().find_map(|x| {
            if let AppRequest::InsertEvent(d) = x {
                Some(d)
            } else {
                None
            }
        })
    }

    fn get_delete_events_mut(&mut self) -> Option<&mut events::delete::Response> {
        self.iter_mut().find_map(|x| {
            if let AppRequest::DeleteEvent(d) = x {
                Some(d)
            } else {
                None
            }
        })
    }

    fn get_none_mut(&mut self) -> Option<()> {
        self.iter_mut().find_map(|x| {
            if let AppRequest::None = x {
                Some(())
            } else {
                None
            }
        })
    }

    fn get_error_mut(&mut self) -> Option<(&mut StatusCode, &mut String)> {
        self.iter_mut().find_map(|x| {
            if let AppRequest::Error(d1, d2) = x {
                Some((d1, d2))
            } else {
                None
            }
        })
    }
}
