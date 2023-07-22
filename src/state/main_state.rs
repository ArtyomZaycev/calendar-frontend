use super::UserState;
use crate::db::table::DbTable;
use crate::requests::AppRequestResponse;
use crate::{
    config::Config,
    db::{
        aliases::*,
        connector::DbConnector,
        request::{RequestDescription, RequestId},
    },
    requests::{AppRequestInfo, AppRequestResponseInfo},
    state::*,
    ui::signal::{RequestSignal, StateSignal},
};
use calendar_lib::api::{auth::types::AccessLevel, events, schedules};
use chrono::{Datelike, Duration, NaiveDate, NaiveDateTime};
use itertools::Itertools;
use reqwest::{Method, RequestBuilder};

use std::collections::HashMap;

pub struct State {
    pub connector: DbConnector<AppRequestResponse, AppRequestInfo, AppRequestResponseInfo>,
    // TODO: Move to app?
    /// Has both server and phantom events
    pub(super) events_per_day: HashMap<NaiveDate, Vec<Event>>,
    pub(super) current_access_level: i32,

    pub(super) me: Option<UserInfo>,

    pub(super) user_state: UserState,
    pub(super) admin_state: Option<AdminState>,

    pub errors: Vec<()>,
}

impl State {
    pub fn new(config: &Config) -> Self {
        Self {
            connector: DbConnector::new(config),
            events_per_day: HashMap::new(),
            current_access_level: -1,
            me: None,
            user_state: UserState::new(),
            admin_state: None,
            errors: Vec::default(),
        }
    }

    pub(super) fn clear_events_for_day(&mut self, date: NaiveDate) {
        self.events_per_day.remove(&date);
    }
    pub(super) fn clear_events(&mut self) {
        self.events_per_day.clear();
    }

