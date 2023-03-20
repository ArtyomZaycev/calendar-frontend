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

use super::popup_builder::{ContentUiInfo, PopupBuilder};

pub struct EventTemplateInput {
    pub eid: egui::Id,

    pub name: String,
    pub event_name: String,
    pub event_description: String,
    pub duration: NaiveTime,
    pub access_level: i32,
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
        }
    }
}

impl<'a> PopupBuilder<'a> for EventTemplateInput {
    fn title(&self) -> Option<String> {
        Some("New Event Template".to_owned())
    }

    fn content(
        &'a mut self,
        ui: &mut egui::Ui,
        _ctx: &'a egui::Context,
        state: &'a State,
    ) -> InnerResponse<ContentUiInfo<'a>> {
        if self.access_level == -1 {
            self.access_level = state.get_access_level().level;
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

            ui.horizontal(|ui| {
                ui.label("Access level: ");
                ui.add(AccessLevelPicker::new(
                    self.eid.with("access_level"),
                    &mut self.access_level,
                    &state.access_levels,
                ));
            });

            ContentUiInfo::new()
                .button(|ui, builder, _| {
                    let response = ui.button("Cancel");
                    if response.clicked() {
                        builder.close();
                    }
                    response
                })
                .button(|ui, builder, is_error| {
                    let response = ui.add_enabled(!is_error, egui::Button::new("Create"));
                    if response.clicked() {
                        builder.signal(AppSignal::StateSignal(StateSignal::InsertEventTemplate(
                            NewEventTemplate {
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
                            },
                        )));
                    }
                    response
                })
        })
    }
}
