use std::cell::Ref;
use std::collections::HashMap;
use std::fmt::Debug;

use calendar_lib::api::auth::types::AccessLevel;
use calendar_lib::api::events::types::{EventVisibility, NewEvent};
use calendar_lib::api::user_state;
use calendar_lib::api::utils::{TableId, User};
use calendar_lib::api::{
    event_templates::types::EventTemplate, events::types::Event, schedules::types::Schedule,
};
use chrono::{Datelike, Duration, NaiveDate, NaiveDateTime};
use itertools::Itertools;
use serde::de::DeserializeOwned;

use crate::db::aliases::UserUtils;
use crate::tables::DbTable;

use super::db_connector::DbConnector;
use super::request::RequestIdentifier;
use super::requests_holder::RequestsHolder;
use super::state_table::StateTable;
use super::state_updater::StateUpdater;
use super::table_requests::TableInsertRequest;

pub trait RequestType where Self: 'static + Send {
    const URL: &'static str;
    const IS_AUTHORIZED: bool;
    const METHOD: reqwest::Method;

    type Query;
    type Body = ();
    type Response: DeserializeOwned;
    type BadResponse: DeserializeOwned = ();

    /// e.g. update request item.id
    type Info: 'static + Clone + Debug + Send = ();

    // TODO: Separate into different trait, move struct to request.rs
    fn push_to_state(response: Self::Response, info: Self::Info, state: &mut State);
    #[allow(unused_variables)]
    fn push_bad_to_state(response: Self::BadResponse, info: Self::Info, state: &mut State);
}

pub struct UserState {
    pub access_levels: StateTable<AccessLevel>,
    pub events: StateTable<Event>,
    pub event_templates: StateTable<EventTemplate>,
    pub schedules: StateTable<Schedule>,
}

impl UserState {
    pub(super) fn new() -> Self {
        Self {
            access_levels: StateTable::new(),
            events: StateTable::new(),
            schedules: StateTable::new(),
            event_templates: StateTable::new(),
        }
    }

    pub fn replace_data(&mut self, data: user_state::load::Response) {
        self.access_levels.get_table_mut().replace_all(data.access_levels);
        self.events.get_table_mut().replace_all(data.events);
        self.schedules.get_table_mut().replace_all(data.schedules);
        self.event_templates.get_table_mut().replace_all(data.event_templates);
    }

    pub fn accept_scheduled_event(
        &self,
        plan_id: TableId,
        date: NaiveDate,
    ) -> Option<RequestIdentifier<TableInsertRequest<Event>>> {
        self.schedules
            .get_table()
            .get()
            .iter()
            .find_map(|schedule| {
                schedule
                    .event_plans
                    .iter()
                    .find(|plan| plan.id == plan_id)
                    .and_then(|plan| {
                        self.event_templates
                            .get_table()
                            .get()
                            .iter()
                            .find(|template| schedule.template_id == template.id)
                            .map(|template| (plan, template))
                    })
            })
            .map(|(plan, template)| {
                let start = NaiveDateTime::new(date, plan.time);
                self.events.insert(NewEvent {
                    user_id: -1,
                    name: template.event_name.clone(),
                    description: template.event_description.clone(),
                    start,
                    end: start
                        .checked_add_signed(Duration::from_std(template.duration).unwrap())
                        .unwrap(),
                    access_level: template.access_level,
                    visibility: EventVisibility::HideAll,
                    plan_id: Some(plan_id),
                })
            })
    }
}

impl From<user_state::load::Response> for UserState {
    fn from(value: user_state::load::Response) -> Self {
        Self {
            access_levels: StateTable::from_vec(value.access_levels),
            events: StateTable::from_vec(value.events),
            schedules: StateTable::from_vec(value.schedules),
            event_templates: StateTable::from_vec(value.event_templates),
        }
    }
}

pub struct AdminState {
    pub users: StateTable<User>,
    pub users_data: HashMap<i32, UserState>,
}

impl AdminState {
    pub(super) fn new() -> Self {
        Self {
            users: StateTable::new(),
            users_data: HashMap::default(),
        }
    }
}

pub struct State {
    pub(super) db_connector: DbConnector,

    pub(super) me: User,
    pub(super) current_access_level: i32,

    pub user_state: UserState,
    pub admin_state: AdminState,

    /// Has both server and phantom events
    pub(super) events_per_day: HashMap<NaiveDate, Vec<Event>>,
}

