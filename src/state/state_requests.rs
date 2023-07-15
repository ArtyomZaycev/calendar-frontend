use crate::requests::AppRequestResponse;
use crate::{
    db::{
        aliases::*,
        request::{RequestDescription, RequestId},
        request_parser::RequestParser,
    },
    requests::AppRequestInfo,
    state::*,
};
use calendar_lib::api::{
    auth::{self, types::NewPassword},
    events, schedules, user_roles,
};

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

        let parser = Self::make_parser(|r| AppRequestResponse::LoadAccessLevels(r));
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

        let parser = Self::make_parser(|r| AppRequestResponse::LoadUserRoles(r));
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

        let parser = Self::make_parser(|r| AppRequestResponse::LoginByKey(r));
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

        let parser = Self::make_typed_bad_request_parser(
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

        let parser = Self::make_typed_bad_request_parser(
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

        let parser = Self::make_parser(|r| AppRequestResponse::NewPassword(r));
        self.connector
            .request(request, parser, AppRequestInfo::None, description)
    }

    pub fn load_event(&mut self, id: i32, description: RequestDescription) -> RequestId {
        use events::load::*;

        let request = self
            .make_request_authorized(METHOD.clone(), PATH)
            .query(&Args { id })
            .build()
            .unwrap();

        let parser = Self::make_typed_bad_request_parser(
            |r| AppRequestResponse::LoadEvent(r),
            |r| AppRequestResponse::LoadEventError(r),
        );
        self.connector
            .request(request, parser, AppRequestInfo::LoadEvent(id), description)
    }
    pub fn load_events(&mut self, description: RequestDescription) -> RequestId {
        use events::load_array::*;

        let request = self
            .make_request_authorized(METHOD.clone(), PATH)
            .query(&Args {})
            .build()
            .unwrap();

        let parser = Self::make_parser(|r| AppRequestResponse::LoadEvents(r));
        self.connector
            .request(request, parser, AppRequestInfo::None, description)
    }
    pub fn insert_event(
        &mut self,
        mut new_event: NewEvent,
        description: RequestDescription,
    ) -> RequestId {
        use events::insert::*;

        if new_event.user_id == -1 {
            new_event.user_id = self.me.as_ref().unwrap().user.id;
        }

        let request = self
            .make_request_authorized(METHOD.clone(), PATH)
            .query(&Args {})
            .json(&Body { new_event })
            .build()
            .unwrap();

        let parser = Self::make_parser(|r| AppRequestResponse::InsertEvent(r));
        self.connector
            .request(request, parser, AppRequestInfo::None, description)
    }
    pub fn update_event(
        &mut self,
        upd_event: UpdateEvent,
        description: RequestDescription,
    ) -> RequestId {
        use events::update::*;

        let id = upd_event.id;
        let request = self
            .make_request_authorized(METHOD.clone(), PATH)
            .query(&Args {})
            .json(&Body { upd_event })
            .build()
            .unwrap();

        let parser = Self::make_parser(|r| AppRequestResponse::UpdateEvent(r));
        self.connector.request(
            request,
            parser,
            AppRequestInfo::UpdateEvent(id),
            description,
        )
    }
    pub fn delete_event(&mut self, id: i32, description: RequestDescription) -> RequestId {
        use events::delete::*;

        let request = self
            .make_request_authorized(METHOD.clone(), PATH)
            .query(&Args { id })
            .json(&Body {})
            .build()
            .unwrap();

        let parser = Self::make_parser(|r| AppRequestResponse::DeleteEvent(r));
        self.connector.request(
            request,
            parser,
            AppRequestInfo::DeleteEvent(id),
            description,
        )
    }

    pub fn load_event_template(&mut self, id: i32, description: RequestDescription) -> RequestId {
        use event_templates::load::*;

        let request = self
            .make_request_authorized(METHOD.clone(), PATH)
            .query(&Args { id })
            .build()
            .unwrap();

        let parser = Self::make_typed_bad_request_parser(
            |r| AppRequestResponse::LoadEventTemplate(r),
            |r| AppRequestResponse::LoadEventTemplateError(r),
        );
        self.connector.request(
            request,
            parser,
            AppRequestInfo::LoadEventTemplate(id),
            description,
        )
    }
    pub fn load_event_templates(&mut self, description: RequestDescription) -> RequestId {
        use event_templates::load_array::*;

        let request = self
            .make_request_authorized(METHOD.clone(), PATH)
            .query(&Args {})
            .build()
            .unwrap();

        let parser = Self::make_parser(|r| AppRequestResponse::LoadEventTemplates(r));
        self.connector
            .request(request, parser, AppRequestInfo::None, description)
    }
    pub fn insert_event_template(
        &mut self,
        mut new_event_template: NewEventTemplate,
        description: RequestDescription,
    ) -> RequestId {
        use event_templates::insert::*;

        if new_event_template.user_id == -1 {
            new_event_template.user_id = self.me.as_ref().unwrap().user.id;
        }

        let request = self
            .make_request_authorized(METHOD.clone(), PATH)
            .query(&Args {})
            .json(&Body { new_event_template })
            .build()
            .unwrap();

        let parser = Self::make_parser(|r| AppRequestResponse::InsertEventTemplate(r));
        self.connector
            .request(request, parser, AppRequestInfo::None, description)
    }
    pub fn update_event_template(
        &mut self,
        upd_event_template: UpdateEventTemplate,
        description: RequestDescription,
    ) -> RequestId {
        use event_templates::update::*;

        let id = upd_event_template.id;
        let request = self
            .make_request_authorized(METHOD.clone(), PATH)
            .query(&Args {})
            .json(&Body { upd_event_template })
            .build()
            .unwrap();

        let parser = Self::make_parser(|r| AppRequestResponse::UpdateEventTemplate(r));
        self.connector.request(
            request,
            parser,
            AppRequestInfo::UpdateEventTemplate(id),
            description,
        )
    }
    pub fn delete_event_template(&mut self, id: i32, description: RequestDescription) -> RequestId {
        use event_templates::delete::*;

        let request = self
            .make_request_authorized(METHOD.clone(), PATH)
            .query(&Args { id })
            .json(&Body {})
            .build()
            .unwrap();

        let parser = Self::make_parser(|r| AppRequestResponse::DeleteEventTemplate(r));
        self.connector.request(
            request,
            parser,
            AppRequestInfo::DeleteEventTemplate(id),
            description,
        )
    }

    pub fn load_schedule(&mut self, id: i32, description: RequestDescription) -> RequestId {
        use schedules::load::*;

        let request = self
            .make_request_authorized(METHOD.clone(), PATH)
            .query(&Args { id })
            .build()
            .unwrap();

        let parser = Self::make_typed_bad_request_parser(
            |r| AppRequestResponse::LoadSchedule(r),
            |r| AppRequestResponse::LoadScheduleError(r),
        );
        self.connector.request(
            request,
            parser,
            AppRequestInfo::LoadSchedule(id),
            description,
        )
    }
    pub fn load_schedules(&mut self, description: RequestDescription) -> RequestId {
        use schedules::load_array::*;

        let request = self
            .make_request_authorized(METHOD.clone(), PATH)
            .query(&Args {})
            .build()
            .unwrap();

        let parser = Self::make_parser(|r| AppRequestResponse::LoadSchedules(r));
        self.connector
            .request(request, parser, AppRequestInfo::None, description)
    }
    pub fn insert_schedule(
        &mut self,
        mut new_schedule: NewSchedule,
        description: RequestDescription,
    ) -> RequestId {
        use schedules::insert::*;

        if new_schedule.user_id == -1 {
            new_schedule.user_id = self.me.as_ref().unwrap().user.id;
        }

        let request = self
            .make_request_authorized(METHOD.clone(), PATH)
            .query(&Args {})
            .json(&Body { new_schedule })
            .build()
            .unwrap();

        let parser = Self::make_parser(|r| AppRequestResponse::InsertSchedule(r));
        self.connector
            .request(request, parser, AppRequestInfo::None, description)
    }
    pub fn update_schedule(
        &mut self,
        upd_schedule: UpdateSchedule,
        description: RequestDescription,
    ) -> RequestId {
        use schedules::update::*;

        let id = upd_schedule.id;
        let request = self
            .make_request_authorized(METHOD.clone(), PATH)
            .query(&Args {})
            .json(&Body { upd_schedule })
            .build()
            .unwrap();

        let parser = Self::make_parser(|r| AppRequestResponse::UpdateSchedule(r));
        self.connector.request(
            request,
            parser,
            AppRequestInfo::UpdateSchedule(id),
            description,
        )
    }
    pub fn delete_schedule(&mut self, id: i32, description: RequestDescription) -> RequestId {
        use schedules::delete::*;

        let request = self
            .make_request_authorized(METHOD.clone(), PATH)
            .query(&Args { id })
            .json(&Body {})
            .build()
            .unwrap();

        let parser = Self::make_parser(|r| AppRequestResponse::DeleteSchedule(r));
        self.connector.request(
            request,
            parser,
            AppRequestInfo::DeleteSchedule(id),
            description,
        )
    }
}
