use calendar_lib::api::{
    auth::types::AccessLevel,
    event_templates::{
        self,
        types::{EventTemplate, NewEventTemplate, UpdateEventTemplate},
    },
    events::{
        self,
        types::{Event, NewEvent, UpdateEvent},
    },
    permissions::{
        self,
        types::{GrantedPermission, NewGrantedPermission, UpdateGrantedPermission},
    },
    roles::{self, types::Role},
    schedules::{
        self,
        types::{NewSchedule, Schedule, UpdateSchedule},
    },
    users,
    utils::*,
};

use crate::db::aliases::UserUtils;

use super::{
    table_requests::{
        TableItemDelete, TableItemInsert, TableItemLoadAll, TableItemLoadById, TableItemUpdate,
    },
    State,
};

impl TableItemLoadAll for AccessLevel {
    const LOAD_ALL_PATH: &'static str = "auth/load_access_levels";

    fn push_from_load_all(state: &mut State, user_id: TableId, items: Vec<Self>) {
        state
            .get_user_state_mut(user_id)
            .access_levels
            .default_push_from_load_all(items);
    }

    fn push_bad_from_load_all(state: &mut State, user_id: TableId) {
        state
            .get_user_state_mut(user_id)
            .access_levels
            .default_push_bad_from_load_all();
    }
}

impl TableItemLoadById for Event {
    const LOAD_BY_ID_PATH: &'static str = events::load::PATH;

    fn push_from_load_by_id(state: &mut State, user_id: TableId, id: TableId, item: Self) {
        state.clear_events(user_id);
        state
            .get_user_state_mut(user_id)
            .events
            .default_push_from_load_by_id(id, item);
    }

    fn push_bad_from_load_by_id(
        state: &mut State,
        user_id: TableId,
        id: TableId,
        response: LoadByIdBadRequestResponse,
    ) {
        state.clear_events(user_id);
        state
            .get_user_state_mut(user_id)
            .events
            .default_push_bad_from_load_by_id(id, response);
    }
}

impl TableItemLoadAll for Event {
    const LOAD_ALL_PATH: &'static str = events::load_array::PATH;

    fn push_from_load_all(state: &mut State, user_id: TableId, items: Vec<Self>) {
        state.clear_events(user_id);
        state
            .get_user_state_mut(user_id)
            .events
            .default_push_from_load_all(items);
    }

    fn push_bad_from_load_all(state: &mut State, user_id: TableId) {
        state.clear_events(user_id);
        state
            .get_user_state_mut(user_id)
            .events
            .default_push_bad_from_load_all();
    }
}

impl TableItemInsert for Event {
    type NewItem = NewEvent;
    const INSERT_PATH: &'static str = events::insert::PATH;

    fn push_from_insert(state: &mut State, user_id: TableId) {
        state.clear_events(user_id);
        state
            .get_user_state_mut(user_id)
            .events
            .default_push_from_insert();
    }

    fn push_bad_from_insert(state: &mut State, user_id: TableId, _: Self::BadResponse) {
        state.clear_events(user_id);
        state
            .get_user_state_mut(user_id)
            .events
            .default_push_bad_from_insert();
    }
}

impl TableItemUpdate for Event {
    type UpdItem = UpdateEvent;
    const UPDATE_PATH: &'static str = events::update::PATH;

    fn push_from_update(state: &mut State, user_id: TableId, id: TableId) {
        state.clear_events(user_id);
        state
            .get_user_state_mut(user_id)
            .events
            .default_push_from_update(id);
    }

    fn push_bad_from_update(
        state: &mut State,
        user_id: TableId,
        id: TableId,
        response: UpdateBadRequestResponse,
    ) {
        state.clear_events(user_id);
        state
            .get_user_state_mut(user_id)
            .events
            .default_push_bad_from_update(id, response);
    }
}

impl TableItemDelete for Event {
    const DELETE_PATH: &'static str = events::delete::PATH;

    fn push_from_delete(state: &mut State, user_id: TableId, id: TableId) {
        state.clear_events(user_id);
        state
            .get_user_state_mut(user_id)
            .events
            .default_push_from_delete(id);
    }

