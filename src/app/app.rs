use calendar_lib::api::utils::TableId;
use chrono::NaiveDate;

use super::{AppView, EventsView};
use crate::{
    app_local_storage::AppLocalStorage,
    state::{main_state::UserState, State},
    tables::DbTable,
    ui::{popups::popup_manager::PopupManager, signal::AppSignal},
};

pub struct CalendarApp {
    pub(super) local_storage: AppLocalStorage,
    pub state: State,
    pub(super) view: AppView,

    pub burger_menu_expanded: bool,
    pub selected_user_id: TableId,
    pub selected_date: NaiveDate,
}

impl CalendarApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
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

        Self::configure_styles(&cc.egui_ctx);

        Self {
            local_storage,
            state,
            view: EventsView::Days.into(),

            burger_menu_expanded: true,
            selected_user_id: -1,
            selected_date: chrono::Local::now().naive_local().date(),
        }
    }
}

impl CalendarApp {
    pub(super) fn logout(&mut self) {
        self.local_storage.clear_jwt();
        PopupManager::get().clear();
        self.view = EventsView::Days.into();
        self.state.logout();

        self.burger_menu_expanded = true;
        self.selected_user_id = -1;
        self.selected_date = chrono::Local::now().naive_local().date();
    }

    pub(super) fn parse_signal(&mut self, signal: AppSignal) {
        match signal {
            AppSignal::ChangeEvent(event_id) => {
                if let Some(event) = self
                    .state
                    .get_user_state(self.selected_user_id)
                    .events
                    .get_table()
                    .get()
                    .iter()
                    .find(|event| event.id == event_id)
                {
                    PopupManager::get().open_update_event(&event.clone());
                }
            }
            AppSignal::ChangeEventTemplate(template_id) => {
                if let Some(template) = self
                    .state
                    .get_user_state(self.selected_user_id)
                    .event_templates
                    .get_table()
                    .get()
                    .iter()
                    .find(|template| template.id == template_id)
                {
                    PopupManager::get().open_update_event_template(&template.clone());
                }
            }
            AppSignal::ChangeSchedule(schedule_id) => {
                if let Some(schedule) = self
                    .state
                    .get_user_state(self.selected_user_id)
                    .schedules
                    .get_table()
                    .get()
                    .iter()
                    .find(|schedule| schedule.id == schedule_id)
                {
                    PopupManager::get().open_update_schedule(&schedule.clone());
                }
            }
            AppSignal::AddPassword => {
                PopupManager::get().open_new_password();
            }
        }
    }

    pub(super) fn parse_signals(&mut self, signals: Vec<AppSignal>) {
        signals
            .into_iter()
            .for_each(|signal| self.parse_signal(signal));
    }

    pub fn get_selected_user_state(&self) -> &UserState {
        self.state.get_user_state(self.selected_user_id)
    }
}
