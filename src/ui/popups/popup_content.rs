use egui::{Align, Color32, Layout, RichText};

use crate::app::CalendarApp;

use super::popup::PopupType;

pub struct ContentInfo {
    error: Option<String>,
    is_closed: bool,
}

impl ContentInfo {
    pub fn new() -> Self {
        Self {
            error: None,
            is_closed: false,
        }
    }

    pub fn is_error(&self) -> bool {
        self.error.is_some()
    }
    pub fn get_error(&self) -> Option<String> {
        self.error.clone()
    }

    pub fn error(&mut self, condition: bool, error: &str) {
        if self.error.is_none() && condition {
            self.error = Some(error.to_owned());
        }
    }

    pub fn close(&mut self) {
        self.is_closed = true;
    }

    pub fn take(self) -> bool {
        self.is_closed
    }
}

#[allow(unused_variables)]
pub trait PopupContent {
    fn get_type(&self) -> PopupType;

    /// Called first each frame
    fn init_frame(&mut self, app: &CalendarApp, info: &mut ContentInfo) {}

    fn get_title(&mut self) -> Option<String> {
        None
    }

    fn show_title(&mut self, app: &CalendarApp, ui: &mut egui::Ui, info: &mut ContentInfo) {
        if let Some(title) = self.get_title() {
            ui.heading(title);
            ui.separator();
        }
    }

    fn show_content(&mut self, app: &CalendarApp, ui: &mut egui::Ui, info: &mut ContentInfo);

    /// RTL
    fn show_buttons(&mut self, app: &CalendarApp, ui: &mut egui::Ui, info: &mut ContentInfo) {}

    fn show_error(&mut self, app: &CalendarApp, ui: &mut egui::Ui, error: &str) {
        ui.with_layout(Layout::left_to_right(Align::TOP), |ui| {
            ui.add(egui::Label::new(RichText::new(error).color(Color32::RED)).wrap(true));
        });
    }
}
