use calendar_lib::api::{sharing::Permissions, utils::TableId};
use chrono::NaiveDate;

use super::{AppView, EventsView};
use crate::{
    app_local_storage::AppLocalStorage,
    state::{main_state::UserState, State},
    ui::popups::popup_manager::PopupManager,
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

    pub fn get_selected_user_state(&self) -> &UserState {
        self.state.get_user_state(self.selected_user_id)
    }

    pub fn get_selected_user_permissions(&self) -> Permissions {
        self.state.get_user_permissions(self.selected_user_id)
    }

    pub fn get_selected_access_level(&self) -> i32 {
        self.get_selected_user_permissions().access_level
    }

    pub fn prepare_date(&mut self, date: NaiveDate) {
        self.state.prepare_date(
            self.selected_user_id,
            self.get_selected_user_permissions().access_level,
            date,
        );
    }
}
