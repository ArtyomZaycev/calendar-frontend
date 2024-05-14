use super::{
    event_input::EventInput,
    event_template_input::EventTemplateInput,
    login::Login,
    permission_input::PermissionInput,
    popup_content::{ContentInfo, PopupContent},
    profile::Profile,
    schedule_input::ScheduleInput,
    sign_up::SignUp,
};
use crate::app::CalendarApp;
use derive_is_enum_variant::is_enum_variant;
use egui::{Align, Layout, Vec2};

#[derive(is_enum_variant)]
pub enum PopupType {
    Profile(Profile),
    Login(Login),
    SignUp(SignUp),
    NewEvent(EventInput),
    UpdateEvent(EventInput),
    NewEventTemplate(EventTemplateInput),
    UpdateEventTemplate(EventTemplateInput),
    NewSchedule(ScheduleInput),
    UpdateSchedule(ScheduleInput),
    NewPermission(PermissionInput),
    UpdatePermission(PermissionInput),
}

impl PopupContent for PopupType {
    fn init_frame(&mut self, state: &CalendarApp, info: &mut ContentInfo) {
        match self {
            PopupType::Profile(w) => w.init_frame(state, info),
            PopupType::Login(w) => w.init_frame(state, info),
            PopupType::SignUp(w) => w.init_frame(state, info),
            PopupType::NewEvent(w) => w.init_frame(state, info),
            PopupType::UpdateEvent(w) => w.init_frame(state, info),
            PopupType::NewEventTemplate(w) => w.init_frame(state, info),
            PopupType::UpdateEventTemplate(w) => w.init_frame(state, info),
            PopupType::NewSchedule(w) => w.init_frame(state, info),
            PopupType::UpdateSchedule(w) => w.init_frame(state, info),
            PopupType::NewPermission(w) => w.init_frame(state, info),
            PopupType::UpdatePermission(w) => w.init_frame(state, info),
        }
    }

    fn get_title(&mut self) -> Option<String> {
        match self {
            PopupType::Profile(w) => w.get_title(),
            PopupType::Login(w) => w.get_title(),
            PopupType::SignUp(w) => w.get_title(),
            PopupType::NewEvent(w) => w.get_title(),
            PopupType::UpdateEvent(w) => w.get_title(),
            PopupType::NewEventTemplate(w) => w.get_title(),
            PopupType::UpdateEventTemplate(w) => w.get_title(),
            PopupType::NewSchedule(w) => w.get_title(),
            PopupType::UpdateSchedule(w) => w.get_title(),
            PopupType::NewPermission(w) => w.get_title(),
            PopupType::UpdatePermission(w) => w.get_title(),
        }
    }

    fn show_title(&mut self, state: &CalendarApp, ui: &mut egui::Ui, info: &mut ContentInfo) {
        match self {
            PopupType::Profile(w) => w.show_title(state, ui, info),
            PopupType::Login(w) => w.show_title(state, ui, info),
            PopupType::SignUp(w) => w.show_title(state, ui, info),
            PopupType::NewEvent(w) => w.show_title(state, ui, info),
            PopupType::UpdateEvent(w) => w.show_title(state, ui, info),
            PopupType::NewEventTemplate(w) => w.show_title(state, ui, info),
            PopupType::UpdateEventTemplate(w) => w.show_title(state, ui, info),
            PopupType::NewSchedule(w) => w.show_title(state, ui, info),
            PopupType::UpdateSchedule(w) => w.show_title(state, ui, info),
            PopupType::NewPermission(w) => w.show_title(state, ui, info),
            PopupType::UpdatePermission(w) => w.show_title(state, ui, info),
        }
    }

    fn show_content(&mut self, state: &CalendarApp, ui: &mut egui::Ui, info: &mut ContentInfo) {
        match self {
            PopupType::Profile(w) => w.show_content(state, ui, info),
            PopupType::Login(w) => w.show_content(state, ui, info),
            PopupType::SignUp(w) => w.show_content(state, ui, info),
            PopupType::NewEvent(w) => w.show_content(state, ui, info),
            PopupType::UpdateEvent(w) => w.show_content(state, ui, info),
            PopupType::NewEventTemplate(w) => w.show_content(state, ui, info),
            PopupType::UpdateEventTemplate(w) => w.show_content(state, ui, info),
            PopupType::NewSchedule(w) => w.show_content(state, ui, info),
            PopupType::UpdateSchedule(w) => w.show_content(state, ui, info),
            PopupType::NewPermission(w) => w.show_content(state, ui, info),
            PopupType::UpdatePermission(w) => w.show_content(state, ui, info),
        }
    }

    fn show_buttons(&mut self, state: &CalendarApp, ui: &mut egui::Ui, info: &mut ContentInfo) {
        match self {
            PopupType::Profile(w) => w.show_buttons(state, ui, info),
            PopupType::Login(w) => w.show_buttons(state, ui, info),
            PopupType::SignUp(w) => w.show_buttons(state, ui, info),
            PopupType::NewEvent(w) => w.show_buttons(state, ui, info),
            PopupType::UpdateEvent(w) => w.show_buttons(state, ui, info),
            PopupType::NewEventTemplate(w) => w.show_buttons(state, ui, info),
            PopupType::UpdateEventTemplate(w) => w.show_buttons(state, ui, info),
            PopupType::NewSchedule(w) => w.show_buttons(state, ui, info),
            PopupType::UpdateSchedule(w) => w.show_buttons(state, ui, info),
            PopupType::NewPermission(w) => w.show_buttons(state, ui, info),
            PopupType::UpdatePermission(w) => w.show_buttons(state, ui, info),
        }
    }
}

impl PopupType {
    pub fn popup(self) -> Popup {
        Popup::new(self)
    }
}

pub struct Popup {
    id: egui::Id,
    t: PopupType,
    is_closed: bool,
}

impl Popup {
    pub fn show(&mut self, app: &CalendarApp, ctx: &egui::Context) {
        let mut info = ContentInfo::new();
        egui::Window::new("")
            .id(self.id)
            .title_bar(false)
            .collapsible(false)
            .resizable(false)
            .default_size(Vec2::new(320., 0.))
            .show(ctx, |ui| {
                self.t.init_frame(app, &mut info);
                self.t.show_title(app, ui, &mut info);
                self.t.show_content(app, ui, &mut info);
                ui.with_layout(Layout::right_to_left(Align::TOP), |ui| {
                    self.t.show_buttons(app, ui, &mut info);
                    if let Some(error) = info.get_error() {
                        ui.with_layout(Layout::left_to_right(Align::TOP), |ui| {
                            self.t.show_error(app, ui, &error);
                        });
                    }
                });
            });
        let is_closed = info.take();
        self.is_closed = is_closed;
    }
}

#[allow(dead_code)]
impl Popup {
    pub fn new(popup: PopupType) -> Self {
        Self {
            id: egui::Id::new(rand::random::<i64>()),
            t: popup,
            is_closed: false,
        }
    }

    pub fn get_type(&self) -> &PopupType {
        &self.t
    }
    pub fn get_type_mut(&mut self) -> &mut PopupType {
        &mut self.t
    }

    pub fn close(&mut self) {
        self.is_closed = true;
    }

    pub fn is_closed(&self) -> bool {
        self.is_closed
    }
}
