use chrono::NaiveDate;
use derive_is_enum_variant::is_enum_variant;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Deserialize, Serialize, is_enum_variant)]
pub(super) enum AppView {
    Calendar(CalendarView),
    AdminPanel,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, is_enum_variant)]
pub(super) enum CalendarView {
    Events(EventsView),
    Schedules,
    EventTemplates,
}

impl Into<AppView> for CalendarView {
    fn into(self) -> AppView {
        AppView::Calendar(self)
    }
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, is_enum_variant)]
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
