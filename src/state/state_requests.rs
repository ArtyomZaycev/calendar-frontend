use crate::requests::AppRequestResponse;
use crate::tables::utils::*;
use crate::{
    db::{aliases::*, request::*, request_parser::*, table::*},
    requests::AppRequestInfo,
    state::*,
};

use calendar_lib::api::{
    self,
    auth::{self, types::NewPassword},
    user_roles,
};
use serde::Serialize;

impl State {
    fn request<Q: Serialize, B: Serialize>(
        &self,
        request: RequestBuilder<Q, B, StateCallback>,
        description: RequestDescription,
    ) -> RequestId {
        let jwt = self.get_me().map(|u| u.jwt.clone()).unwrap_or_default();
        match self.connector.request2(request, &jwt, description) {
            Ok(id) => id,
            Err(err) => {
                println!("Error sending the request: {err:?}");
                RequestId::default()
            }
        }
    }
}

pub type StateCallback = Box<dyn FnOnce(&mut State, AppRequestResponse)>;

impl State {
    pub fn change_access_level(&mut self, new_access_level: i32) {
        self.current_access_level = new_access_level;
        self.clear_events();
    }

    pub fn load_access_levels(
        &self,
        description: RequestDescription,
        callback: Option<StateCallback>,
    ) -> RequestId {
        use auth::load_access_levels::*;

        let request = self
            .make_request_authorized(METHOD.clone(), PATH)
            .query(&Args {})
            .build()
            .unwrap();

        let parser = make_parser(|r| AppRequestResponse::LoadAccessLevels(r));
        self.connector
            .request(request, parser, AppRequestInfo::None, description, callback)
    }

    pub fn load_user_roles(
        &self,
        description: RequestDescription,
        callback: Option<StateCallback>,
    ) -> RequestId {
        use user_roles::load_array::*;

        let request = self
            .make_request_authorized(METHOD.clone(), PATH)
            .query(&Args { user_id: None })
            .build()
            .unwrap();

        let parser = make_parser(|r| AppRequestResponse::LoadUserRoles(r));
        self.connector
            .request(request, parser, AppRequestInfo::None, description, callback)
    }

    pub fn logout(
        &mut self,
        description: RequestDescription,
        callback: Option<StateCallback>,
    ) -> RequestId {
        use auth::logout::*;

        let request = self
            .make_request_authorized(METHOD.clone(), PATH)
            .query(&Args {})
            .json(&Body {})
            .build()
            .unwrap();

        self.clear_state();

        let parser = RequestParser::new_split(
            |_| AppRequestResponse::None,
            |code, _| AppRequestResponse::Error(code, "Logout error".to_owned()),
        );
        self.connector
            .request(request, parser, AppRequestInfo::None, description, callback)
    }

    pub fn login_by_jwt(
        &self,
        jwt: &str,
        description: RequestDescription,
        callback: Option<StateCallback>,
    ) -> RequestId {
        use auth::login_by_key::*;

        let request = self
            .make_request(METHOD.clone(), PATH)
            .bearer_auth(jwt)
            .query(&Args {})
            .json(&Body {})
            .build()
            .unwrap();

        let parser = make_parser(|r| AppRequestResponse::LoginByKey(r));
        self.connector
            .request(request, parser, AppRequestInfo::None, description, callback)
    }

    pub fn login(
        &self,
        email: &str,
        password: &str,
        description: RequestDescription,
        callback: Option<StateCallback>,
    ) -> RequestId {
        use auth::login::*;

        // Always save login data for persistency
        let description = description.save_results();

        let request = self
            .make_request(METHOD.clone(), PATH)
            .query(&Args {})
            .json(&Body {
                email: email.to_owned(),
                password: password.to_owned(),
            })
            .build()
            .unwrap();

        let parser = make_typed_bad_request_parser(
            |r| AppRequestResponse::Login(r),
            |r| AppRequestResponse::LoginError(r),
        );
        self.connector
            .request(request, parser, AppRequestInfo::None, description, callback)
    }

