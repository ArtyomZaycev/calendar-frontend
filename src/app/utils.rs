use chrono::NaiveDate;
use derive_is_enum_variant::is_enum_variant;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, is_enum_variant)]
pub(super) enum EventsView {
    Month(NaiveDate),
    Week(NaiveDate),
    Day(NaiveDate),
    Days(NaiveDate),
}

#[derive(Debug, Clone, Deserialize, Serialize, is_enum_variant)]
pub(super) enum CalendarView {
    Events(EventsView),
    Schedules,
    EventTemplates,
}