    fn push_bad_from_delete(
        state: &mut State,
        user_id: TableId,
        id: TableId,
        response: DeleteBadRequestResponse,
    ) {
        state.clear_events(user_id);
        state
            .get_user_state_mut(user_id)
            .events
            .default_push_bad_from_delete(id, response);
    }
}

impl TableItemLoadById for EventTemplate {
    const LOAD_BY_ID_PATH: &'static str = event_templates::load::PATH;

    fn push_from_load_by_id(state: &mut State, user_id: TableId, id: TableId, item: Self) {
        state.clear_events(user_id);
        state
            .get_user_state_mut(user_id)
            .event_templates
            .default_push_from_load_by_id(id, item);
    }

    fn push_bad_from_load_by_id(
        state: &mut State,
        user_id: TableId,
        id: TableId,
        response: LoadByIdBadRequestResponse,
    ) {
        state.clear_events(user_id);
        state
            .get_user_state_mut(user_id)
            .event_templates
            .default_push_bad_from_load_by_id(id, response);
    }
}

impl TableItemLoadAll for EventTemplate {
    const LOAD_ALL_PATH: &'static str = event_templates::load_array::PATH;

    fn push_from_load_all(state: &mut State, user_id: TableId, items: Vec<Self>) {
        state.clear_events(user_id);
        state
            .get_user_state_mut(user_id)
            .event_templates
            .default_push_from_load_all(items);
    }

    fn push_bad_from_load_all(state: &mut State, user_id: TableId) {
        state.clear_events(user_id);
        state
            .get_user_state_mut(user_id)
            .event_templates
            .default_push_bad_from_load_all();
    }
}

impl TableItemInsert for EventTemplate {
    type NewItem = NewEventTemplate;
    const INSERT_PATH: &'static str = event_templates::insert::PATH;

    fn push_from_insert(state: &mut State, user_id: TableId) {
        state.clear_events(user_id);
        state
            .get_user_state_mut(user_id)
            .event_templates
            .default_push_from_insert();
    }

    fn push_bad_from_insert(state: &mut State, user_id: TableId, _: Self::BadResponse) {
        state.clear_events(user_id);
        state
            .get_user_state_mut(user_id)
            .event_templates
            .default_push_bad_from_insert();
    }
}

impl TableItemUpdate for EventTemplate {
    type UpdItem = UpdateEventTemplate;
    const UPDATE_PATH: &'static str = event_templates::update::PATH;

    fn push_from_update(state: &mut State, user_id: TableId, id: TableId) {
        state.clear_events(user_id);
        state
            .get_user_state_mut(user_id)
            .event_templates
            .default_push_from_update(id);
    }

    fn push_bad_from_update(
        state: &mut State,
        user_id: TableId,
        id: TableId,
        response: UpdateBadRequestResponse,
    ) {
        state.clear_events(user_id);
        state
            .get_user_state_mut(user_id)
            .event_templates
            .default_push_bad_from_update(id, response);
    }
}

impl TableItemDelete for EventTemplate {
    const DELETE_PATH: &'static str = event_templates::delete::PATH;

    fn push_from_delete(state: &mut State, user_id: TableId, id: TableId) {
        state.clear_events(user_id);
        state
            .get_user_state_mut(user_id)
            .event_templates
            .default_push_from_delete(id);
    }

    fn push_bad_from_delete(
        state: &mut State,
        user_id: TableId,
        id: TableId,
        response: DeleteBadRequestResponse,
    ) {
        state.clear_events(user_id);
        state
            .get_user_state_mut(user_id)
            .event_templates
            .default_push_bad_from_delete(id, response);
    }
}

impl TableItemLoadById for Schedule {
    const LOAD_BY_ID_PATH: &'static str = schedules::load::PATH;

    fn push_from_load_by_id(state: &mut State, user_id: TableId, id: TableId, item: Self) {
        state.clear_events(user_id);
        state
            .get_user_state_mut(user_id)
            .schedules
            .default_push_from_load_by_id(id, item);
    }