    pub fn register(
        &self,
        name: &str,
        email: &str,
        password: &str,
        description: RequestDescription,
        callback: Option<StateCallback>,
    ) -> RequestId {
        use auth::register::*;

        let request = self
            .make_request(METHOD.clone(), PATH)
            .query(&Args {})
            .json(&Body {
                name: name.to_owned(),
                email: email.to_owned(),
                password: password.to_owned(),
            })
            .build()
            .unwrap();

        let parser = make_typed_bad_request_parser(
            |r| AppRequestResponse::Register(r),
            |r| AppRequestResponse::RegisterError(r),
        );
        self.connector
            .request(request, parser, AppRequestInfo::None, description, callback)
    }

    pub fn new_password(
        &self,
        access_level: i32,
        viewer: Option<NewPassword>,
        editor: Option<NewPassword>,
        description: RequestDescription,
        callback: Option<StateCallback>,
    ) -> RequestId {
        use auth::new_password::*;

        let request = self
            .make_request_authorized(METHOD.clone(), PATH)
            .query(&Args {})
            .json(&Body {
                user_id: self.me.as_ref().unwrap().user.id,
                access_level,
                viewer_password: viewer,
                editor_password: editor,
            })
            .build()
            .unwrap();

        let parser = make_parser(|r| AppRequestResponse::NewPassword(r));
        self.connector
            .request(request, parser, AppRequestInfo::None, description, callback)
    }

    pub fn load_user_ids(
        &self,
        description: RequestDescription,
        callback: Option<StateCallback>,
    ) -> RequestId {
        use users::load_ids::*;

        let request = self
            .make_request_authorized(METHOD.clone(), PATH)
            .query(&Args {})
            .build()
            .unwrap();

        let parser = make_parser(|r| AppRequestResponse::LoadUserIds(r));
        self.connector
            .request(request, parser, AppRequestInfo::None, description, callback)
    }

    pub fn load_user(
        &self,
        id: i32,
        description: RequestDescription,
        callback: Option<StateCallback>,
    ) -> RequestId {
        use users::load::*;

        let request = self
            .make_request_authorized(METHOD.clone(), PATH)
            .query(&Args { user_id: id })
            .build()
            .unwrap();

        let parser = make_typed_bad_request_parser(
            |r| AppRequestResponse::LoadUser(r),
            |r| AppRequestResponse::LoadUserError(r),
        );
        self.connector.request(
            request,
            parser,
            AppRequestInfo::LoadUser(id),
            description,
            callback,
        )
    }

    pub fn load_users(
        &self,
        description: RequestDescription,
        callback: Option<StateCallback>,
    ) -> RequestId {
        use users::load_array::*;

        let request = self
            .make_request_authorized(METHOD.clone(), PATH)
            .query(&Args {})
            .build()
            .unwrap();

        let parser = make_parser(|r| AppRequestResponse::LoadUsers(r));
        self.connector
            .request(request, parser, AppRequestInfo::None, description, callback)
    }

    pub fn load_user_state(
        &self,
        user_id: i32,
        description: RequestDescription,
        callback: Option<StateCallback>,
    ) -> RequestId {
        use api::user_state::load::*;

        let request = self
            .make_request_authorized(METHOD.clone(), PATH)
            .query(&Args {
                user_id: Some(user_id),
            })
            .build()
            .unwrap();

        let parser = make_typed_bad_request_parser(
            |r| AppRequestResponse::LoadUserState(r),
            |r| AppRequestResponse::LoadUserStateError(r),
        );
        self.connector.request(
            request,
            parser,
            AppRequestInfo::LoadUserState { user_id },
            description,
            callback,
        )
    }

