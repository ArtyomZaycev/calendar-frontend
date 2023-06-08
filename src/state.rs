use super::requests::AppRequestResponse;
use crate::{
    config::Config,
    db::{
        aliases::*,
        connector::Connector,
        request::{RequestDescription, RequestId},
        request_parser::RequestParser,
    },
    requests::{AppRequestInfo, AppRequestResponseInfo},
    ui::signal::{RequestSignal, StateSignal},
};
use calendar_lib::api::{
    auth::{
        self,
        types::{AccessLevel, NewPassword},
    },
    events, schedules, user_roles,
};
use chrono::{Datelike, Duration, NaiveDate, NaiveDateTime};
use itertools::Itertools;
use reqwest::{Method, RequestBuilder, StatusCode};
use serde::de::DeserializeOwned;
use std::collections::HashMap;

pub struct State {
    pub connector: Connector<AppRequestResponse, AppRequestInfo, AppRequestResponseInfo>,
    /// Has both server and phantom events
    events_per_day: HashMap<NaiveDate, Vec<Event>>,
    current_access_level: i32,

    me: Option<UserInfo>,
    access_levels: Vec<AccessLevel>,
    users: Vec<User>,
    event_templates: Vec<EventTemplate>,
    events: Vec<Event>,
    schedules: Vec<Schedule>,

    pub errors: Vec<()>,
}

impl State {
    pub fn new(config: &Config) -> Self {
        Self {
            connector: Connector::new(config),
            events_per_day: HashMap::new(),
            me: None,
            current_access_level: -1,
            access_levels: Vec::default(),
            users: Vec::default(),
            event_templates: Vec::default(),
            events: Vec::default(),
            schedules: Vec::default(),
            errors: Vec::default(),
        }
    }

    fn clear_events_for_day(&mut self, date: NaiveDate) {
        self.events_per_day.remove(&date);
    }
    fn clear_events(&mut self) {
        self.events_per_day.clear();
    }

    fn generate_phantom_events(&self, date: NaiveDate) -> Vec<Event> {
        let event_exists = |plan_id: i32| {
            self.events
                .iter()
                .any(|e| e.plan_id == Some(plan_id) && e.start.date() == date)
        };

        let level = self.get_access_level().level;
        self.schedules
            .iter()
            .filter(move |s| s.access_level <= level)
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
                            let start = NaiveDateTime::new(date, event_plan.time);
                            (event_plan.weekday == date.weekday() && !event_exists(event_plan.id))
                                .then(|| Event {
                                    id: -1,
                                    user_id: schedule.user_id,
                                    name: template.event_name.clone(),
                                    description: template.event_description.clone(),
                                    start,
                                    end: start
                                        + chrono::Duration::from_std(template.duration).unwrap(),
                                    access_level: schedule.access_level,
                                    visibility: EventVisibility::HideAll,
                                    plan_id: Some(event_plan.id),
                                })
                        })
                        .collect(),
                    None => vec![],
                }
            })
            .collect()
    }

    pub fn prepare_date(&mut self, date: NaiveDate) {
        if !self.events_per_day.contains_key(&date) {
            let level = self.get_access_level().level;
            self.events_per_day.insert(date, {
                self.events
                    .iter()
                    .filter(|e| e.start.date() == date)
                    .filter_map(move |e| {
                        if e.access_level <= level {
                            Some(e.clone())
                        } else {
                            match e.visibility {
                                EventVisibility::HideAll => None,
                                EventVisibility::HideName => Some(Event {
                                    name: "".to_owned(),
                                    description: None,
                                    ..e.clone()
                                }),
                                EventVisibility::HideDescription => Some(Event {
                                    description: None,
                                    ..e.clone()
                                }),
                                EventVisibility::Show => Some(e.clone()),
                            }
                        }
                    })
                    .chain(self.generate_phantom_events(date))
                    .sorted_by_key(|v| v.start)
                    .collect()
            });
        }
    }
}

impl State {
    pub fn get_access_level(&self) -> AccessLevel {
        let levels = self
            .access_levels
            .iter()
            .filter(|l| l.level == self.current_access_level)
            .collect_vec();
        if levels.len() == 0 {
            self.access_levels.last().unwrap().clone()
        } else if levels.len() == 1 {
            levels[0].clone()
        } else {
            (*levels.iter().find(|v| v.edit_rights).unwrap_or(&levels[0])).clone()
        }
    }

