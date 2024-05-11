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
        state.get_user_state_mut(user_id).access_levels.default_push_from_load_all(items);
    }

    fn push_bad_from_load_all(state: &mut State, user_id: TableId) {
        state.get_user_state_mut(user_id).access_levels.default_push_bad_from_load_all();
    }
}

impl TableItemLoadById for Event {
    const LOAD_BY_ID_PATH: &'static str = events::load::PATH;

    fn push_from_load_by_id(state: &mut State, user_id: TableId, id: TableId, item: Self) {
        state.clear_events(user_id);
        state.get_user_state_mut(user_id).events.default_push_from_load_by_id(id, item);
    }

    fn push_bad_from_load_by_id(
        state: &mut State, user_id: TableId,
        id: TableId,
        response: LoadByIdBadRequestResponse,
    ) {
        state.clear_events(user_id);
        state.get_user_state_mut(user_id).events.default_push_bad_from_load_by_id(id, response);
    }
}

impl TableItemLoadAll for Event {
    const LOAD_ALL_PATH: &'static str = events::load_array::PATH;

    fn push_from_load_all(state: &mut State, user_id: TableId, items: Vec<Self>) {
        state.clear_events(user_id);
        state.get_user_state_mut(user_id).events.default_push_from_load_all(items);
    }

    fn push_bad_from_load_all(state: &mut State, user_id: TableId) {
        state.clear_events(user_id);
        state.get_user_state_mut(user_id).events.default_push_bad_from_load_all();
    }
}

impl TableItemInsert for Event {
    type NewItem = NewEvent;
    const INSERT_PATH: &'static str = events::insert::PATH;

    fn push_from_insert(state: &mut State, user_id: TableId) {
        state.clear_events(user_id);
        state.get_user_state_mut(user_id).events.default_push_from_insert();
    }

    fn push_bad_from_insert(state: &mut State, user_id: TableId) {
        state.clear_events(user_id);
        state.get_user_state_mut(user_id).events.default_push_bad_from_insert();
    }
}

impl TableItemUpdate for Event {
    type UpdItem = UpdateEvent;
    const UPDATE_PATH: &'static str = events::update::PATH;

    fn push_from_update(state: &mut State, user_id: TableId, id: TableId) {
        state.clear_events(user_id);
        state.get_user_state_mut(user_id).events.default_push_from_update(id);
    }

    fn push_bad_from_update(state: &mut State, user_id: TableId, id: TableId, response: UpdateBadRequestResponse) {
        state.clear_events(user_id);
        state.get_user_state_mut(user_id).events.default_push_bad_from_update(id, response);
    }
}

impl TableItemDelete for Event {
    const DELETE_PATH: &'static str = events::delete::PATH;

    fn push_from_delete(state: &mut State, user_id: TableId, id: TableId) {
        state.clear_events(user_id);
        state.get_user_state_mut(user_id).events.default_push_from_delete(id);
    }

    fn push_bad_from_delete(state: &mut State, user_id: TableId, id: TableId, response: DeleteBadRequestResponse) {
        state.clear_events(user_id);
        state.get_user_state_mut(user_id).events.default_push_bad_from_delete(id, response);
    }
}

impl TableItemLoadById for EventTemplate {
    const LOAD_BY_ID_PATH: &'static str = event_templates::load::PATH;

    fn push_from_load_by_id(state: &mut State, user_id: TableId, id: TableId, item: Self) {
        state.get_user_state_mut(user_id).event_templates.default_push_from_load_by_id(id, item);
    }

    fn push_bad_from_load_by_id(
        state: &mut State, user_id: TableId,
        id: TableId,
        response: LoadByIdBadRequestResponse,
    ) {
        state.get_user_state_mut(user_id).event_templates.default_push_bad_from_load_by_id(id, response);
    }
}

impl TableItemLoadAll for EventTemplate {
    const LOAD_ALL_PATH: &'static str = event_templates::load_array::PATH;

    fn push_from_load_all(state: &mut State, user_id: TableId, items: Vec<Self>) {
        state.get_user_state_mut(user_id).event_templates.default_push_from_load_all(items);
    }

    fn push_bad_from_load_all(state: &mut State, user_id: TableId) {
        state.get_user_state_mut(user_id).event_templates.default_push_bad_from_load_all();
    }
}

impl TableItemInsert for EventTemplate {
    type NewItem = NewEventTemplate;
    const INSERT_PATH: &'static str = event_templates::insert::PATH;

    fn push_from_insert(state: &mut State, user_id: TableId) {
        state.get_user_state_mut(user_id).event_templates.default_push_from_insert();
    }