    pub fn load_event(
        &self,
        id: i32,
        description: RequestDescription,
        callback: Option<StateCallback>,
    ) -> RequestId {
        self.request(
            self.user_state.events.load_by_id_request(id).with_callback(callback),
            description,
        )
    }
    pub fn load_events(
        &self,
        description: RequestDescription,
        callback: Option<StateCallback>,
    ) -> RequestId {
        self.request(self.user_state.events.load_all().with_callback(callback), description,)
    }
    pub fn insert_event(
        &self,
        mut new_event: NewEvent,
        description: RequestDescription,
        callback: Option<StateCallback>,
    ) -> RequestId {
        if new_event.user_id == -1 {
            new_event.user_id = self.me.as_ref().unwrap().user.id;
        }
        self.request(
            self.user_state.events.insert_request(new_event).with_callback(callback),
            description,
            
        )
    }
    pub fn update_event(
        &self,
        upd_event: UpdateEvent,
        description: RequestDescription,
        callback: Option<StateCallback>,
    ) -> RequestId {
        self.request(
            self.user_state.events.update_request(upd_event).with_callback(callback),
            description,
            
        )
    }
    pub fn delete_event(
        &self,
        id: i32,
        description: RequestDescription,
        callback: Option<StateCallback>,
    ) -> RequestId {
        self.request(
            self.user_state.events.delete_by_id_request(id).with_callback(callback),
            description,
            
        )
    }

    pub fn load_event_template(
        &self,
        id: i32,
        description: RequestDescription,
        callback: Option<StateCallback>,
    ) -> RequestId {
        self.request(
            self.user_state.event_templates.load_by_id_request(id).with_callback(callback),
            description,
            
        )
    }
    pub fn load_event_templates(
        &self,
        description: RequestDescription,
        callback: Option<StateCallback>,
    ) -> RequestId {
        self.request(
            self.user_state.event_templates.load_all().with_callback(callback),
            description,
            
        )
    }
    pub fn insert_event_template(
        &self,
        mut new_event_template: NewEventTemplate,
        description: RequestDescription,
        callback: Option<StateCallback>,
    ) -> RequestId {
        if new_event_template.user_id == -1 {
            new_event_template.user_id = self.me.as_ref().unwrap().user.id;
        }
        self.request(
            self.user_state
                .event_templates
                .insert_request(new_event_template).with_callback(callback),
            description,
            
        )
    }
    pub fn update_event_template(
        &self,
        upd_event_template: UpdateEventTemplate,
        description: RequestDescription,
        callback: Option<StateCallback>,
    ) -> RequestId {
        self.request(
            self.user_state
                .event_templates
                .update_request(upd_event_template).with_callback(callback),
            description,
            
        )
    }
    pub fn delete_event_template(
        &self,
        id: i32,
        description: RequestDescription,
        callback: Option<StateCallback>,
    ) -> RequestId {
        self.request(
            self.user_state.event_templates.delete_by_id_request(id).with_callback(callback),
            description,
            
        )
    }

    pub fn load_schedule(
        &self,
        id: i32,
        description: RequestDescription,
        callback: Option<StateCallback>,
    ) -> RequestId {
        self.request(
            self.user_state.schedules.load_by_id_request(id).with_callback(callback),
            description,
            
        )
    }
    pub fn load_schedules(
        &self,
        description: RequestDescription,
        callback: Option<StateCallback>,
    ) -> RequestId {
        self.request(self.user_state.schedules.load_all().with_callback(callback), description,)
    }
    pub fn insert_schedule(
        &self,
        mut new_schedule: NewSchedule,
        description: RequestDescription,
        callback: Option<StateCallback>,
    ) -> RequestId {
        if new_schedule.user_id == -1 {
            new_schedule.user_id = self.me.as_ref().unwrap().user.id;
        }
        self.request(
            self.user_state.schedules.insert_request(new_schedule).with_callback(callback),
            description,
            
        )
    }
    pub fn update_schedule(
        &self,
        upd_schedule: UpdateSchedule,
        description: RequestDescription,
        callback: Option<StateCallback>,
    ) -> RequestId {
        self.request(
            self.user_state.schedules.update_request(upd_schedule).with_callback(callback),
            description,
            
        )
    }
    pub fn delete_schedule(
        &self,
        id: i32,
        description: RequestDescription,
        callback: Option<StateCallback>,
    ) -> RequestId {
        self.request(
            self.user_state.schedules.delete_by_id_request(id).with_callback(callback),
            description,
            
        )
    }
}