    fn push_bad_from_load_by_id(
        state: &mut State,
        user_id: TableId,
        id: TableId,
        response: LoadByIdBadRequestResponse,
    ) {
        state.clear_events(user_id);
        state
            .get_user_state_mut(user_id)
            .schedules
            .default_push_bad_from_load_by_id(id, response);
    }
}

impl TableItemLoadAll for Schedule {
    const LOAD_ALL_PATH: &'static str = schedules::load_array::PATH;

    fn push_from_load_all(state: &mut State, user_id: TableId, items: Vec<Self>) {
        state.clear_events(user_id);
        state
            .get_user_state_mut(user_id)
            .schedules
            .default_push_from_load_all(items);
    }

    fn push_bad_from_load_all(state: &mut State, user_id: TableId) {
        state.clear_events(user_id);
        state
            .get_user_state_mut(user_id)
            .schedules
            .default_push_bad_from_load_all();
    }
}

impl TableItemInsert for Schedule {
    type NewItem = NewSchedule;
    const INSERT_PATH: &'static str = schedules::insert::PATH;

    fn push_from_insert(state: &mut State, user_id: TableId) {
        state.clear_events(user_id);
        state
            .get_user_state_mut(user_id)
            .schedules
            .default_push_from_insert();
    }

    fn push_bad_from_insert(state: &mut State, user_id: TableId, _: Self::BadResponse) {
        state.clear_events(user_id);
        state
            .get_user_state_mut(user_id)
            .schedules
            .default_push_bad_from_insert();
    }
}

impl TableItemUpdate for Schedule {
    type UpdItem = UpdateSchedule;
    const UPDATE_PATH: &'static str = schedules::update::PATH;

    fn push_from_update(state: &mut State, user_id: TableId, id: TableId) {
        state.clear_events(user_id);
        state
            .get_user_state_mut(user_id)
            .schedules
            .default_push_from_update(id);
    }

    fn push_bad_from_update(
        state: &mut State,
        user_id: TableId,
        id: TableId,
        response: UpdateBadRequestResponse,
    ) {
        state.clear_events(user_id);
        state
            .get_user_state_mut(user_id)
            .schedules
            .default_push_bad_from_update(id, response);
    }
}

impl TableItemDelete for Schedule {
    const DELETE_PATH: &'static str = schedules::delete::PATH;

    fn push_from_delete(state: &mut State, user_id: TableId, id: TableId) {
        state.clear_events(user_id);
        state
            .get_user_state_mut(user_id)
            .schedules
            .default_push_from_delete(id);
    }

    fn push_bad_from_delete(
        state: &mut State,
        user_id: TableId,
        id: TableId,
        response: DeleteBadRequestResponse,
    ) {
        state.clear_events(user_id);
        state
            .get_user_state_mut(user_id)
            .schedules
            .default_push_bad_from_delete(id, response);
    }
}

impl TableItemLoadAll for Role {
    const LOAD_ALL_PATH: &'static str = roles::load_array::PATH;

    fn push_from_load_all(state: &mut State, user_id: TableId, items: Vec<Self>) {
        if state.me.id == user_id {
            state.me.roles = items;
        }
    }

    fn push_bad_from_load_all(state: &mut State, user_id: TableId) {
        if state.me.id == user_id {
            state.me.roles = Vec::new();
        }
    }
}

impl TableItemLoadAll for User {
    const LOAD_ALL_PATH: &'static str = users::load_array::PATH;

    fn push_from_load_all(state: &mut State, user_id: TableId, items: Vec<Self>) {
        if state.me.is_admin() {
            state
                .admin_state
                .users
                .default_push_from_load_all(items.clone());
        } else {
            state
                .get_user_state_mut(user_id)
                .users
                .default_push_from_load_all(items);
            state.populate_granted_user_states(user_id);
        }
    }

