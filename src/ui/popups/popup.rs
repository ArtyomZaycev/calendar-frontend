use derive_is_enum_variant::is_enum_variant;
use egui::Vec2;

use crate::ui::{widget_builder::AppWidgetBuilder, widget_signal::AppSignal};

use super::{event_input::EventInput, login::Login, sign_up::SignUp};

#[derive(is_enum_variant)]
pub enum PopupType {
    Login(Login),
    SignUp(SignUp),
    NewEvent(EventInput),
    UpdateEvent(EventInput),
}

impl PopupType {
    pub fn popup(self) -> Popup {
        Popup::new(self)
    }
}

pub struct Popup {
    id: egui::Id,
    t: PopupType,
}

impl<'a> AppWidgetBuilder<'a> for Popup {
    type OutputWidget = Box<dyn FnOnce(&mut egui::Ui) -> egui::Response + 'a>;
    type Signal = AppSignal;

    fn build(&'a mut self, ctx: &'a egui::Context) -> Self::OutputWidget
    where
        Self::OutputWidget: egui::Widget + 'a,
    {
        Box::new(|_| {
            egui::Window::new("")
                .id(self.id)
                .title_bar(false)
                .resizable(false)
                .default_size(Vec2::new(320., 0.))
                .show(ctx, |ui| {
                    // TODO: enum_dispatch?
                    match &mut self.t {
                        PopupType::Login(w) => ui.add(w.build(ctx)),
                        PopupType::SignUp(w) => ui.add(w.build(ctx)),
                        PopupType::NewEvent(w) => ui.add(w.build(ctx)),
                        PopupType::UpdateEvent(w) => ui.add(w.build(ctx)),
                    }
                })
                .unwrap()
                .inner
                .unwrap()
        })
    }

    fn signals(&'a self) -> Vec<Self::Signal> {
        match &self.t {
            PopupType::Login(w) => w.signals(),
            PopupType::SignUp(w) => w.signals(),
            PopupType::NewEvent(w) => w.signals(),
            PopupType::UpdateEvent(w) => w.signals(),
        }
    }

    fn is_closed(&self) -> bool {
        match &self.t {
            PopupType::Login(w) => w.is_closed(),
            PopupType::SignUp(w) => w.is_closed(),
            PopupType::NewEvent(w) => w.is_closed(),
            PopupType::UpdateEvent(w) => w.is_closed(),
        }
    }
}

impl Popup {
    pub fn new(popup: PopupType) -> Self {
        Self {
            id: egui::Id::new(rand::random::<i64>()),
            t: popup,
        }
    }

    pub fn get_type(&self) -> &PopupType {
        &self.t
    }
    pub fn get_type_mut(&mut self) -> &mut PopupType {
        &mut self.t
    }
}
