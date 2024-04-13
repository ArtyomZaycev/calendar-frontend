#[derive(Debug, Clone)]
pub enum AppSignal {
    ChangeEvent(i32),
    ChangeEventTemplate(i32),
    ChangeSchedule(i32),

    AddPassword,
}
