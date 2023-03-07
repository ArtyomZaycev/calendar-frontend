use calendar_lib::api::{auth, events, schedules, user_roles};
use chrono::{Datelike, NaiveDateTime};
use reqwest::{Method, RequestBuilder, StatusCode};
use serde::de::DeserializeOwned;

use crate::{
    config::Config,
    db::{
        aliases::*,
        connector::{Connector, RequestDescriptor},
        request_parser::RequestParser,
    },
    requests::AppRequestDescription,
    ui::widget_signal::StateSignal,
};

use super::requests::AppRequest;

pub struct State {
    connector: Connector<AppRequest, AppRequestDescription>,

    // Should not be modified manually, use requests
    pub me: Option<UserInfo>,
    pub users: Vec<User>,
    pub event_templates: Vec<EventTemplate>,
    pub events: Vec<Event>,
    pub phantom_events: Vec<Event>, // Created from schedules, do not exist in the db
    pub schedules: Vec<Schedule>,

    pub errors: Vec<()>,
}

impl State {
    pub fn new(config: &Config) -> Self {
        Self {
            connector: Connector::new(config),
            me: None,
            users: Vec::default(),
            event_templates: Vec::default(),
            events: Vec::default(),
            phantom_events: Vec::default(),
            schedules: Vec::default(),
            errors: Vec::default(),
        }
    }

    pub fn generate_scheduled_events(&mut self) {
        let now = chrono::Local::now().naive_local();

        let event_exists = |schedule: &Schedule, start: &NaiveDateTime| {
            self.events
                .iter()
                .any(|e| e.plan_id == Some(schedule.id) && &e.start == start)
        };

        self.phantom_events = self
            .schedules
            .iter()
            .flat_map(|schedule| {
                match self
                    .event_templates
                    .iter()
                    .find(|template| template.id == schedule.template_id)
                {
                    Some(template) => schedule
                        .event_plans
                        .iter()
                        .filter_map(|event_plan| {
                            let start = NaiveDateTime::new(now.date(), event_plan.time);
                            (event_plan.weekday == now.date().weekday()
                                && !event_exists(schedule, &start))
                            .then(|| Event {
                                id: -1,
                                user_id: schedule.user_id,
                                name: template.event_name.clone(),
                                description: template.event_description.clone(),
                                start,
                                end: start + chrono::Duration::from_std(template.duration).unwrap(),
                                access_level: schedule.access_level,
                                visibility: EventVisibility::HideAll,
                                plan_id: Some(event_plan.id),
                            })
                        })
                        .collect(),
                    None => vec![],
                }
            })
            .collect();
    }
}

impl State {
    pub fn parse_signal(&mut self, signal: StateSignal) {
        match signal {
            StateSignal::Login(email, password) => self.login(&email, &password),
            StateSignal::Register(name, email, password) => self.register(&name, &email, &password),

            StateSignal::InsertEvent(new_event) => self.insert_event(new_event),
            StateSignal::UpdateEvent(upd_event) => self.update_event(upd_event),
            StateSignal::DeleteEvent(id) => self.delete_event(id),

            StateSignal::InsertEventTemplate(new_event_template) => {
                self.insert_event_template(new_event_template)
            }
            StateSignal::DeleteEventTemplate(id) => self.delete_event_template(id),

            StateSignal::InsertSchedule(new_schedule) => self.insert_schedule(new_schedule),
            StateSignal::UpdateSchedule(upd_schedule) => self.update_schedule(upd_schedule),
            StateSignal::DeleteSchedule(id) => self.delete_schedule(id),
        }
    }
}

impl State {
    pub fn make_request(&self, method: Method, op: &str) -> RequestBuilder {
        self.connector.make_request(method, op)
    }

    pub fn make_request_authorized(&self, method: Method, op: &str) -> RequestBuilder {
        if let Some(me) = &self.me {
            self.connector.make_request(method, op).basic_auth(
                me.user.id,
                Some(std::str::from_utf8(&me.key).expect("parse error")),
            )
        } else {
            todo!()
        }
    }

