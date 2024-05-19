use std::{cell::Ref, collections::HashMap};

use calendar_lib::api::{
    events::types::{Event, EventVisibility},
    permissions::types::Permissions,
    utils::{TableId, User},
};

use chrono::{Datelike, NaiveDate, NaiveDateTime};
use itertools::Itertools;

use crate::{
    db::{
        aliases::UserUtils,
        db_connector::{DbConnector, DbConnectorData},
        request::RequestIdentifier,
    },
    tables::DbTable,
};

use super::{request::RequestType, shared_state::GrantedUserState, state_updater::StateUpdater};

pub use super::{admin_state::AdminState, user_state::UserState};

pub struct State {
    pub(super) db_connector: DbConnector,

    pub(super) me: User,

    pub user_state: UserState,
    pub granted_states: Vec<GrantedUserState>,
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
            user_state: UserState::new(-1),
            granted_states: Vec::new(),
            admin_state: AdminState::new(),

            events_per_day: HashMap::new(),
            events_per_day_user_id: -1,
        }
    }

    pub fn any_pending_requests(&self) -> bool {
        // This is not quite correct
        StateUpdater::get().any_checkers()
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

    /// Can return this user state instead of requested
    pub fn get_user_state(&self, user_id: i32) -> &UserState {
        match self.try_get_user_state(user_id) {
            Some(user_state) => user_state,
            None => {
                println!("get_user_state state {} not found", user_id);
                &self.user_state
            }
        }
    }

    /// Can return this user state instead of requested
    pub fn get_user_state_mut<'a>(&'a mut self, user_id: i32) -> &'a mut UserState {
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
                self.granted_states
                    .iter_mut()
                    .find_map(|state| (state.user.id == user_id).then_some(&mut state.state))
                    .unwrap_or(&mut self.user_state)
            }
        }
    }

    pub fn try_get_user_state(&self, user_id: i32) -> Option<&UserState> {
        if self.me.is_admin() {
            self.admin_state.users_data.get(&user_id)
        } else {
            if user_id == self.me.id {
                Some(&self.user_state)
            } else {
                self.granted_states
                    .iter()
                    .find_map(|state| (state.user.id == user_id).then_some(&state.state))
            }
        }
    }

    pub fn get_user_permissions(&self, user_id: i32) -> Permissions {
        if self.me.is_admin() {
            Permissions::FULL
        } else {
            if user_id == self.me.id {
                Permissions::FULL
            } else {
                self.granted_states
                    .iter()
                    .find_map(|state| (state.user.id == user_id).then_some(state.permissions))
                    .unwrap_or(Permissions::NONE)
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

    pub fn update(&mut self) {
        StateUpdater::get().update(self);
        self.db_connector.pull_responses();
        self.db_connector.send_requests();
    }
}

impl State {
    pub fn clear_events(&mut self, _user_id: TableId) {
        self.events_per_day.clear();
    }

    pub(super) fn generate_phantom_events(
        &self,
        user_id: TableId,
        access_level: i32,
        date: NaiveDate,
    ) -> Vec<Event> {
        let user_state = self.get_user_state(user_id);

        let event_exists = |plan_id: i32| {
            user_state
                .events
                .get_table()
                .get()
                .iter()
                .any(|e| e.plan_id == Some(plan_id) && e.start.date() == date)
        };

        user_state
            .schedules
            .get_table()
            .get()
            .iter()
            .filter(move |s| s.access_level <= access_level)
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

    pub fn prepare_date(&mut self, user_id: TableId, access_level: i32, date: NaiveDate) {
        if self.events_per_day_user_id != user_id {
            self.events_per_day_user_id = user_id;
            self.events_per_day.clear();
        }

        if !self.events_per_day.contains_key(&date) {
            self.events_per_day.insert(date, {
                self.get_user_state(user_id)
                    .events
                    .get_table()
                    .get()
                    .iter()
                    .filter(|e| e.start.date() == date)
                    .filter_map(move |e| {
                        if e.access_level <= access_level {
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
                    .chain(self.generate_phantom_events(user_id, access_level, date))
                    .sorted_by_key(|v| v.start)
                    .collect()
            });
        }
    }

    pub fn get_events_for_date(&self, date: NaiveDate) -> &[Event] {
        self.events_per_day.get(&date).unwrap()
    }
}

impl State {
    pub(super) fn on_logged_in(&mut self, user: User, jwt: String) {
        DbConnectorData::get().push_jwt(jwt);
        self.me = user;
        self.user_state.set_user_id(self.me.id);
        self.load_state();
    }

    pub(super) fn populate_granted_user_states(&mut self, user_id: TableId) {
        let new_given_permissions = self
            .get_user_state(user_id)
            .granted_permissions
            .get_table()
            .get()
            .iter()
            .filter(|gp| gp.receiver_user_id == user_id)
            .filter(|gp| {
                !self
                    .granted_states
                    .iter()
                    .any(|gs| gs.user.id == gp.giver_user_id)
            })
            .collect_vec();
        let mut new_states = new_given_permissions
            .into_iter()
            .map(|gp| {
                GrantedUserState::new(
                    User {
                        id: gp.giver_user_id,
                        ..User::default()
                    },
                    gp.permissions,
                )
            })
            .collect_vec();
        new_states.iter().for_each(|state| {
            state.state.load_state();
        });
        self.granted_states.append(&mut new_states);
        self.populate_granted_user_states_users(user_id);
    }

    pub(super) fn populate_granted_user_states_users(&mut self, user_id: TableId) {
        let users = self.get_user_state(user_id).users.get_table().get().clone();
        self.granted_states.iter_mut().for_each(|gs| {
            if let Some(user) = users.iter().find(|u| u.id == gs.user.id) {
                gs.user = user.clone();
            }
        })
    }
}
