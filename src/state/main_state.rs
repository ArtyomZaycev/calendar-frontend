use std::{cell::Ref, collections::HashMap};

use calendar_lib::api::{
    auth::types::AccessLevel,
    events::types::{Event, EventVisibility},
    sharing::SharedPermissions,
    utils::{TableId, User},
};

use chrono::{Datelike, NaiveDate, NaiveDateTime};
use itertools::Itertools;

use crate::{db::aliases::UserUtils, tables::DbTable};

use super::{
    db_connector::DbConnector,
    request::{RequestIdentifier, RequestType},
    requests_holder::RequestsHolder,
    shared_state::SharedUserState,
    state_updater::StateUpdater,
};

pub use super::{admin_state::AdminState, user_state::UserState};

pub struct State {
    pub(super) db_connector: DbConnector,

    pub(super) me: User,
    pub(super) current_access_level: i32,

    pub user_state: UserState,
    pub shared_states: Vec<SharedUserState>,
    pub admin_state: AdminState,

    /// Has both server and phantom events
    pub(super) events_per_day: HashMap<NaiveDate, Vec<Event>>,
    pub(super) events_per_day_user_id: TableId,
}

impl State {
    pub fn new() -> Self {
        State {
            db_connector: DbConnector::new(),
            me: User::default(),
            current_access_level: -1,
            user_state: UserState::new(-1),
            shared_states: Vec::new(),
            admin_state: AdminState::new(),

            events_per_day: HashMap::new(),
            events_per_day_user_id: -1,
        }
    }

    pub fn any_pending_requests(&self) -> bool {
        // This is not quite correct
        StateUpdater::get().any_checkers()
    }

    pub fn change_access_level(&mut self, new_access_level: i32) {
        self.current_access_level = new_access_level;
        self.clear_events(-1);
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

    /// Can return user state instead of requested
    pub fn get_user_state(&self, user_id: i32) -> &UserState {
        self.try_get_user_state(user_id).unwrap_or(&self.user_state)
    }

    pub fn try_get_user_state(&self, user_id: i32) -> Option<&UserState> {
        if self.me.is_admin() {
            self.admin_state.users_data.get(&user_id)
        } else {
            if user_id == self.me.id {
                Some(&self.user_state)
            } else {
                self.shared_states
                    .iter()
                    .find_map(|state| (state.user.id == user_id).then_some(&state.state))
            }
        }
    }

    pub fn get_user_state_mut(&mut self, user_id: i32) -> &mut UserState {
        if self.me.is_admin() {
            self.admin_state
                .users_data
                .entry(user_id)
                .or_insert_with(|| {
                    println!("Admin mode: New user state created: {user_id}");
                    UserState::new(user_id)
                })
        } else {
            if user_id == self.me.id {
                &mut self.user_state
            } else {
                self.shared_states
                    .iter_mut()
                    .find_map(|state| (state.user.id == user_id).then_some(&mut state.state))
                    .unwrap()
            }
        }
    }

    pub fn get_user_permissions(&mut self, user_id: i32) -> SharedPermissions {
        if self.me.is_admin() {
            SharedPermissions::FULL
        } else {
            if user_id == self.me.id {
                SharedPermissions::FULL
            } else {
                self.shared_states
                    .iter_mut()
                    .find_map(|state| (state.user.id == user_id).then_some(state.permissions))
                    .unwrap()
            }
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
    pub fn clear_events_for_day(&mut self, user_id: TableId, date: NaiveDate) {
        if self.events_per_day_user_id == user_id {
            self.events_per_day.remove(&date);
        }
    }
    pub fn clear_events(&mut self, user_id: TableId) {
        println!("{:?}", self.me);
        println!("{}, {}", self.events_per_day_user_id, user_id);
        if self.events_per_day_user_id == user_id {
            self.events_per_day.clear();
        }
    }

    pub(super) fn generate_phantom_events(&self, user_id: TableId, date: NaiveDate) -> Vec<Event> {
        let user_state = self.get_user_state(user_id);

        let event_exists = |plan_id: i32| {
            user_state
                .events
                .get_table()
                .get()
                .iter()
                .any(|e| e.plan_id == Some(plan_id) && e.start.date() == date)
        };

        let level = self.get_access_level().level;
        user_state
            .schedules
            .get_table()
            .get()
            .iter()
            .filter(move |s| s.access_level <= level)
            .flat_map(|schedule| {
                match user_state
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

    pub fn prepare_date(&mut self, user_id: TableId, date: NaiveDate) {
        if self.events_per_day_user_id != user_id {
            self.events_per_day_user_id = user_id;
            self.events_per_day.clear();
        }

        if !self.events_per_day.contains_key(&date) {
            let level = self.get_access_level().level;
            self.events_per_day.insert(date, {
                self.get_user_state(user_id)
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
                    .chain(self.generate_phantom_events(user_id, date))
                    .sorted_by_key(|v| v.start)
                    .collect()
            });
        }
    }

    pub fn get_events_for_date(&self, date: NaiveDate) -> &[Event] {
        self.events_per_day.get(&date).unwrap()
    }
}
