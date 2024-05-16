use calendar_lib::api::{
    event_templates::types::EventTemplate, events::types::Event, schedules::types::Schedule,
    utils::User,
};
use derive_is_enum_variant::is_enum_variant;
use serde::{Deserialize, Serialize};

use crate::ui::table_view::TableView;

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, is_enum_variant)]
pub(super) enum AppView {
    Calendar(CalendarView),
    AdminPanel(AdminPanelView),
    ManageAccess(ManageAccessView),
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

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, is_enum_variant)]
pub(super) enum ManageAccessView {
    Sharing,
    AccessLevels,
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

impl Into<AppView> for ManageAccessView {
    fn into(self) -> AppView {
        AppView::ManageAccess(self)
    }
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, is_enum_variant)]
pub(super) enum EventsView {
    Month,
    Week,
    Day,
    Days,
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
