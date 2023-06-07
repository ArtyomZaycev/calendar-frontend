use egui::{Align, Color32, Layout, RichText};

use crate::{state::State, ui::signal::AppSignal};

pub struct ContentInfo {
    signals: Vec<AppSignal>,
    error: Option<String>,
    is_closed: bool,
}

#[allow(dead_code)]
impl ContentInfo {
    pub fn new() -> Self {
        Self {
            signals: vec![],
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
    pub fn some_error(&mut self, error: Option<String>) {
        if self.error.is_none() {
            self.error = error;
        }
    }

    pub fn signal(&mut self, signal: impl Into<AppSignal>) {
        self.signals.push(signal.into());
    }

    pub fn close(&mut self) {
        self.is_closed = true;
    }

    pub fn take(self) -> (Vec<AppSignal>, bool) {
        (self.signals, self.is_closed)
    }
}

pub trait PopupContent {
    fn get_title(&mut self) -> Option<String> {
        None
    }

    /// Called first each frame
    fn show_title(&mut self, _state: &State, ui: &mut egui::Ui, _info: &mut ContentInfo) {
        if let Some(title) = self.get_title() {
            ui.heading(title);
            ui.separator();
        }
    }
    /// Called second each frame
    fn show_content(&mut self, state: &State, ui: &mut egui::Ui, info: &mut ContentInfo);
    /// RTL. Called second each frame
    fn show_buttons(&mut self, _state: &State, _ui: &mut egui::Ui, _info: &mut ContentInfo) {}
    /// Called last
    fn show_error(&mut self, _state: &State, ui: &mut egui::Ui, error: &str) {
        ui.with_layout(Layout::left_to_right(Align::TOP), |ui| {
            ui.add(egui::Label::new(RichText::new(error).color(Color32::RED)).wrap(true));
        });
    }
}
