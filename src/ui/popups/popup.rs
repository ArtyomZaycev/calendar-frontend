use derive_is_enum_variant::is_enum_variant;
use egui::Vec2;

use crate::ui::{widget_builder::WidgetBuilder, widget_signal::AppSignal};

use super::{
    event_input::EventInput, login::Login, popup_builder::PopupBuilder, profile::Profile,
    sign_up::SignUp,
};

#[derive(is_enum_variant)]
pub enum PopupType {
    Profile(Profile),
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

impl<'a> WidgetBuilder<'a> for Popup {
    type OutputWidget = Box<dyn FnOnce(&mut egui::Ui) -> egui::Response + 'a>;

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
                        PopupType::Profile(w) => ui.add(w.build(ctx)),
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
}

impl Popup {
    pub fn new(popup: PopupType) -> Self {
        Self {
            id: egui::Id::new(rand::random::<i64>()),
            t: popup,
        }
    }

    pub fn is_closed(&self) -> bool {
        match &self.t {
            PopupType::Profile(w) => w.is_closed(),
            PopupType::Login(w) => w.is_closed(),
            PopupType::SignUp(w) => w.is_closed(),
            PopupType::NewEvent(w) => w.is_closed(),
            PopupType::UpdateEvent(w) => w.is_closed(),
        }
    }

    pub fn signals(&self) -> Vec<AppSignal> {
        match &self.t {
            PopupType::Profile(w) => w.signals(),
            PopupType::Login(w) => w.signals(),
            PopupType::SignUp(w) => w.signals(),
            PopupType::NewEvent(w) => w.signals(),
            PopupType::UpdateEvent(w) => w.signals(),
        }
    }

    pub fn get_type(&self) -> &PopupType {
        &self.t
    }
    pub fn get_type_mut(&mut self) -> &mut PopupType {
        &mut self.t
    }
}