    fn push_bad_from_insert(state: &mut State, user_id: TableId) {
        state.get_user_state_mut(user_id).event_templates.default_push_bad_from_insert();
    }
}

impl TableItemUpdate for EventTemplate {
    type UpdItem = UpdateEventTemplate;
    const UPDATE_PATH: &'static str = event_templates::update::PATH;

    fn push_from_update(state: &mut State, user_id: TableId, id: TableId) {
        state.get_user_state_mut(user_id).event_templates.default_push_from_update(id);
    }

    fn push_bad_from_update(state: &mut State, user_id: TableId, id: TableId, response: UpdateBadRequestResponse) {
        state.get_user_state_mut(user_id).event_templates.default_push_bad_from_update(id, response);
    }
}

impl TableItemDelete for EventTemplate {
    const DELETE_PATH: &'static str = event_templates::delete::PATH;

    fn push_from_delete(state: &mut State, user_id: TableId, id: TableId) {
        state.get_user_state_mut(user_id).event_templates.default_push_from_delete(id);
    }

    fn push_bad_from_delete(state: &mut State, user_id: TableId, id: TableId, response: DeleteBadRequestResponse) {
        state.get_user_state_mut(user_id).event_templates.default_push_bad_from_delete(id, response);
    }
}

impl TableItemLoadById for Schedule {
    const LOAD_BY_ID_PATH: &'static str = schedules::load::PATH;

    fn push_from_load_by_id(state: &mut State, user_id: TableId, id: TableId, item: Self) {
        state.get_user_state_mut(user_id).schedules.default_push_from_load_by_id(id, item);
    }

    fn push_bad_from_load_by_id(
        state: &mut State, user_id: TableId,
        id: TableId,
        response: LoadByIdBadRequestResponse,
    ) {
        state.get_user_state_mut(user_id).schedules.default_push_bad_from_load_by_id(id, response);
    }
}

impl TableItemLoadAll for Schedule {
    const LOAD_ALL_PATH: &'static str = schedules::load_array::PATH;

    fn push_from_load_all(state: &mut State, user_id: TableId, items: Vec<Self>) {
        state.get_user_state_mut(user_id).schedules.default_push_from_load_all(items);
    }

    fn push_bad_from_load_all(state: &mut State, user_id: TableId) {
        state.get_user_state_mut(user_id).schedules.default_push_bad_from_load_all();
    }
}

impl TableItemInsert for Schedule {
    type NewItem = NewSchedule;
    const INSERT_PATH: &'static str = schedules::insert::PATH;

    fn push_from_insert(state: &mut State, user_id: TableId) {
        state.get_user_state_mut(user_id).schedules.default_push_from_insert();
    }

    fn push_bad_from_insert(state: &mut State, user_id: TableId) {
        state.get_user_state_mut(user_id).schedules.default_push_bad_from_insert();
    }
}

impl TableItemUpdate for Schedule {
    type UpdItem = UpdateSchedule;
    const UPDATE_PATH: &'static str = schedules::update::PATH;

    fn push_from_update(state: &mut State, user_id: TableId, id: TableId) {
        state.get_user_state_mut(user_id).schedules.default_push_from_update(id);
    }

    fn push_bad_from_update(state: &mut State, user_id: TableId, id: TableId, response: UpdateBadRequestResponse) {
        state.get_user_state_mut(user_id).schedules.default_push_bad_from_update(id, response);
    }
}

impl TableItemDelete for Schedule {
    const DELETE_PATH: &'static str = schedules::delete::PATH;

    fn push_from_delete(state: &mut State, user_id: TableId, id: TableId) {
        state.get_user_state_mut(user_id).schedules.default_push_from_delete(id);
    }

    fn push_bad_from_delete(state: &mut State, user_id: TableId, id: TableId, response: DeleteBadRequestResponse) {
        state.get_user_state_mut(user_id).schedules.default_push_bad_from_delete(id, response);
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

#[allow(unused_variables)]
impl TableItemLoadAll for User {
    const LOAD_ALL_PATH: &'static str = users::load_array::PATH;

    fn push_from_load_all(state: &mut State, user_id: TableId, items: Vec<Self>) {
        if state.me.is_admin() {
            state.admin_state.users.default_push_from_load_all(items);
        } else {
            println!("How did you get here? O.o");
        }
    }

    fn push_bad_from_load_all(state: &mut State, user_id: TableId) {
        if state.me.is_admin() {
            state.admin_state.users.default_push_bad_from_load_all();
        } else {
            println!("How did you get here? O.o");
        }
    }
}
