use calendar_lib::api::{
    auth::types::AccessLevel,
    event_templates::types::EventTemplate,
    events::types::{Event, EventVisibility, NewEvent},
    permissions::types::GrantedPermission,
    schedules::types::Schedule,
    user_state,
    utils::{TableId, User},
};

use chrono::{Duration, NaiveDate, NaiveDateTime};

use crate::{db::request::RequestIdentifier, tables::DbTable};

use super::{state_table::StateTable, table_requests::TableInsertRequest};

pub struct UserState {
    pub(super) user_id: TableId,

    pub users: StateTable<User>,
    pub access_levels: StateTable<AccessLevel>,
    pub events: StateTable<Event>,
    pub event_templates: StateTable<EventTemplate>,
    pub schedules: StateTable<Schedule>,
    pub granted_permissions: StateTable<GrantedPermission>,
}

impl UserState {
    pub(super) fn new(user_id: TableId) -> Self {
        let mut state = Self {
            user_id: -1,
            users: StateTable::new(),
            access_levels: StateTable::new(),
            events: StateTable::new(),
            schedules: StateTable::new(),
            event_templates: StateTable::new(),
            granted_permissions: StateTable::new(),
        };
        state.set_user_id(user_id);
        state
    }

    pub fn set_user_id(&mut self, user_id: TableId) {
        self.user_id = user_id;
        self.access_levels.set_user_id(user_id);
        self.events.set_user_id(user_id);
        self.event_templates.set_user_id(user_id);
        self.schedules.set_user_id(user_id);
        self.granted_permissions.set_user_id(user_id);
    }

    pub fn replace_data(&mut self, data: user_state::load::Response) {
        self.access_levels
            .get_table_mut()
            .replace_all(data.access_levels);
        self.events.get_table_mut().replace_all(data.events);
        self.schedules.get_table_mut().replace_all(data.schedules);
        self.event_templates
            .get_table_mut()
            .replace_all(data.event_templates);
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