    pub(super) fn generate_phantom_events(&self, date: NaiveDate) -> Vec<Event> {
        let event_exists = |plan_id: i32| {
            self.get_events()
                .iter()
                .any(|e| e.plan_id == Some(plan_id) && e.start.date() == date)
        };

        let level = self.get_access_level().level;
        self.get_schedules()
            .iter()
            .filter(move |s| s.access_level <= level)
            .flat_map(|schedule| {
                match self
                    .get_event_templates()
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
                self.get_events()
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
            .get_access_levels()
            .iter()
            .filter(|l| l.level == self.current_access_level)
            .collect_vec();
        if levels.len() == 0 {
            self.get_access_levels().last().unwrap().clone()
        } else if levels.len() == 1 {
            levels[0].clone()
        } else {
            (*levels.iter().find(|v| v.edit_rights).unwrap_or(&levels[0])).clone()
        }
    }

    pub fn get_me(&self) -> Option<&UserInfo> {
        self.me.as_ref()
    }
    pub fn get_access_levels(&self) -> &Vec<AccessLevel> {
        self.user_state.get_access_levels()
    }
    pub fn get_event_templates(&self) -> &Vec<EventTemplate> {
        self.user_state.event_templates.get()
    }
    pub fn get_events(&self) -> &Vec<Event> {
        self.user_state.events.get()
    }
    pub fn get_schedules(&self) -> &Vec<Schedule> {
        self.user_state.schedules.get()
    }

    pub(super) fn get_access_levels_mut(&mut self) -> &mut Vec<AccessLevel> {
        self.user_state.get_access_levels_mut()
    }
    pub(super) fn get_event_templates_mut(&mut self) -> &mut Vec<EventTemplate> {
        self.user_state.event_templates.get_mut()
    }
    pub(super) fn get_events_mut(&mut self) -> &mut Vec<Event> {
        self.user_state.events.get_mut()
    }
    pub(super) fn get_schedules_mut(&mut self) -> &mut Vec<Schedule> {
        self.user_state.schedules.get_mut()
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
                    if let Some((schedule, plan)) =
                        self.get_schedules().iter().find_map(|schedule| {
                            schedule
                                .event_plans
                                .iter()
                                .find(|plan| plan.id == plan_id)
                                .map(|plan| (schedule, plan))
                        })
                    {
                        if let Some(template) = self
                            .get_event_templates()
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
            self.connector.make_request(method, op).bearer_auth(&me.jwt)
        } else {
            panic!()
        }
    }
}

impl State {
    pub(super) fn load_state(&mut self) {
        self.load_user_state(
            self.get_me()
                .clone()
                .map(|me| me.user.id)
                .unwrap_or_default(),
            RequestDescription::default(),
        );
        /*
        self.load_access_levels(RequestDescription::default());
        self.load_events(RequestDescription::default());
        self.load_event_templates(RequestDescription::default());
        self.load_schedules(RequestDescription::default());
         */

        if let Some(me) = self.get_me() {
            if me.is_admin() {
                self.load_admin_state();
            }
        }
    }

    pub(super) fn clear_state(&mut self) {
        self.me = None;
        self.events_per_day.clear();
        self.current_access_level = -1;
        self.user_state.clear();
        self.admin_state = None;
    }

    pub(super) fn load_admin_state(&mut self) {
        self.admin_state = Some(AdminState::new());
        self.load_user_ids(RequestDescription::default());
    }

    pub(super) fn parse_request(&mut self, response: AppRequestResponse, info: AppRequestInfo) {
        match response {
            AppRequestResponse::Login(res) => {
                self.me = Some(UserInfo {
                    user: res.user,
                    jwt: res.jwt,
                });
                self.current_access_level = res.access_level.level;
                self.user_state.access_levels = vec![res.access_level];
                self.load_state();
            }
            AppRequestResponse::LoginError(_) => {}
            AppRequestResponse::LoginByKey(res) => {
                self.me = Some(UserInfo {
                    user: res.user,
                    jwt: res.jwt,
                });
                self.current_access_level = res.access_level.level;
                self.user_state.access_levels = vec![res.access_level];
                self.load_state();
            }
            AppRequestResponse::Register(_) => {}
            AppRequestResponse::RegisterError(_) => {}
            AppRequestResponse::NewPassword(_) => {
                self.load_access_levels(RequestDescription::default());
            }
            AppRequestResponse::LoadUserIds(res) => {
                if let Some(admin_state) = &mut self.admin_state {
                    admin_state.parse_load_user_ids(res);
                }
            }
            AppRequestResponse::LoadUser(res) => {
                if let Some(admin_state) = &mut self.admin_state {
                    admin_state.parse_load_user(res);
                }
            }
            AppRequestResponse::LoadUserError(res) => {
                if let AppRequestInfo::LoadUser(id) = info {
                    if let Some(admin_state) = &mut self.admin_state {
                        admin_state.parse_load_user_error(id, res);
                    }
                }
            }
            AppRequestResponse::LoadUsers(res) => {
                if let Some(admin_state) = &mut self.admin_state {
                    admin_state.parse_load_users(res);
                }
            }
            AppRequestResponse::LoadUserState(res) => {
                if let AppRequestInfo::LoadUserState { user_id } = info {
                    if self
                        .get_me()
                        .clone()
                        .map_or(false, |me| me.user.id == user_id)
                    {
                        *self.get_access_levels_mut() = res.access_levels;
                        *self.get_events_mut() = res.events;
                        *self.get_event_templates_mut() = res.event_templates;
                        *self.get_schedules_mut() = res.schedules;
                        self.clear_events();
                    } else {
                        if let Some(admin_state) = &mut self.admin_state {
                            admin_state.parse_load_state(user_id, res);
                        }
                    }
                }
            }
            AppRequestResponse::LoadUserStateError(res) => {
                if let AppRequestInfo::LoadUserState { user_id } = info {
                    if let Some(admin_state) = &mut self.admin_state {
                        admin_state.parse_load_state_error(user_id, res);
                    }
                }
            }
            AppRequestResponse::LoadAccessLevels(mut r) => {
                r.array.sort_by(|a, b| a.level.cmp(&b.level));
                self.user_state.access_levels = r.array;
                self.user_state.access_levels.sort_by_key(|l| l.level);
            }
            AppRequestResponse::LoadUserRoles(res) => {
                if let Some(me) = &mut self.me {
                    me.user.roles = res.array;
                }
            }
            AppRequestResponse::LoadEvent(res) => {
                let event = res.value;
                self.clear_events_for_day(event.start.date());
                match self.get_events_mut().iter_mut().find(|e| e.id == event.id) {
                    Some(e) => *e = event,
                    None => self.get_events_mut().push(event),
                }
            }
            AppRequestResponse::LoadEventError(res) => match res {
                events::load::BadRequestResponse::NotFound => {
                    if let AppRequestInfo::LoadEvent(id) = info {
                        if let Some(ind) = self.get_events().iter().position(|e| e.id == id) {
                            self.clear_events_for_day(self.get_events()[ind].start.date());
                            self.get_events_mut().remove(ind);
                        }
                    }
                }
            },
            AppRequestResponse::LoadEvents(res) => {
                *self.get_events_mut() = res.array;
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
                    if let Some(ind) = self.get_events().iter().position(|e| e.id == id) {
                        self.clear_events_for_day(self.get_events()[ind].start.date());
                        self.get_events_mut().remove(ind);
                    }
                }
            }
            AppRequestResponse::LoadEventTemplate(res) => {
                let template = res.value;
                match self
                    .get_event_templates_mut()
                    .iter_mut()
                    .find(|t| t.id == template.id)
                {
                    Some(t) => *t = template,
                    None => self.get_event_templates_mut().push(template),
                }
                self.clear_events();
            }
            AppRequestResponse::LoadEventTemplateError(res) => match res {
                event_templates::load::BadRequestResponse::NotFound => {
                    if let AppRequestInfo::LoadEventTemplate(id) = info {
                        if let Some(ind) =
                            self.get_event_templates().iter().position(|t| t.id == id)
                        {
                            self.get_event_templates_mut().remove(ind);
                        }
                    }
                }
            },
            AppRequestResponse::LoadEventTemplates(res) => {
                *self.get_event_templates_mut() = res.array;
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
                    if let Some(ind) = self.get_event_templates().iter().position(|e| e.id == id) {
                        self.get_event_templates_mut().remove(ind);
                    }
                }
            }
            AppRequestResponse::LoadSchedule(res) => {
                let schedule = res.value;
                match self
                    .get_schedules_mut()
                    .iter_mut()
                    .find(|s| s.id == schedule.id)
                {
                    Some(s) => *s = schedule,
                    None => self.get_schedules_mut().push(schedule),
                }
                self.clear_events();
            }
            AppRequestResponse::LoadScheduleError(res) => match res {
                schedules::load::BadRequestResponse::NotFound => {
                    if let AppRequestInfo::LoadSchedule(id) = info {
                        if let Some(ind) = self.get_schedules().iter().position(|t| t.id == id) {
                            self.get_schedules_mut().remove(ind);
                            self.clear_events();
                        }
                    }
                }
            },
            AppRequestResponse::LoadSchedules(res) => {
                *self.get_schedules_mut() = res.array;
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
                    if let Some(ind) = self.get_schedules().iter().position(|s| s.id == id) {
                        self.get_schedules_mut().remove(ind);
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
        polled.iter().for_each(|&request_id| {
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
