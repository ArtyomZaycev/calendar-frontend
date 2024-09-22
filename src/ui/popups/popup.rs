use super::popup_content::{ContentInfo, PopupContent};
use crate::app::CalendarApp;
use derive_is_enum_variant::is_enum_variant;
use egui::{Align, Layout, Vec2};

#[derive(is_enum_variant, Clone, Copy)]
pub enum PopupType {
    Profile,
    Login,
    SignUp,
    NewEvent,
    UpdateEvent,
    NewEventTemplate,
    UpdateEventTemplate,
    NewSchedule,
    UpdateSchedule,
    NewPermission,
    UpdatePermission,
    ChangeAccessLevels,
}

pub struct Popup {
    id: egui::Id,
    popup_type: PopupType,
    is_closed: bool,
    content: Box<dyn PopupContent + Send>,
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
                self.content.init_frame(app, &mut info);
                self.content.show_title(app, ui, &mut info);
                self.content.show_content(app, ui, &mut info);
                ui.with_layout(Layout::right_to_left(Align::TOP), |ui| {
                    self.content.show_buttons(app, ui, &mut info);
                    if let Some(error) = info.get_error() {
                        ui.with_layout(Layout::left_to_right(Align::TOP), |ui| {
                            self.content.show_error(app, ui, &error);
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
    pub fn new<P: PopupContent + Send + 'static>(popup: P) -> Self {
        Self {
            id: egui::Id::new(rand::random::<i64>()),
            popup_type: popup.get_type(),
            is_closed: false,
            content: Box::new(popup),
        }
    }

    pub fn get_type(&self) -> PopupType {
        self.popup_type
    }

    pub fn close(&mut self) {
        self.is_closed = true;
    }

    pub fn is_closed(&self) -> bool {
        self.is_closed
    }
}
