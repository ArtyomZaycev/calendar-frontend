use super::{CalendarView, EventsView};
use crate::{
    app_local_storage::AppLocalStorage,
    config::Config,
    db::request::RequestDescription,
    state::State,
    ui::{popups::popup_manager::PopupManager, signal::AppSignal},
};

pub struct CalendarApp {
    pub(super) local_storage: AppLocalStorage,
    pub(super) state: State,
    pub(super) view: CalendarView,
    pub(super) popup_manager: PopupManager,
}

impl CalendarApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        if let Some(_storage) = cc.storage {
            //return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        let config = Config::load();
        let mut local_storage = AppLocalStorage::new();
        let mut state = State::new(&config);
        match (local_storage.get_user_id(), local_storage.get_key()) {
            (Some(user_id), Some(key)) => {
                state.login_by_key(user_id, key, RequestDescription::new());
            }
            _ => {
                println!("Auth info not found");
            }
        }

        Self {
            local_storage,
            state,
            view: CalendarView::Events(EventsView::Days(chrono::Local::now().naive_local().date())),
            popup_manager: PopupManager::new(),
        }
    }
}

impl CalendarApp {
    pub(super) fn logout(&mut self) {
        self.popup_manager.clear();
        self.state.logout(RequestDescription::default());
    }

    pub(super) fn parse_signal(&mut self, signal: AppSignal) {
        match signal {
            AppSignal::StateSignal(signal) => self.state.parse_signal(signal),
            AppSignal::ChangeEvent(event_id) => {
                if let Some(event) = self
                    .state
                    .get_events()
                    .iter()
                    .find(|event| event.id == event_id)
                {
                    self.popup_manager.open_update_event(&event.clone());
                }
            }
            AppSignal::ChangeEventTemplate(template_id) => {
                if let Some(template) = self
                    .state
                    .get_event_templates()
                    .iter()
                    .find(|template| template.id == template_id)
                {
                    self.popup_manager
                        .open_update_event_template(&template.clone());
                }
            }
            AppSignal::ChangeSchedule(schedule_id) => {
                if let Some(schedule) = self
                    .state
                    .get_schedules()
                    .iter()
                    .find(|schedule| schedule.id == schedule_id)
                {
                    self.popup_manager.open_update_schedule(&schedule.clone());
                }
            }
            AppSignal::AddPassword => {
                self.popup_manager.open_new_password();
            }
        }
    }

    pub(super) fn parse_signals(&mut self, signals: Vec<AppSignal>) {
        signals
            .into_iter()
            .for_each(|signal| self.parse_signal(signal));
    }
}