impl State {
    pub fn new() -> Self {
        State {
            db_connector: DbConnector::new(),
            me: User::default(),
            current_access_level: -1,
            user_state: UserState::new(),
            admin_state: AdminState::new(),

            events_per_day: HashMap::new(),
        }
    }

    pub fn any_pending_requests(&self) -> bool {
        // This is not quite correct
        StateUpdater::get().any_checkers()
    }

    pub fn change_access_level(&mut self, new_access_level: i32) {
        self.current_access_level = new_access_level;
        self.clear_events();
    }

    pub fn get_access_level(&self) -> AccessLevel {
        let levels = self
            .user_state
            .access_levels
            .get_table()
            .get()
            .iter()
            .filter(|l| l.level == self.current_access_level)
            .collect_vec();
        if levels.len() == 0 {
            self.user_state
                .access_levels
                .get_table()
                .get()
                .last()
                .unwrap()
                .clone()
        } else if levels.len() == 1 {
            levels[0].clone()
        } else {
            (*levels.iter().find(|v| v.edit_rights).unwrap_or(&levels[0])).clone()
        }
    }

    pub fn try_get_me(&self) -> Option<&User> {
        if self.me.id > 0 {
            Some(&self.me)
        } else {
            None
        }
    }

    pub fn get_me(&self) -> &User {
        &self.me
    }

    pub fn get_user_state(&mut self, user_id: i32) -> &mut UserState {
        if self.me.is_admin() {
            self.admin_state
                .users_data
                .entry(user_id)
                .or_insert_with(|| {
                    println!("Admin mode: New user state created: {user_id}");
                    UserState::new()
                })
        } else {
            &mut self.user_state
        }
    }

    pub fn get_response<'a, T: RequestType>(
        &'a self,
        identifier: &RequestIdentifier<T>,
    ) -> Option<Result<Ref<'a, T::Response>, Ref<'a, T::BadResponse>>> {
        self.db_connector
            .convert_response::<T::Response, T::BadResponse>(identifier.id);
        self.db_connector
            .get_response::<T::Response, T::BadResponse>(identifier.id)
            .and_then(|r| r.ok())
    }

    pub fn find_response_by_type<'a, T: RequestType>(
        &'a self,
    ) -> Option<Result<Ref<'a, T::Response>, Ref<'a, T::BadResponse>>> {
        self.db_connector
            .find_response_by_type::<T::Response, T::BadResponse>()
    }

    pub fn take_response<T: RequestType>(
        &mut self,
        identifier: &RequestIdentifier<T>,
    ) -> Option<Result<Box<T::Response>, Box<T::BadResponse>>> {
        self.db_connector
            .convert_response::<T::Response, T::BadResponse>(identifier.id);
        self.db_connector
            .take_response::<T::Response, T::BadResponse>(identifier.id)
            .and_then(|r| r.ok())
    }

    fn send_requests(&mut self) {
        let requests = RequestsHolder::get().take();
        requests.into_iter().for_each(|request| {
            self.db_connector.request(request);
        });
    }

    pub fn update(&mut self) {
        StateUpdater::get().update(self);
        self.db_connector.pull();
        self.send_requests();
    }
}

impl State {
    pub fn clear_events_for_day(&mut self, date: NaiveDate) {
        self.events_per_day.remove(&date);
    }
    pub fn clear_events(&mut self) {
        self.events_per_day.clear();
    }

    pub(super) fn generate_phantom_events(&self, date: NaiveDate) -> Vec<Event> {
        let event_exists = |plan_id: i32| {
            self.user_state
                .events
                .get_table()
                .get()
                .iter()
                .any(|e| e.plan_id == Some(plan_id) && e.start.date() == date)
        };

        let level = self.get_access_level().level;
        self.user_state
            .schedules
            .get_table()
            .get()
            .iter()
            .filter(move |s| s.access_level <= level)
            .flat_map(|schedule| {
                match self
                    .user_state
                    .event_templates
                    .get_table()
                    .get()
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
                self.user_state
                    .events
                    .get_table()
                    .get()
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

    pub fn get_events_for_date(&self, date: NaiveDate) -> &[Event] {
        self.events_per_day.get(&date).unwrap()
    }
    pub fn get_prepared_events_for_date(&mut self, date: NaiveDate) -> &[Event] {
        self.prepare_date(date);
        self.get_events_for_date(date)
    }
}
