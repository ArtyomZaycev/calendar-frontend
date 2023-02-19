use calendar_lib::api::{auth::{login, register}, events, user_roles};
use derive_is_enum_variant::is_enum_variant;
use reqwest::StatusCode;

#[derive(Clone, Debug, is_enum_variant)]
pub enum StateAction {
    Login(login::Response),
    Register(register::Response),
    LoadUserRoles(user_roles::load_array::Response),
    LoadEvents(events::load_array::Response),
    InsertEvent(events::insert::Response),
    UpdateEvent(events::update::Response),
    DeleteEvent(events::delete::Response),

    #[allow(dead_code)]
    None, // for debug only
    Error(StatusCode, String),
}

// TODO: macro
pub trait HasStateAction {
    fn has_login(&self) -> bool;
    fn has_register(&self) -> bool;
    fn has_load_user_roles(&self) -> bool;
    fn has_load_events(&self) -> bool;
    fn has_insert_event(&self) -> bool;
    fn has_update_event(&self) -> bool;
    fn has_delete_events(&self) -> bool;
    fn has_none(&self) -> bool;
    fn has_error(&self) -> bool;
}

pub trait GetStateAction {
    fn get_login(&self) -> Option<&login::Response>;
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

impl HasStateAction for Vec<StateAction> {
    fn has_login(&self) -> bool {
        self.iter().any(|x| x.is_login())
    }
    fn has_register(&self) -> bool {
        self.iter().any(|x| x.is_register())
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
    fn has_none(&self) -> bool {
        self.iter().any(|x| x.is_none())
    }
    fn has_error(&self) -> bool {
        self.iter().any(|x| x.is_error())
    }
}

impl GetStateAction for Vec<StateAction> {
    fn get_login(&self) -> Option<&login::Response> {
        self.iter().find_map(|x| {
            if let StateAction::Login(d) = x {
                Some(d)
            } else {
                None
            }
        })
    }

    fn get_load_user_roles(&self) -> Option<&user_roles::load_array::Response> {
        self.iter().find_map(|x| {
            if let StateAction::LoadUserRoles(d) = x {
                Some(d)
            } else {
                None
            }
        })
    }

    fn get_load_events(&self) -> Option<&events::load_array::Response> {
        self.iter().find_map(|x| {
            if let StateAction::LoadEvents(d) = x {
                Some(d)
            } else {
                None
            }
        })
    }

    fn get_insert_event(&self) -> Option<&events::insert::Response> {
        self.iter().find_map(|x| {
            if let StateAction::InsertEvent(d) = x {
                Some(d)
            } else {
                None
            }
        })
    }

    fn get_delete_events(&self) -> Option<&events::delete::Response> {
        self.iter().find_map(|x| {
            if let StateAction::DeleteEvent(d) = x {
                Some(d)
            } else {
                None
            }
        })
    }

    fn get_none(&self) -> Option<()> {
        self.iter().find_map(|x| {
            if let StateAction::None = x {
                Some(())
            } else {
                None
            }
        })
    }

    fn get_error(&self) -> Option<(&StatusCode, &String)> {
        self.iter().find_map(|x| {
            if let StateAction::Error(d1, d2) = x {
                Some((d1, d2))
            } else {
                None
            }
        })
    }
}

impl GetMutStateAction for Vec<StateAction> {
    fn get_login_mut(&mut self) -> Option<&mut login::Response> {
        self.iter_mut().find_map(|x| {
            if let StateAction::Login(d) = x {
                Some(d)
            } else {
                None
            }
        })
    }

    fn get_load_user_roles_mut(&mut self) -> Option<&mut user_roles::load_array::Response> {
        self.iter_mut().find_map(|x| {
            if let StateAction::LoadUserRoles(d) = x {
                Some(d)
            } else {
                None
            }
        })
    }

    fn get_load_events_mut(&mut self) -> Option<&mut events::load_array::Response> {
        self.iter_mut().find_map(|x| {
            if let StateAction::LoadEvents(d) = x {
                Some(d)
            } else {
                None
            }
        })
    }

    fn get_insert_event_mut(&mut self) -> Option<&mut events::insert::Response> {
        self.iter_mut().find_map(|x| {
            if let StateAction::InsertEvent(d) = x {
                Some(d)
            } else {
                None
            }
        })
    }

    fn get_delete_events_mut(&mut self) -> Option<&mut events::delete::Response> {
        self.iter_mut().find_map(|x| {
            if let StateAction::DeleteEvent(d) = x {
                Some(d)
            } else {
                None
            }
        })
    }

    fn get_none_mut(&mut self) -> Option<()> {
        self.iter_mut().find_map(|x| {
            if let StateAction::None = x {
                Some(())
            } else {
                None
            }
        })
    }

    fn get_error_mut(&mut self) -> Option<(&mut StatusCode, &mut String)> {
        self.iter_mut().find_map(|x| {
            if let StateAction::Error(d1, d2) = x {
                Some((d1, d2))
            } else {
                None
            }
        })
    }
}