    pub fn get_me(&self) -> &Option<UserInfo> {
        &self.me
    }
    pub fn get_access_levels(&self) -> &Vec<AccessLevel> {
        &self.access_levels
    }
    pub fn get_users(&self) -> &Vec<User> {
        &self.users
    }
    pub fn get_event_templates(&self) -> &Vec<EventTemplate> {
        &self.event_templates
    }
    pub fn get_events(&self) -> &Vec<Event> {
        &self.events
    }
    pub fn get_schedules(&self) -> &Vec<Schedule> {
        &self.schedules
    }

    pub fn get_events_for_date(&self, date: NaiveDate) -> &[Event] {
        self.events_per_day.get(&date).unwrap()
    }
    pub fn get_prepared_events_for_date(&mut self, date: NaiveDate) -> &[Event] {
        self.prepare_date(date);
        self.get_events_for_date(date)
    }
}

impl State {
    pub fn parse_signal(&mut self, signal: StateSignal) {
        match signal {
            StateSignal::ChangeAccessLevel(new_access_level) => {
                self.change_access_level(new_access_level);
            }

            StateSignal::RequestSignal(description, signal) => match signal {
                RequestSignal::Login(email, password) => {
                    self.login(&email, &password, description);
                }
                RequestSignal::Register(name, email, password) => {
                    self.register(&name, &email, &password, description);
                }

                RequestSignal::InsertEvent(new_event) => {
                    self.insert_event(new_event, description);
                }
                RequestSignal::UpdateEvent(upd_event) => {
                    self.update_event(upd_event, description);
                }
                RequestSignal::DeleteEvent(id) => {
                    self.delete_event(id, description);
                }

                RequestSignal::InsertEventTemplate(new_event_template) => {
                    self.insert_event_template(new_event_template, description);
                }
                RequestSignal::UpdateEventTemplate(upd_event_template) => {
                    self.update_event_template(upd_event_template, description);
                }
                RequestSignal::DeleteEventTemplate(id) => {
                    self.delete_event_template(id, description);
                }

                RequestSignal::InsertSchedule(new_schedule) => {
                    self.insert_schedule(new_schedule, description);
                }
                RequestSignal::UpdateSchedule(upd_schedule) => {
                    self.update_schedule(upd_schedule, description);
                }
                RequestSignal::DeleteSchedule(id) => {
                    self.delete_schedule(id, description);
                }

                RequestSignal::InsertPassword(access_level, viewer_password, editor_password) => {
                    self.new_password(access_level, viewer_password, editor_password, description);
                }
                RequestSignal::AcceptScheduledEvent(date, plan_id) => {
                    if let Some((schedule, plan)) = self.schedules.iter().find_map(|schedule| {
                        schedule
                            .event_plans
                            .iter()
                            .find(|plan| plan.id == plan_id)
                            .map(|plan| (schedule, plan))
                    }) {
                        if let Some(template) = self
                            .event_templates
                            .iter()
                            .find(|template| schedule.template_id == template.id)
                        {
                            let start = NaiveDateTime::new(date, plan.time);
                            let new_event = NewEvent {
                                user_id: -1,
                                name: template.event_name.clone(),
                                description: template.event_description.clone(),
                                start,
                                end: start
                                    .checked_add_signed(
                                        Duration::from_std(template.duration).unwrap(),
                                    )
                                    .unwrap(),
                                access_level: template.access_level,
                                visibility: EventVisibility::HideAll,
                                plan_id: Some(plan_id),
                            };
                            self.insert_event(new_event, description);
                        }
                    }
                }
            },
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
            panic!()
        }
    }

    // Use for testing only
    #[cfg(debug_assertions)]
    #[allow(dead_code)]
    fn make_empty_parser() -> RequestParser<AppRequestResponse> {
        RequestParser::new_split(
            |_| AppRequestResponse::None,
            |_, _| AppRequestResponse::None,
        )
    }

    fn make_parser<U, F>(on_success: F) -> RequestParser<AppRequestResponse>
    where
        U: DeserializeOwned,
        F: FnOnce(U) -> AppRequestResponse + 'static,
    {
        RequestParser::new_complex(on_success, |code, s| AppRequestResponse::Error(code, s))
    }

