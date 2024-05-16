use calendar_lib::api::{auth::types::AccessLevel, events::types::EventVisibility};
use chrono::{Datelike, NaiveDate, Weekday};
use email_address::EmailAddress;
use itertools::Itertools;
use std::future::Future;

#[cfg(not(target_arch = "wasm32"))]
pub fn easy_spawn<F>(f: F)
where
    F: Future<Output = ()> + Send + 'static,
{
    tokio::spawn(f);
}

#[cfg(target_arch = "wasm32")]
pub fn easy_spawn<F>(f: F)
where
    F: Future<Output = ()> + 'static,
{
    wasm_bindgen_futures::spawn_local(f);
}

pub fn is_valid_email(email: &str) -> bool {
    EmailAddress::is_valid(email)
}
pub fn is_password_valid(password: &str) -> bool {
    password.len() <= 30
}
pub fn is_password_strong_enough(password: &str) -> bool {
    #[cfg(debug_assertions)]
    true || password.len() >= 4
}

pub fn get_first_month_day_date(date: &NaiveDate) -> NaiveDate {
    date.checked_sub_days(chrono::Days::new(date.day0() as u64))
        .unwrap()
}
pub fn get_monday(date: &NaiveDate) -> NaiveDate {
    date.checked_sub_days(chrono::Days::new(
        (date.weekday().num_days_from_monday()) as u64,
    ))
    .unwrap()
}

pub fn weekday_human_name(weekday: &Weekday) -> &'static str {
    match weekday {
        chrono::Weekday::Mon => "Monday",
        chrono::Weekday::Tue => "Tuesday",
        chrono::Weekday::Wed => "Wednesday",
        chrono::Weekday::Thu => "Thursday",
        chrono::Weekday::Fri => "Friday",
        chrono::Weekday::Sat => "Saturday",
        chrono::Weekday::Sun => "Sunday",
    }
}

pub fn weekday_human_name_short(weekday: &Weekday) -> &'static str {
    match weekday {
        chrono::Weekday::Mon => "Mon",
        chrono::Weekday::Tue => "Tue",
        chrono::Weekday::Wed => "Wed",
        chrono::Weekday::Thu => "Thu",
        chrono::Weekday::Fri => "Fri",
        chrono::Weekday::Sat => "Sat",
        chrono::Weekday::Sun => "Sun",
    }
}

pub fn access_levels_human_name(access_levels: &[AccessLevel], access_level: i32) -> String {
    access_levels
        .iter()
        .filter_map(|level| (level.level == access_level).then_some(level.name.as_str()))
        // Use intersperse: https://github.com/rust-lang/rust/issues/79524
        .collect_vec()
        .join(" | ")
}

pub fn event_visibility_human_name(visibility: &EventVisibility) -> &'static str {
    match visibility {
        EventVisibility::HideAll => "Hide completelly",
        EventVisibility::HideName => "Hide name and description",
        EventVisibility::HideDescription => "Hide description",
        EventVisibility::Show => "Show",
    }
}
