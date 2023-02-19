use derive_is_enum_variant::is_enum_variant;
use egui::Vec2;

use crate::{db::state::State, ui::widget_builder::AppWidgetBuilder};

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
    type Output = Box<dyn FnOnce(&mut egui::Ui) -> egui::Response + 'a>;

    fn build(&'a mut self, state: &'a mut State, ctx: &'a egui::Context) -> Self::Output
    where
        Self::Output: egui::Widget + 'a,
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
                        PopupType::Login(w) => ui.add(w.build(state, ctx)),
                        PopupType::SignUp(w) => ui.add(w.build(state, ctx)),
                        PopupType::NewEvent(w) => ui.add(w.build(state, ctx)),
                        PopupType::UpdateEvent(w) => ui.add(w.build(state, ctx)),
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
            PopupType::Login(w) => w.closed,
            PopupType::SignUp(w) => w.closed,
            PopupType::NewEvent(w) => w.closed,
            PopupType::UpdateEvent(w) => w.closed,
        }
    }

    pub fn get_type(&self) -> &PopupType {
        &self.t
    }
    pub fn get_type_mut(&mut self) -> &mut PopupType {
        &mut self.t
    }
}