    #[allow(dead_code)]
    fn make_bad_request_parser<T, F1, F2>(
        on_success: F1,
        on_bad_request: F2,
    ) -> RequestParser<AppRequestResponse>
    where
        T: DeserializeOwned,
        F1: FnOnce(T) -> AppRequestResponse + 'static,
        F2: FnOnce(String) -> AppRequestResponse + 'static,
    {
        RequestParser::new_complex(on_success, |code, msg| {
            if code == StatusCode::BAD_REQUEST {
                on_bad_request(msg)
            } else {
                AppRequestResponse::Error(code, msg)
            }
        })
    }

    fn make_typed_bad_request_parser<T, U, F1, F2>(
        on_success: F1,
        on_bad_request: F2,
    ) -> RequestParser<AppRequestResponse>
    where
        T: DeserializeOwned,
        U: DeserializeOwned,
        F1: FnOnce(T) -> AppRequestResponse + 'static,
        F2: FnOnce(U) -> AppRequestResponse + 'static,
    {
        RequestParser::new_complex(on_success, |code, msg| {
            if code == StatusCode::BAD_REQUEST {
                on_bad_request(serde_json::from_str(&msg).unwrap())
            } else {
                AppRequestResponse::Error(code, msg)
            }
        })
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

        self.me = None;
        self.events_per_day.clear();
        self.current_access_level = -1;
        self.access_levels = vec![];
        self.users = vec![];
        self.event_templates = vec![];
        self.events = vec![];
        self.schedules = vec![];

        let parser = RequestParser::new_split(
            |_| AppRequestResponse::None,
            |code, _| AppRequestResponse::Error(code, "Logout error".to_owned()),
        );
        self.connector
            .request(request, parser, AppRequestInfo::None, description)
    }

