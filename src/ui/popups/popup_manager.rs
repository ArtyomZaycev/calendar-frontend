use std::sync::{Mutex, MutexGuard};

use calendar_lib::api::{
    auth::types::AccessLevel,
    event_templates::types::EventTemplate,
    events::types::Event,
    permissions::types::GrantedPermission,
    schedules::types::Schedule,
    utils::{TableId, User},
};
use itertools::Itertools;

use crate::{app::CalendarApp, state::state_table::StateTable};

use super::{
    change_access_levels::ChangeAccessLevelsPopup,
    event_input::EventInput,
    event_template_input::EventTemplateInput,
    login::Login,
    permission_input::PermissionInput,
    popup::{Popup, PopupType},
    profile::Profile,
    schedule_input::ScheduleInput,
    sign_up::SignUp,
};

pub struct PopupManager {
    popups: Vec<Popup>,
}

impl PopupManager {
    fn new() -> Self {
        Self { popups: vec![] }
    }

    pub fn get() -> MutexGuard<'static, Self> {
        use std::sync::OnceLock;

        static DATA: OnceLock<Mutex<PopupManager>> = OnceLock::new();
        DATA.get_or_init(|| Mutex::new(PopupManager::new()))
            .lock()
            .unwrap()
    }

    pub fn clear(&mut self) {
        self.popups.clear();
    }

    pub fn show(&mut self, app: &CalendarApp, ctx: &egui::Context) {
        self.popups.iter_mut().for_each(|p| p.show(app, ctx))
    }

    pub fn update(&mut self) {
        self.popups
            .iter_mut()
            .enumerate()
            .filter_map(|(i, popup)| popup.is_closed().then_some(i))
            .collect_vec()
            .iter()
            .rev()
            .for_each(|&i| {
                self.popups.swap_remove(i);
            });
    }
}

impl PopupManager {
    #[allow(dead_code)]
    fn get_popup_mut<'a, F: Fn(&PopupType) -> bool>(
        &'a mut self,
        check: F,
    ) -> Option<&'a mut Popup> {
        self.popups
            .iter_mut()
            .find_map(|p| check(p.get_type()).then_some(p))
    }

    pub fn is_open<'a, F: Fn(&PopupType) -> bool>(&'a mut self, check: F) -> bool {
        self.popups.iter_mut().any(|p| check(p.get_type()))
    }
}

impl PopupManager {
    pub fn open_profile(&mut self) {
        self.popups.push(PopupType::Profile(Profile::new()).popup());
    }
    pub fn open_login(&mut self) {
        self.popups.push(PopupType::Login(Login::new()).popup());
    }
    pub fn open_sign_up(&mut self) {
        self.popups.push(PopupType::SignUp(SignUp::new()).popup());
    }
    pub fn open_new_event(&mut self, user_id: i32) {
        self.popups
            .push(PopupType::NewEvent(EventInput::new("new_event_popup", user_id)).popup());
    }
    pub fn open_update_event(&mut self, event: &Event) {
        self.popups.push(
            PopupType::UpdateEvent(EventInput::change(
                format!("update_event_popup_{}", event.id),
                event,
            ))
            .popup(),
        );
    }
    pub fn open_new_event_template(&mut self, user_id: i32) {
        self.popups.push(
            PopupType::NewEventTemplate(EventTemplateInput::new(
                "new_event_template_popup",
                user_id,
            ))
            .popup(),
        );
    }
    pub fn open_update_event_template(&mut self, template: &EventTemplate) {
        self.popups.push(
            PopupType::UpdateEventTemplate(EventTemplateInput::change(
                format!("update_event_template_popup_{}", template.id),
                template,
            ))
            .popup(),
        );
    }
    pub fn open_new_schedule(&mut self, user_id: i32) {
        self.popups.push(
            PopupType::NewSchedule(ScheduleInput::new("new_schedule_popup", user_id)).popup(),
        );
    }
    pub fn open_update_schedule(&mut self, schedule: &Schedule) {
        self.popups.push(
            PopupType::UpdateSchedule(ScheduleInput::change(
                format!("update_schedule_popup_{}", schedule.id),
                schedule,
            ))
            .popup(),
        );
    }
    pub fn open_new_permission(&mut self, giver_user_id: TableId) {
        self.popups.push(
            PopupType::UpdatePermission(PermissionInput::new(
                format!("new_permission_popup_{}", giver_user_id),
                giver_user_id,
            ))
            .popup(),
        );
    }
    pub fn open_update_permission(&mut self, permission: &GrantedPermission, user: &User) {
        self.popups.push(
            PopupType::NewPermission(PermissionInput::change(
                format!("update_permission_popup_{}", permission.id),
                permission,
                user,
            ))
            .popup(),
        );
    }
    pub fn open_change_access_levels(
        &mut self,
        user_id: TableId,
        access_levels: &StateTable<AccessLevel>,
    ) {
        self.popups.push(
            PopupType::ChangeAccessLevels(ChangeAccessLevelsPopup::new(
                format!("change_access_levels_popup_{}", user_id),
                user_id,
                access_levels,
            ))
            .popup(),
        );
    }
}
