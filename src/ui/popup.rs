use egui::{Widget, Context};

use crate::db::state::State;

use super::event_input::EventInput;

pub enum PopupType {
    Login,
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

impl Popup {
    pub fn new(popup: PopupType) -> Self {
        Self {
            id: egui::Id::new(rand::random::<i64>()),
            t: popup,
        }
    }

    // Returns false if this popup should be dropped
    pub fn show(&mut self, ctx: &Context, state: &mut State) -> bool {
        let w = match &mut self.t {
            PopupType::Login => todo!(),
            PopupType::NewEvent(w) => w.make_widget(state),
        };
        egui::Window::new("").id(self.id).title_bar(true).show(ctx, |ui| ui.add(w));
        false
    }
}