    // Use for testing only
    #[cfg(debug_assertions)]
    #[allow(dead_code)]
    fn make_empty_parser() -> RequestParser<AppRequest> {
        RequestParser::new_split(|_| AppRequest::None, |_, _| AppRequest::None)
    }

    fn make_parser<U, F>(on_success: F) -> RequestParser<AppRequest>
    where
        U: DeserializeOwned,
        F: FnOnce(U) -> AppRequest + 'static,
    {
        RequestParser::new_complex(on_success, |code, s| AppRequest::Error(code, s))
    }

    #[allow(dead_code)]
    fn make_bad_request_parser<T, F1, F2>(
        on_success: F1,
        on_bad_request: F2,
    ) -> RequestParser<AppRequest>
    where
        T: DeserializeOwned,
        F1: FnOnce(T) -> AppRequest + 'static,
        F2: FnOnce(String) -> AppRequest + 'static,
    {
        RequestParser::new_complex(on_success, |code, msg| {
            if code == StatusCode::BAD_REQUEST {
                on_bad_request(msg)
            } else {
                AppRequest::Error(code, msg)
            }
        })
    }

    fn make_typed_bad_request_parser<T, U, F1, F2>(
        on_success: F1,
        on_bad_request: F2,
    ) -> RequestParser<AppRequest>
    where
        T: DeserializeOwned,
        U: DeserializeOwned,
        F1: FnOnce(T) -> AppRequest + 'static,
        F2: FnOnce(U) -> AppRequest + 'static,
    {
        RequestParser::new_complex(on_success, |code, msg| {
            if code == StatusCode::BAD_REQUEST {
                on_bad_request(serde_json::from_str(&msg).unwrap())
            } else {
                AppRequest::Error(code, msg)
            }
        })
    }
}

impl State {
    pub fn load_access_levels(&mut self) {
        use auth::load_access_levels::*;

        let request = self
            .make_request_authorized(METHOD.clone(), PATH)
            .query(&Args {})
            .build()
            .unwrap();

        let parser = Self::make_parser(|r| AppRequest::LoadAccessLevels(r));
        self.connector
            .request(request, RequestDescriptor::no_description(parser));
    }

    pub fn load_user_roles(&mut self) {
        use user_roles::load_array::*;

        let request = self
            .make_request_authorized(METHOD.clone(), PATH)
            .query(&Args { user_id: None })
            .build()
            .unwrap();

        let parser = Self::make_parser(|r| AppRequest::LoadUserRoles(r));
        self.connector
            .request(request, RequestDescriptor::no_description(parser));
    }

    pub fn logout(&mut self) {
        use auth::logout::*;

        let request = self
            .make_request_authorized(METHOD.clone(), PATH)
            .query(&Args {})
            .json(&Body {})
            .build()
            .unwrap();

        self.me = None;
        self.users = vec![];
        self.events = vec![];

        let parser = RequestParser::new_split(
            |_| AppRequest::None,
            |code, _| AppRequest::Error(code, "Logout error".to_owned()),
        );
        self.connector
            .request(request, RequestDescriptor::no_description(parser));
    }

    pub fn login(&mut self, email: &str, password: &str) {
        use auth::login::*;

        let request = self
            .make_request(METHOD.clone(), PATH)
            .query(&Args {})
            .json(&Body {
                email: email.to_owned(),
                password: password.to_owned(),
            })
            .build()
            .unwrap();

        let parser = Self::make_parser(|r| AppRequest::Login(r));
        self.connector
            .request(request, RequestDescriptor::no_description(parser));
    }

