use calendar_lib::api::event_templates::types::NewEventTemplate;
use chrono::NaiveTime;
use egui::{InnerResponse, TextEdit};
use std::hash::Hash;

use crate::{
    state::State,
    ui::{
        access_level_picker::AccessLevelPicker,
        time_picker::TimePicker,
        widget_signal::{AppSignal, StateSignal},
    },
};

use super::popup_builder::{ContentInfo, PopupBuilder};

pub struct EventTemplateInput {
    pub eid: egui::Id,

    pub name: String,
    pub event_name: String,
    pub event_description: String,
    pub duration: NaiveTime,
    pub access_level: i32,

    pub closed: bool,
    pub signals: Vec<AppSignal>,
}

impl EventTemplateInput {
    pub fn new(eid: impl Hash) -> Self {
        Self {
            eid: egui::Id::new(eid),
            name: String::default(),
            event_name: String::default(),
            event_description: String::default(),
            duration: NaiveTime::from_hms_opt(0, 30, 0).unwrap(),
            access_level: -1,
            closed: false,
            signals: vec![],
        }
    }
}

impl<'a> PopupBuilder<'a> for EventTemplateInput {
    fn content(
        &'a mut self,
        ui: &mut egui::Ui,
        _ctx: &'a egui::Context,
        state: &'a State,
    ) -> InnerResponse<ContentInfo<'a>> {
        self.signals.clear();

        if self.access_level == -1 {
            self.access_level = state.me.as_ref().unwrap().current_access_level;
        }

        ui.vertical(|ui| {
            ui.add(TextEdit::singleline(&mut self.name).hint_text("Template name"));
            ui.separator();

            ui.add(TextEdit::singleline(&mut self.event_name).hint_text("Name"));
            ui.add(TextEdit::multiline(&mut self.event_description).hint_text("Description"));

            ui.horizontal(|ui| {
                ui.label("Duration: ");
                ui.add(TimePicker::new(
                    "event_template_duration_picker",
                    &mut self.duration,
                ));
            });

            ui.add(
                AccessLevelPicker::new(
                    self.eid.with("access_level"),
                    &mut self.access_level,
                    &state.me.as_ref().unwrap().access_levels,
                )
                .with_label("Access level: "),
            );

            ContentInfo::new()
                .button(|ui, _| {
                    let response = ui.button("Cancel");
                    if response.clicked() {
                        self.closed = true;
                    }
                    response
                })
                .button(|ui, is_error| {
                    let response = ui.add_enabled(!is_error, egui::Button::new("Create"));
                    if response.clicked() {
                        self.signals.push(AppSignal::StateSignal(
                            StateSignal::InsertEventTemplate(NewEventTemplate {
                                user_id: -1,
                                name: self.name.clone(),
                                event_name: self.event_name.clone(),
                                event_description: (!self.event_description.is_empty())
                                    .then_some(self.event_description.clone()),
                                duration: self
                                    .duration
                                    .signed_duration_since(NaiveTime::default())
                                    .to_std()
                                    .unwrap(),
                                access_level: self.access_level,
                            }),
                        ));
                    }
                    response
                })
        })
    }

    fn signals(&'a self) -> Vec<AppSignal> {
        self.signals.clone()
    }

    fn is_closed(&'a self) -> bool {
        self.closed
    }
}
