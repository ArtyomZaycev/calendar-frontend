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

impl<'a> PopupBuilder<'a> for PopupType {
    fn build(
        &'a mut self,
        ctx: &'a egui::Context,
    ) -> Box<dyn FnOnce(&mut egui::Ui) -> egui::Response + 'a> {
        match self {
            PopupType::Profile(w) => w.build(ctx),
            PopupType::Login(w) => w.build(ctx),
            PopupType::SignUp(w) => w.build(ctx),
            PopupType::NewEvent(w) => w.build(ctx),
            PopupType::UpdateEvent(w) => w.build(ctx),
        }
    }

    fn title(&'a self) -> Option<String> {
        match self {
            PopupType::Profile(w) => w.title(),
            PopupType::Login(w) => w.title(),
            PopupType::SignUp(w) => w.title(),
            PopupType::NewEvent(w) => w.title(),
            PopupType::UpdateEvent(w) => w.title(),
        }
    }

    fn signals(&'a self) -> Vec<AppSignal> {
        match self {
            PopupType::Profile(w) => w.signals(),
            PopupType::Login(w) => w.signals(),
            PopupType::SignUp(w) => w.signals(),
            PopupType::NewEvent(w) => w.signals(),
            PopupType::UpdateEvent(w) => w.signals(),
        }
    }

    fn is_closed(&'a self) -> bool {
        match self {
            PopupType::Profile(w) => w.is_closed(),
            PopupType::Login(w) => w.is_closed(),
            PopupType::SignUp(w) => w.is_closed(),
            PopupType::NewEvent(w) => w.is_closed(),
            PopupType::UpdateEvent(w) => w.is_closed(),
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
}

impl<'a> WidgetBuilder<'a> for Popup {
    type OutputWidget = Box<dyn FnOnce(&mut egui::Ui) -> egui::Response + 'a>;

    fn build(&'a mut self, ctx: &'a egui::Context) -> Self::OutputWidget
    where
        Self::OutputWidget: egui::Widget + 'a,
    {
        Box::new(|_| {
            let title = self.t.title();
            egui::Window::new(title.clone().unwrap_or_default())
                .id(self.id)
                .title_bar(title.is_some())
                .collapsible(false)
                .resizable(false)
                .default_size(Vec2::new(320., 0.))
                .show(ctx, |ui| {
                    ui.add(self.t.build(ctx))
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
        self.t.is_closed()
    }

    pub fn signals(&self) -> Vec<AppSignal> {
        self.t.signals()
    }

    pub fn get_type(&self) -> &PopupType {
        &self.t
    }
    pub fn get_type_mut(&mut self) -> &mut PopupType {
        &mut self.t
    }
}