    pub fn register(&mut self, name: &str, email: &str, password: &str) {
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
            |r| AppRequest::Register(r),
            |r| AppRequest::RegisterError(r),
        );
        self.connector
            .request(request, RequestDescriptor::no_description(parser));
    }

    pub fn load_event(&mut self, id: i32) {
        use events::load::*;

        let request = self
            .make_request_authorized(METHOD.clone(), PATH)
            .query(&Args { id })
            .build()
            .unwrap();

        let parser = Self::make_typed_bad_request_parser(
            |r| AppRequest::LoadEvent(r),
            |r| AppRequest::LoadEventError(r),
        );
        self.connector.request(
            request,
            RequestDescriptor::new(AppRequestDescription::LoadEvent(id), parser),
        );
    }
    pub fn load_events(&mut self) {
        use events::load_array::*;

        let request = self
            .make_request_authorized(METHOD.clone(), PATH)
            .query(&Args {})
            .build()
            .unwrap();

        let parser = Self::make_parser(|r| AppRequest::LoadEvents(r));
        self.connector
            .request(request, RequestDescriptor::no_description(parser));
    }
    pub fn insert_event(&mut self, mut new_event: NewEvent) {
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

        let parser = Self::make_parser(|r| AppRequest::InsertEvent(r));
        self.connector
            .request(request, RequestDescriptor::no_description(parser));
    }
    pub fn update_event(&mut self, upd_event: UpdateEvent) {
        use events::update::*;

        let id = upd_event.id;
        let request = self
            .make_request_authorized(METHOD.clone(), PATH)
            .query(&Args {})
            .json(&Body { upd_event })
            .build()
            .unwrap();

        let parser = Self::make_parser(|r| AppRequest::UpdateEvent(r));
        self.connector.request(
            request,
            RequestDescriptor::new(AppRequestDescription::UpdateEvent(id), parser),
        );
    }
    pub fn delete_event(&mut self, id: i32) {
        use events::delete::*;

        let request = self
            .make_request_authorized(METHOD.clone(), PATH)
            .query(&Args { id })
            .json(&Body {})
            .build()
            .unwrap();

        let parser = Self::make_parser(|r| AppRequest::DeleteEvent(r));
        self.connector.request(
            request,
            RequestDescriptor::new(AppRequestDescription::DeleteEvent(id), parser),
        );
    }

    pub fn load_event_template(&mut self, id: i32) {
        use event_templates::load::*;

        let request = self
            .make_request_authorized(METHOD.clone(), PATH)
            .query(&Args { id })
            .build()
            .unwrap();

        let parser = Self::make_typed_bad_request_parser(
            |r| AppRequest::LoadEventTemplate(r),
            |r| AppRequest::LoadEventTemplateError(r),
        );
        self.connector.request(
            request,
            RequestDescriptor::new(AppRequestDescription::LoadEventTemplate(id), parser),
        );
    }
    pub fn load_event_templates(&mut self) {
        use event_templates::load_array::*;

        let request = self
            .make_request_authorized(METHOD.clone(), PATH)
            .query(&Args {})
            .build()
            .unwrap();

        let parser = Self::make_parser(|r| AppRequest::LoadEventTemplates(r));
        self.connector
            .request(request, RequestDescriptor::no_description(parser));
    }
    pub fn insert_event_template(&mut self, mut new_event_template: NewEventTemplate) {
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

        let parser = Self::make_parser(|r| AppRequest::InsertEventTemplate(r));
        self.connector
            .request(request, RequestDescriptor::no_description(parser));
    }
    pub fn update_event_template(&mut self, upd_event_template: UpdateEventTemplate) {
        use event_templates::update::*;

        let id = upd_event_template.id;
        let request = self
            .make_request_authorized(METHOD.clone(), PATH)
            .query(&Args {})
            .json(&Body { upd_event_template })
            .build()
            .unwrap();

        let parser = Self::make_parser(|r| AppRequest::UpdateEventTemplate(r));
        self.connector.request(
            request,
            RequestDescriptor::new(AppRequestDescription::UpdateEventTemplate(id), parser),
        );
    }
    pub fn delete_event_template(&mut self, id: i32) {
        use event_templates::delete::*;

        let request = self
            .make_request_authorized(METHOD.clone(), PATH)
            .query(&Args { id })
            .json(&Body {})
            .build()
            .unwrap();

        let parser = Self::make_parser(|r| AppRequest::DeleteEventTemplate(r));
        self.connector.request(
            request,
            RequestDescriptor::new(AppRequestDescription::DeleteEventTemplate(id), parser),
        );
    }

    pub fn load_schedule(&mut self, id: i32) {
        use schedules::load::*;

        let request = self
            .make_request_authorized(METHOD.clone(), PATH)
            .query(&Args { id })
            .build()
            .unwrap();

        let parser = Self::make_typed_bad_request_parser(
            |r| AppRequest::LoadSchedule(r),
            |r| AppRequest::LoadScheduleError(r),
        );
        self.connector.request(
            request,
            RequestDescriptor::new(AppRequestDescription::LoadSchedule(id), parser),
        );
    }
    pub fn load_schedules(&mut self) {
        use schedules::load_array::*;

        let request = self
            .make_request_authorized(METHOD.clone(), PATH)
            .query(&Args {})
            .build()
            .unwrap();

        let parser = Self::make_parser(|r| AppRequest::LoadSchedules(r));
        self.connector
            .request(request, RequestDescriptor::no_description(parser));
    }
    pub fn insert_schedule(&mut self, mut new_schedule: NewSchedule) {
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

        let parser = Self::make_parser(|r| AppRequest::InsertSchedule(r));
        self.connector
            .request(request, RequestDescriptor::no_description(parser));
    }
    pub fn update_schedule(&mut self, upd_schedule: UpdateSchedule) {
        use schedules::update::*;

        let id = upd_schedule.id;
        let request = self
            .make_request_authorized(METHOD.clone(), PATH)
            .query(&Args {})
            .json(&Body { upd_schedule })
            .build()
            .unwrap();

        let parser = Self::make_parser(|r| AppRequest::UpdateSchedule(r));
        self.connector.request(
            request,
            RequestDescriptor::new(AppRequestDescription::UpdateSchedule(id), parser),
        );
    }
    pub fn delete_schedule(&mut self, id: i32) {
        use schedules::delete::*;

        let request = self
            .make_request_authorized(METHOD.clone(), PATH)
            .query(&Args { id })
            .json(&Body {})
            .build()
            .unwrap();

        let parser = Self::make_parser(|r| AppRequest::DeleteSchedule(r));
        self.connector.request(
            request,
            RequestDescriptor::new(AppRequestDescription::DeleteSchedule(id), parser),
        );
    }
}

