use super::{AppView, EventsView};
use crate::{
    app_local_storage::AppLocalStorage,
    config::Config,
    state::{custom_requests::LoginRequest, main_state::RequestIdentifier, State},
    tables::DbTable,
    ui::{popups::popup_manager::PopupManager, signal::AppSignal},
};

pub struct CalendarApp {
    pub(super) local_storage: AppLocalStorage,
    pub(super) state: State,
    pub(super) view: AppView,
    pub(super) popup_manager: PopupManager,
}

impl CalendarApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let mut local_storage = AppLocalStorage::new();
        let state = State::new();
        match local_storage.get_jwt() {
            Some(jwt) => {
                state.login_by_jwt(jwt);
            }
            _ => {
                println!("Auth info not found");
            }
        }

        Self {
            local_storage,
            state,
            view: EventsView::Days(chrono::Local::now().naive_local().date()).into(),
            popup_manager: PopupManager::new(),
        }
    }
}

impl CalendarApp {
    pub(super) fn logout(&mut self) {
        self.local_storage.clear_jwt();
        self.popup_manager.clear();
        self.view = EventsView::Days(chrono::Local::now().naive_local().date()).into();
        self.state.logout();
    }

    pub(super) fn parse_signal(&mut self, signal: AppSignal) {
        match signal {
            AppSignal::ChangeEvent(event_id) => {
                if let Some(event) = self
                    .state
                    .user_state
                    .events
                    .get_table()
                    .get()
                    .iter()
                    .find(|event| event.id == event_id)
                {
                    self.popup_manager.open_update_event(&event.clone());
                }
            }
            AppSignal::ChangeEventTemplate(template_id) => {
                if let Some(template) = self
                    .state
                    .user_state
                    .event_templates
                    .get_table()
                    .get()
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
                    .user_state
                    .schedules
                    .get_table()
                    .get()
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