    fn push_bad_from_load_all(state: &mut State, user_id: TableId) {
        if state.me.is_admin() {
            state.admin_state.users.default_push_bad_from_load_all();
        } else {
            state
                .get_user_state_mut(user_id)
                .users
                .default_push_bad_from_load_all();
        }
    }
}

impl TableItemLoadById for GrantedPermission {
    const LOAD_BY_ID_PATH: &'static str = permissions::load::PATH;

    fn push_from_load_by_id(state: &mut State, user_id: TableId, id: TableId, item: Self) {
        state.clear_events(user_id);
        state
            .get_user_state_mut(user_id)
            .granted_permissions
            .default_push_from_load_by_id(id, item);
        state.populate_granted_user_states(user_id);
        state.get_user_state(user_id).users.load_all();
    }

    fn push_bad_from_load_by_id(
        state: &mut State,
        user_id: TableId,
        id: TableId,
        response: LoadByIdBadRequestResponse,
    ) {
        state.clear_events(user_id);
        state
            .get_user_state_mut(user_id)
            .granted_permissions
            .default_push_bad_from_load_by_id(id, response);
    }
}

impl TableItemLoadAll for GrantedPermission {
    const LOAD_ALL_PATH: &'static str = permissions::load_array::PATH;

    fn push_from_load_all(state: &mut State, user_id: TableId, items: Vec<Self>) {
        state.clear_events(user_id);
        state
            .get_user_state_mut(user_id)
            .granted_permissions
            .default_push_from_load_all(items);
        state.populate_granted_user_states(user_id);
        state.get_user_state(user_id).users.load_all();
    }

    fn push_bad_from_load_all(state: &mut State, user_id: TableId) {
        state.clear_events(user_id);
        state
            .get_user_state_mut(user_id)
            .granted_permissions
            .default_push_bad_from_load_all();
    }
}

#[allow(unused_variables)]
impl TableItemInsert for GrantedPermission {
    type NewItem = NewGrantedPermission;

    const INSERT_PATH: &'static str = permissions::insert::PATH;

    type BadResponse = permissions::insert::BadRequestResponse;
    // Email
    type Info = String;

    fn push_from_insert(state: &mut State, user_id: TableId) {
        state.clear_events(user_id);
        state
            .get_user_state_mut(user_id)
            .granted_permissions
            .default_push_from_insert();
    }

    fn push_bad_from_insert(state: &mut State, user_id: TableId, response: Self::BadResponse) {
        state.clear_events(user_id);
        state
            .get_user_state_mut(user_id)
            .granted_permissions
            .default_push_bad_from_insert();
    }
}

#[allow(unused_variables)]
impl TableItemUpdate for GrantedPermission {
    type UpdItem = UpdateGrantedPermission;

    const UPDATE_PATH: &'static str = permissions::update::PATH;

    type BadResponse = permissions::update::BadRequestResponse;
    // Email
    type Info = String;

    fn push_from_update(state: &mut State, user_id: TableId, id: TableId) {
        state.clear_events(user_id);
        state
            .get_user_state_mut(user_id)
            .granted_permissions
            .default_push_from_update(id);
    }

    fn push_bad_from_update(
        state: &mut State,
        user_id: TableId,
        id: TableId,
        response: Self::BadResponse,
    ) {
        state.clear_events(user_id);
        state
            .get_user_state_mut(user_id)
            .granted_permissions
            .default_push_bad_from_update(id, UpdateBadRequestResponse::NotFound);
    }
}

impl TableItemDelete for GrantedPermission {
    const DELETE_PATH: &'static str = permissions::delete::PATH;

    fn push_from_delete(state: &mut State, user_id: TableId, id: TableId) {
        state.clear_events(user_id);
        state
            .get_user_state_mut(user_id)
            .granted_permissions
            .default_push_from_delete(id);
    }

    fn push_bad_from_delete(
        state: &mut State,
        user_id: TableId,
        id: TableId,
        response: DeleteBadRequestResponse,
    ) {
        state.clear_events(user_id);
        state
            .get_user_state_mut(user_id)
            .granted_permissions
            .default_push_bad_from_delete(id, response);
    }
}
