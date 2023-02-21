use calendar_lib::api::utils::User;
use egui::{Align, Layout};

use crate::ui::widget_signal::{AppSignal, StateSignal};

use super::popup_builder::PopupBuilder;

pub struct Profile {
    pub user: User,

    pub closed: bool,
    pub signals: Vec<AppSignal>,
}

impl Profile {
    pub fn new(user: User) -> Self {
        Self {
            user,
            closed: false,
            signals: vec![],
        }
    }
}

impl<'a> PopupBuilder<'a> for Profile {
    fn build(
        &'a mut self,
        _ctx: &'a egui::Context,
    ) -> Box<dyn FnOnce(&mut egui::Ui) -> egui::Response + 'a> {
        self.signals.clear();
        Box::new(|ui: &mut egui::Ui| {
            ui.with_layout(Layout::top_down(Align::LEFT), |ui| {})
                .response
        })
    }

    fn signals(&'a self) -> Vec<AppSignal> {
        self.signals.clone()
    }

    fn is_closed(&'a self) -> bool {
        self.closed
    }
}
