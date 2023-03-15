use derive_is_enum_variant::is_enum_variant;
use egui::{InnerResponse, Vec2};

use crate::{
    state::State,
    ui::{widget_builder::WidgetBuilder, widget_signal::AppSignal},
};

use super::{
    event_input::EventInput,
    event_template_input::EventTemplateInput,
    login::Login,
    popup_builder::{ContentUiInfo, PopupBuilder},
    profile::Profile,
    schedule_input::ScheduleInput,
    sign_up::SignUp,
};

#[derive(is_enum_variant)]
pub enum PopupType {
    Profile(Profile),
    Login(Login),
    SignUp(SignUp),
    NewEvent(EventInput),
    UpdateEvent(EventInput),
    NewSchedule(ScheduleInput),
    UpdateSchedule(ScheduleInput),
    NewEventTemplate(EventTemplateInput),
}

impl<'a> PopupBuilder<'a> for PopupType {
    fn content(
        &'a mut self,
        ui: &mut egui::Ui,
        ctx: &'a egui::Context,
        state: &'a State,
    ) -> InnerResponse<ContentUiInfo<'a>> {
        match self {
            PopupType::Profile(w) => w.content(ui, ctx, state),
            PopupType::Login(w) => w.content(ui, ctx, state),
            PopupType::SignUp(w) => w.content(ui, ctx, state),
            PopupType::NewEvent(w) => w.content(ui, ctx, state),
            PopupType::UpdateEvent(w) => w.content(ui, ctx, state),
            PopupType::NewSchedule(w) => w.content(ui, ctx, state),
            PopupType::UpdateSchedule(w) => w.content(ui, ctx, state),
            PopupType::NewEventTemplate(w) => w.content(ui, ctx, state),
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

    signals: Vec<AppSignal>,
    is_closed: bool,
}

impl<'a> WidgetBuilder<'a> for Popup {
    type OutputWidget = Box<dyn FnOnce(&mut egui::Ui) -> egui::Response + 'a>;

    fn build(&'a mut self, ctx: &'a egui::Context, state: &'a State) -> Self::OutputWidget
    where
        Self::OutputWidget: egui::Widget + 'a,
    {
        Box::new(|_| {
            let title: Option<String> = None;
            egui::Window::new(title.clone().unwrap_or_default())
                .id(self.id)
                .title_bar(title.is_some())
                .collapsible(false)
                .resizable(false)
                .default_size(Vec2::new(320., 0.))
                .show(ctx, |ui| {
                    let InnerResponse {
                        mut inner,
                        response,
                    } = self.t.build(ctx, state)(ui);
                    self.signals.append(&mut inner.signals);
                    self.is_closed = self.is_closed || inner.is_closed;
                    response
                })
                .unwrap()
                .inner
                .unwrap()
        })
    }
}

impl Popup {
    pub fn new(popup: PopupType) -> Self {
        println!("new");
        Self {
            id: egui::Id::new(rand::random::<i64>()),
            t: popup,
            signals: vec![],
            is_closed: false,
        }
    }

    pub fn get_type(&self) -> &PopupType {
        &self.t
    }
    pub fn get_type_mut(&mut self) -> &mut PopupType {
        &mut self.t
    }

    pub fn close(&mut self) {
        self.is_closed = true;
    }

    pub fn is_closed(&self) -> bool {
        self.is_closed
    }
    pub fn signals(&mut self) -> Vec<AppSignal> {
        self.signals.drain(..).collect()
    }
}
