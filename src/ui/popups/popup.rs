use derive_is_enum_variant::is_enum_variant;

use crate::{db::state::State, ui::widget_builder::WidgetBuilder};

use super::{event_input::EventInput, login::Login, sign_up::SignUp};

#[derive(is_enum_variant)]
pub enum PopupType {
    Login(Login),
    SignUp(SignUp),
    NewEvent(EventInput),
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

impl WidgetBuilder for Popup {
    fn show(&mut self, state: &mut State, ctx: &egui::Context, ui: &mut egui::Ui) -> bool {
        egui::Window::new("")
            .id(self.id)
            .title_bar(false)
            .show(ctx, |ui| {
                // TODO: enum_dispatch?
                match &mut self.t {
                    PopupType::Login(w) => w.show(state, ctx, ui),
                    PopupType::SignUp(w) => w.show(state, ctx, ui),
                    PopupType::NewEvent(w) => w.show(state, ctx, ui),
                }
            })
            .unwrap()
            .inner
            .unwrap_or(true)
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
}