impl State {
    fn parse_action(&mut self, request: AppRequest, description: AppRequestDescription) {
        match request {
            AppRequest::Login(res) => {
                self.me = Some(UserInfo {
                    user: res.user,
                    current_access_level: res.access_level.level,
                    access_levels: vec![res.access_level],
                    key: res.key,
                    roles: vec![],
                });
                self.load_access_levels();
                self.load_events();
                self.load_user_roles();
                self.load_event_templates();
                self.load_schedules();
            }
            AppRequest::Register(_) => {}
            AppRequest::RegisterError(_) => {}
            AppRequest::LoadAccessLevels(mut r) => {
                if let Some(me) = &mut self.me {
                    r.array.sort_by(|a, b| a.level.cmp(&b.level));
                    me.access_levels = r.array;
                }
            }
            AppRequest::LoadUserRoles(res) => {
                if let Some(me) = &mut self.me {
                    me.roles = res.array;
                }
            }
            AppRequest::LoadEvent(res) => {
                let event = res.value;
                match self.events.iter_mut().find(|e| e.id == event.id) {
                    Some(e) => *e = event,
                    None => self.events.push(event),
                }
                self.events.sort_by_key(|v| v.start);
            }
            AppRequest::LoadEventError(res) => match res {
                events::load::BadRequestResponse::NotFound => {
                    if let AppRequestDescription::LoadEvent(id) = description {
                        if let Some(ind) = self.events.iter().position(|e| e.id == id) {
                            self.events.remove(ind);
                            self.events.sort_by_key(|v| v.start);
                        }
                    }
                }
            },
            AppRequest::LoadEvents(res) => {
                self.events = res.array;
                self.events.sort_by_key(|v| v.start);
            }
            AppRequest::InsertEvent(_) => {
                self.load_events();
            }
            AppRequest::UpdateEvent(_) => {
                if let AppRequestDescription::UpdateEvent(id) = description {
                    self.load_event(id);
                }
            }
            AppRequest::DeleteEvent(_) => {
                if let AppRequestDescription::DeleteEvent(id) = description {
                    if let Some(ind) = self.events.iter().position(|e| e.id == id) {
                        self.events.remove(ind);
                    }
                }
            }
            AppRequest::LoadEventTemplate(res) => {
                let template = res.value;
                match self
                    .event_templates
                    .iter_mut()
                    .find(|t| t.id == template.id)
                {
                    Some(t) => *t = template,
                    None => self.event_templates.push(template),
                }
            }
            AppRequest::LoadEventTemplateError(res) => match res {
                event_templates::load::BadRequestResponse::NotFound => {
                    if let AppRequestDescription::LoadEventTemplate(id) = description {
                        if let Some(ind) = self.event_templates.iter().position(|t| t.id == id) {
                            self.event_templates.remove(ind);
                        }
                    }
                }
            },
            AppRequest::LoadEventTemplates(res) => {
                self.event_templates = res.array;
            }
            AppRequest::InsertEventTemplate(_) => {
                self.load_event_templates();
            }
            AppRequest::UpdateEventTemplate(_) => {
                if let AppRequestDescription::UpdateEventTemplate(id) = description {
                    self.load_event_template(id);
                }
            }
            AppRequest::DeleteEventTemplate(_) => {
                if let AppRequestDescription::DeleteEventTemplate(id) = description {
                    if let Some(ind) = self.event_templates.iter().position(|e| e.id == id) {
                        self.event_templates.remove(ind);
                    }
                }
            }
            AppRequest::LoadSchedule(res) => {
                let schedule = res.value;
                match self.schedules.iter_mut().find(|s| s.id == schedule.id) {
                    Some(s) => *s = schedule,
                    None => self.schedules.push(schedule),
                }
                self.generate_scheduled_events();
            }
            AppRequest::LoadScheduleError(res) => match res {
                schedules::load::BadRequestResponse::NotFound => {
                    if let AppRequestDescription::LoadSchedule(id) = description {
                        if let Some(ind) = self.schedules.iter().position(|t| t.id == id) {
                            self.schedules.remove(ind);
                            self.generate_scheduled_events();
                        }
                    }
                }
            },
            AppRequest::LoadSchedules(res) => {
                self.schedules = res.array;
                self.generate_scheduled_events();
            }
            AppRequest::InsertSchedule(_) => {
                self.load_schedules();
            }
            AppRequest::UpdateSchedule(_) => {
                if let AppRequestDescription::UpdateSchedule(id) = description {
                    self.load_schedule(id);
                }
            }
            /*StateAction::UpdateSchedule(_) => {
                self.load_schedules();
            }*/
            AppRequest::DeleteSchedule(_) => {
                if let AppRequestDescription::DeleteSchedule(id) = description {
                    if let Some(ind) = self.schedules.iter().position(|s| s.id == id) {
                        self.schedules.remove(ind);
                    }
                }
            }
            AppRequest::None => {}
            AppRequest::Error(status, s) => {
                println!("smth went wrong: {status:?}=>{s:?}");
            }
        }
    }

    pub fn poll(&mut self) -> Vec<(AppRequest, AppRequestDescription)> {
        let actions = self.connector.poll();
        actions
            .clone()
            .into_iter()
            .for_each(|(request, description)| self.parse_action(request, description));
        actions
    }

    pub fn get_active_requests_descriptions(&self) -> Vec<AppRequestDescription> {
        self.connector.get_active_requests_descriptions()
    }
}
