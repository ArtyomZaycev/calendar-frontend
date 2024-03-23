use crate::requests::AppRequestResponse;
use crate::tables::utils::*;
use crate::{
    db::{aliases::*, request::*, request_parser::*, table::*},
    requests::AppRequestInfo,
};
use super::*;

use calendar_lib::api::{
    self,
    auth::{self, types::NewPassword},
    user_roles,
};
use serde::Serialize;

impl State {
    fn request<Q: Serialize, B: Serialize>(
        &mut self,
        request: RequestBuilder<Q, B>,
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

impl State {
    pub fn change_access_level(&mut self, new_access_level: i32) {
        self.current_access_level = new_access_level;
        self.clear_events();
    }

    pub fn load_access_levels(&mut self, description: RequestDescription) -> RequestId {
        use auth::load_access_levels::*;

        let request = self
            .make_request_authorized(METHOD.clone(), PATH)
            .query(&Args {})
            .build()
            .unwrap();

        let parser = make_parser(|r| AppRequestResponse::LoadAccessLevels(r));
        self.connector
            .request(request, parser, AppRequestInfo::None, description)
    }

    pub fn load_user_roles(&mut self, description: RequestDescription) -> RequestId {
        use user_roles::load_array::*;

        let request = self
            .make_request_authorized(METHOD.clone(), PATH)
            .query(&Args { user_id: None })
            .build()
            .unwrap();

        let parser = make_parser(|r| AppRequestResponse::LoadUserRoles(r));
        self.connector
            .request(request, parser, AppRequestInfo::None, description)
    }

    pub fn logout(&mut self, description: RequestDescription) -> RequestId {
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
            .request(request, parser, AppRequestInfo::None, description)
    }

    pub fn login_by_jwt(&mut self, jwt: &str, description: RequestDescription) -> RequestId {
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
            .request(request, parser, AppRequestInfo::None, description)
    }

    pub fn login(
        &mut self,
        email: &str,
        password: &str,
        description: RequestDescription,
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
            .request(request, parser, AppRequestInfo::None, description)
    }

    pub fn register(
        &mut self,
        name: &str,
        email: &str,
        password: &str,
        description: RequestDescription,
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
            .request(request, parser, AppRequestInfo::None, description)
    }

    pub fn new_password(
        &mut self,
        access_level: i32,
        viewer: Option<NewPassword>,
        editor: Option<NewPassword>,
        description: RequestDescription,
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
            .request(request, parser, AppRequestInfo::None, description)
    }

    pub fn load_user_ids(&mut self, description: RequestDescription) -> RequestId {
        use users::load_ids::*;

        let request = self
            .make_request_authorized(METHOD.clone(), PATH)
            .query(&Args {})
            .build()
            .unwrap();

        let parser = make_parser(|r| AppRequestResponse::LoadUserIds(r));
        self.connector
            .request(request, parser, AppRequestInfo::None, description)
    }

    pub fn load_user(&mut self, id: i32, description: RequestDescription) -> RequestId {
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
        self.connector
            .request(request, parser, AppRequestInfo::LoadUser(id), description)
    }

    pub fn load_users(&mut self, description: RequestDescription) -> RequestId {
        use users::load_array::*;

        let request = self
            .make_request_authorized(METHOD.clone(), PATH)
            .query(&Args {})
            .build()
            .unwrap();

        let parser = make_parser(|r| AppRequestResponse::LoadUsers(r));
        self.connector
            .request(request, parser, AppRequestInfo::None, description)
    }

    pub fn load_user_state(&mut self, user_id: i32, description: RequestDescription) -> RequestId {
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
        )
    }

    pub fn load_event(&mut self, id: i32, description: RequestDescription) -> RequestId {
        self.request(self.user_state.events.load_by_id_request(id), description)
    }
    pub fn load_events(&mut self, description: RequestDescription) -> RequestId {
        self.request(self.user_state.events.load_all(), description)
    }
    pub fn insert_event(
        &mut self,
        mut new_event: NewEvent,
        description: RequestDescription,
    ) -> RequestId {
        if new_event.user_id == -1 {
            new_event.user_id = self.me.as_ref().unwrap().user.id;
        }
        self.request(
            self.user_state.events.insert_request(new_event),
            description,
        )
    }
    pub fn update_event(
        &mut self,
        upd_event: UpdateEvent,
        description: RequestDescription,
    ) -> RequestId {
        self.request(
            self.user_state.events.update_request(upd_event),
            description,
        )
    }
    pub fn delete_event(&mut self, id: i32, description: RequestDescription) -> RequestId {
        self.request(self.user_state.events.delete_by_id_request(id), description)
    }

    pub fn load_event_template(&mut self, id: i32, description: RequestDescription) -> RequestId {
        self.request(
            self.user_state.event_templates.load_by_id_request(id),
            description,
        )
    }
    pub fn load_event_templates(&mut self, description: RequestDescription) -> RequestId {
        self.request(self.user_state.event_templates.load_all(), description)
    }
    pub fn insert_event_template(
        &mut self,
        mut new_event_template: NewEventTemplate,
        description: RequestDescription,
    ) -> RequestId {
        if new_event_template.user_id == -1 {
            new_event_template.user_id = self.me.as_ref().unwrap().user.id;
        }
        self.request(
            self.user_state
                .event_templates
                .insert_request(new_event_template),
            description,
        )
    }
    pub fn update_event_template(
        &mut self,
        upd_event_template: UpdateEventTemplate,
        description: RequestDescription,
    ) -> RequestId {
        self.request(
            self.user_state
                .event_templates
                .update_request(upd_event_template),
            description,
        )
    }
    pub fn delete_event_template(&mut self, id: i32, description: RequestDescription) -> RequestId {
        self.request(
            self.user_state.event_templates.delete_by_id_request(id),
            description,
        )
    }

    pub fn load_schedule(&mut self, id: i32, description: RequestDescription) -> RequestId {
        self.request(
            self.user_state.schedules.load_by_id_request(id),
            description,
        )
    }
    pub fn load_schedules(&mut self, description: RequestDescription) -> RequestId {
        self.request(self.user_state.schedules.load_all(), description)
    }
    pub fn insert_schedule(
        &mut self,
        mut new_schedule: NewSchedule,
        description: RequestDescription,
    ) -> RequestId {
        if new_schedule.user_id == -1 {
            new_schedule.user_id = self.me.as_ref().unwrap().user.id;
        }
        self.request(
            self.user_state.schedules.insert_request(new_schedule),
            description,
        )
    }
    pub fn update_schedule(
        &mut self,
        upd_schedule: UpdateSchedule,
        description: RequestDescription,
    ) -> RequestId {
        self.request(
            self.user_state.schedules.update_request(upd_schedule),
            description,
        )
    }
    pub fn delete_schedule(&mut self, id: i32, description: RequestDescription) -> RequestId {
        self.request(
            self.user_state.schedules.delete_by_id_request(id),
            description,
        )
    }
}