    pub fn login_by_key(
        &mut self,
        key: Vec<u8>,
        description: RequestDescription,
    ) -> RequestId {
        use auth::login_by_key::*;

        let request = self
            .make_request(METHOD.clone(), PATH)
            .query(&Args {})
            .json(&Body {
                user_id: 1,
                key,
            })
            .build()
            .unwrap();

        let parser = Self::make_parser(
            |r| AppRequestResponse::LoginByKey(r),
        );
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

impl State {
    fn load_state(&mut self) {
        self.load_access_levels(RequestDescription::default());
        self.load_events(RequestDescription::default());
        self.load_user_roles(RequestDescription::default());
        self.load_event_templates(RequestDescription::default());
        self.load_schedules(RequestDescription::default());
    }

    fn parse_request(&mut self, response: AppRequestResponse, info: AppRequestInfo) {
        match response {
            AppRequestResponse::Login(res) => {
                self.me = Some(UserInfo {
                    user: res.user,
                    key: res.key,
                    roles: vec![],
                });
                self.current_access_level = res.access_level.level;
                self.access_levels = vec![res.access_level];
                self.load_state();
            }
            AppRequestResponse::LoginError(_) => {}
            AppRequestResponse::LoginByKey(res) => {
                self.me = Some(UserInfo {
                    user: res.user,
                    key: res.key,
                    roles: vec![],
                });
                self.current_access_level = res.access_level.level;
                self.access_levels = vec![res.access_level];
                self.load_state();
            },
            AppRequestResponse::Register(_) => {}
            AppRequestResponse::RegisterError(_) => {}
            AppRequestResponse::NewPassword(_) => {
                self.load_access_levels(RequestDescription::default());
            }
            AppRequestResponse::LoadAccessLevels(mut r) => {
                r.array.sort_by(|a, b| a.level.cmp(&b.level));
                self.access_levels = r.array;
                self.access_levels.sort_by_key(|l| l.level);
            }
            AppRequestResponse::LoadUserRoles(res) => {
                if let Some(me) = &mut self.me {
                    me.roles = res.array;
                }
            }
            AppRequestResponse::LoadEvent(res) => {
                let event = res.value;
                self.clear_events_for_day(event.start.date());
                match self.events.iter_mut().find(|e| e.id == event.id) {
                    Some(e) => *e = event,
                    None => self.events.push(event),
                }
            }
            AppRequestResponse::LoadEventError(res) => match res {
                events::load::BadRequestResponse::NotFound => {
                    if let AppRequestInfo::LoadEvent(id) = info {
                        if let Some(ind) = self.events.iter().position(|e| e.id == id) {
                            self.clear_events_for_day(self.events[ind].start.date());
                            self.events.remove(ind);
                        }
                    }
                }
            },
            AppRequestResponse::LoadEvents(res) => {
                self.events = res.array;
                self.clear_events();
            }
            AppRequestResponse::InsertEvent(_) => {
                self.load_events(RequestDescription::default());
            }
            AppRequestResponse::UpdateEvent(_) => {
                if let AppRequestInfo::UpdateEvent(id) = info {
                    self.load_event(id, RequestDescription::default());
                }
            }
            AppRequestResponse::DeleteEvent(_) => {
                if let AppRequestInfo::DeleteEvent(id) = info {
                    if let Some(ind) = self.events.iter().position(|e| e.id == id) {
                        self.clear_events_for_day(self.events[ind].start.date());
                        self.events.remove(ind);
                    }
                }
            }
            AppRequestResponse::LoadEventTemplate(res) => {
                let template = res.value;
                match self
                    .event_templates
                    .iter_mut()
                    .find(|t| t.id == template.id)
                {
                    Some(t) => *t = template,
                    None => self.event_templates.push(template),
                }
                self.clear_events();
            }
            AppRequestResponse::LoadEventTemplateError(res) => match res {
                event_templates::load::BadRequestResponse::NotFound => {
                    if let AppRequestInfo::LoadEventTemplate(id) = info {
                        if let Some(ind) = self.event_templates.iter().position(|t| t.id == id) {
                            self.event_templates.remove(ind);
                        }
                    }
                }
            },
            AppRequestResponse::LoadEventTemplates(res) => {
                self.event_templates = res.array;
            }
            AppRequestResponse::InsertEventTemplate(_) => {
                self.load_event_templates(RequestDescription::default());
            }
            AppRequestResponse::UpdateEventTemplate(_) => {
                if let AppRequestInfo::UpdateEventTemplate(id) = info {
                    self.load_event_template(id, RequestDescription::default());
                }
            }
            AppRequestResponse::DeleteEventTemplate(_) => {
                if let AppRequestInfo::DeleteEventTemplate(id) = info {
                    if let Some(ind) = self.event_templates.iter().position(|e| e.id == id) {
                        self.event_templates.remove(ind);
                    }
                }
            }
            AppRequestResponse::LoadSchedule(res) => {
                let schedule = res.value;
                match self.schedules.iter_mut().find(|s| s.id == schedule.id) {
                    Some(s) => *s = schedule,
                    None => self.schedules.push(schedule),
                }
                self.clear_events();
            }
            AppRequestResponse::LoadScheduleError(res) => match res {
                schedules::load::BadRequestResponse::NotFound => {
                    if let AppRequestInfo::LoadSchedule(id) = info {
                        if let Some(ind) = self.schedules.iter().position(|t| t.id == id) {
                            self.schedules.remove(ind);
                            self.clear_events();
                        }
                    }
                }
            },
            AppRequestResponse::LoadSchedules(res) => {
                self.schedules = res.array;
                self.clear_events();
            }
            AppRequestResponse::InsertSchedule(_) => {
                self.load_schedules(RequestDescription::default());
            }
            AppRequestResponse::UpdateSchedule(_) => {
                if let AppRequestInfo::UpdateSchedule(id) = info {
                    self.load_schedule(id, RequestDescription::default());
                }
            }
            AppRequestResponse::DeleteSchedule(_) => {
                if let AppRequestInfo::DeleteSchedule(id) = info {
                    if let Some(ind) = self.schedules.iter().position(|s| s.id == id) {
                        self.schedules.remove(ind);
                        self.clear_events();
                    }
                }
            }
            AppRequestResponse::None => {}
            AppRequestResponse::Error(status, s) => {
                println!("smth went wrong: {status:?}=>{s:?}; info={info:?}");
            }
        }
    }

    pub fn poll(&mut self) -> Vec<RequestId> {
        let polled = self.connector.poll();
        polled.iter()
            .for_each(|&request_id| {
                let response = self.connector.get_response(request_id);
                let info: Option<AppRequestInfo> = self.connector.get_request_info(request_id);
                match (response, info) {
                    (Some(response), Some(info)) => self.parse_request(response, info),
                    _ => {}
                }
            });
        polled
    }
}
