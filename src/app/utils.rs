use calendar_lib::api::{
    event_templates::types::EventTemplate, events::types::Event, schedules::types::Schedule,
    utils::User,
};
use chrono::NaiveDate;
use derive_is_enum_variant::is_enum_variant;
use serde::{Deserialize, Serialize};

use crate::ui::table_view::TableView;

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, is_enum_variant)]
pub(super) enum AppView {
    Calendar(CalendarView),
    AdminPanel(AdminPanelView),
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, is_enum_variant)]
pub(super) enum CalendarView {
    Events(EventsView),
    Schedules,
    EventTemplates,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, is_enum_variant)]
pub(super) enum AdminPanelView {
    Users {
        table: TableView<User>,
    },
    UserData {
        user_id: i32,
        view: AdminPanelUserDataView,
    },
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, is_enum_variant)]
pub(super) enum AdminPanelUserDataView {
    Events { table: TableView<Event> },
    EventTemplates { table: TableView<EventTemplate> },
    Schedules { table: TableView<Schedule> },
}

impl Into<AppView> for CalendarView {
    fn into(self) -> AppView {
        AppView::Calendar(self)
    }
}

impl Into<AppView> for AdminPanelView {
    fn into(self) -> AppView {
        AppView::AdminPanel(self)
    }
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, is_enum_variant)]
pub(super) enum EventsView {
    Month(NaiveDate),
    Week(NaiveDate),
    Day(NaiveDate),
    Days(NaiveDate),
}

impl Into<CalendarView> for EventsView {
    fn into(self) -> CalendarView {
        CalendarView::Events(self)
    }
}

impl Into<AppView> for EventsView {
    fn into(self) -> AppView {
        CalendarView::Events(self).into()
    }
}
