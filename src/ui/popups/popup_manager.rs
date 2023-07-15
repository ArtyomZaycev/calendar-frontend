use calendar_lib::api::{
    event_templates::types::EventTemplate, events::types::Event, schedules::types::Schedule,
};
use itertools::Itertools;

use crate::{states::State, ui::signal::AppSignal};

use super::{
    event_input::EventInput,
    event_template_input::EventTemplateInput,
    login::Login,
    new_password_input::NewPasswordInput,
    popup::{Popup, PopupType},
    profile::Profile,
    schedule_input::ScheduleInput,
    sign_up::SignUp,
};

pub struct PopupManager {
    popups: Vec<Popup>,
}

impl PopupManager {
    pub fn new() -> Self {
        Self { popups: vec![] }
    }

    pub fn clear(&mut self) {
        self.popups.clear();
    }

    pub fn show(&mut self, state: &State, ctx: &egui::Context) {
        self.popups.iter_mut().for_each(|p| p.show(state, ctx))
    }

    pub fn get_signals(&mut self) -> Vec<AppSignal> {
        self.popups
            .iter_mut()
            .flat_map(|p| p.get_signals())
            .collect()
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

#[allow(dead_code)]
impl PopupManager {
    fn get_popup<'a, F: Fn(&PopupType) -> bool>(&'a mut self, check: F) -> Option<&'a mut Popup> {
        self.popups
            .iter_mut()
            .find_map(|p| check(p.get_type()).then_some(p))
    }

    pub fn get_profile<'a>(&'a mut self) -> Option<&'a mut Popup> {
        self.get_popup(|t| t.is_profile())
    }
    pub fn get_login<'a>(&'a mut self) -> Option<&'a mut Popup> {
        self.get_popup(|t| t.is_login())
    }
    pub fn get_sign_up<'a>(&'a mut self) -> Option<&'a mut Popup> {
        self.get_popup(|t| t.is_sign_up())
    }
    pub fn get_new_event<'a>(&'a mut self) -> Option<&'a mut Popup> {
        self.get_popup(|t| t.is_new_event())
    }
    pub fn get_update_event<'a>(&'a mut self) -> Option<&'a mut Popup> {
        self.get_popup(|t| t.is_update_event())
    }
    pub fn get_new_event_template<'a>(&'a mut self) -> Option<&'a mut Popup> {
        self.get_popup(|t| t.is_new_event_template())
    }
    pub fn get_update_event_template<'a>(&'a mut self) -> Option<&'a mut Popup> {
        self.get_popup(|t| t.is_update_event_template())
    }
    pub fn get_new_schedule<'a>(&'a mut self) -> Option<&'a mut Popup> {
        self.get_popup(|t| t.is_new_schedule())
    }
    pub fn get_update_schedule<'a>(&'a mut self) -> Option<&'a mut Popup> {
        self.get_popup(|t| t.is_update_schedule())
    }
    pub fn get_new_password<'a>(&'a mut self) -> Option<&'a mut Popup> {
        self.get_popup(|t| t.is_new_password())
    }
}

#[allow(dead_code)]
impl PopupManager {
    fn is_open<'a, F: Fn(&PopupType) -> bool>(&'a mut self, check: F) -> bool {
        self.popups.iter_mut().any(|p| check(p.get_type()))
    }

    pub fn is_open_profile<'a>(&'a mut self) -> bool {
        self.is_open(|t| t.is_profile())
    }
    pub fn is_open_login<'a>(&'a mut self) -> bool {
        self.is_open(|t| t.is_login())
    }
    pub fn is_open_sign_up<'a>(&'a mut self) -> bool {
        self.is_open(|t| t.is_sign_up())
    }
    pub fn is_open_new_event<'a>(&'a mut self) -> bool {
        self.is_open(|t| t.is_new_event())
    }
    pub fn is_open_update_event<'a>(&'a mut self) -> bool {
        self.is_open(|t| t.is_update_event())
    }
    pub fn is_open_new_event_template<'a>(&'a mut self) -> bool {
        self.is_open(|t| t.is_new_event_template())
    }
    pub fn is_open_update_event_template<'a>(&'a mut self) -> bool {
        self.is_open(|t| t.is_update_event_template())
    }
    pub fn is_open_new_schedule<'a>(&'a mut self) -> bool {
        self.is_open(|t| t.is_new_schedule())
    }
    pub fn is_open_update_schedule<'a>(&'a mut self) -> bool {
        self.is_open(|t| t.is_update_schedule())
    }
    pub fn is_open_new_password<'a>(&'a mut self) -> bool {
        self.is_open(|t| t.is_new_password())
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
    pub fn open_new_event(&mut self) {
        self.popups
            .push(PopupType::NewEvent(EventInput::new("new_event_popup")).popup());
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
    pub fn open_new_event_template(&mut self) {
        self.popups.push(
            PopupType::NewEventTemplate(EventTemplateInput::new("new_event_template_popup"))
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
    pub fn open_new_schedule(&mut self) {
        self.popups
            .push(PopupType::NewSchedule(ScheduleInput::new("new_schedule_popup")).popup());
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
    pub fn open_new_password(&mut self) {
        self.popups
            .push(PopupType::NewPassword(NewPasswordInput::new()).popup());
    }
}
