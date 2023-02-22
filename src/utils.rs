use std::future::Future;

use chrono::Weekday;

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
    email.contains(|c| c == '@')
}
pub fn is_valid_password(password: &str) -> bool {
    password.is_ascii() && password.len() <= 24
}
pub fn is_strong_enough_password(password: &str) -> bool {
    is_valid_password(password) && password.len() >= 8
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
