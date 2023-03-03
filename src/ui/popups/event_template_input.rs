use std::ops::RangeInclusive;

use calendar_lib::api::event_templates::types::NewEventTemplate;
use chrono::NaiveTime;
use egui::{Align, Layout, TextEdit};

use crate::ui::{
    time_picker::TimePicker,
    widget_signal::{AppSignal, StateSignal},
};

use super::popup_builder::PopupBuilder;

pub struct EventTemplateInput {
    pub max_access_level: i32,
    pub user_id: i32,

    pub name: String,
    pub event_name: String,
    pub event_description: String,
    pub duration: NaiveTime,
    pub access_level: i32,

    pub closed: bool,
    pub signals: Vec<AppSignal>,
}

impl EventTemplateInput {
    pub fn new(user_id: i32, max_access_level: i32) -> Self {
        Self {
            max_access_level,
            user_id,
            name: String::default(),
            event_name: String::default(),
            event_description: String::default(),
            duration: NaiveTime::from_hms_opt(0, 30, 0).unwrap(),
            access_level: 0,
            closed: false,
            signals: vec![],
        }
    }
}

impl<'a> PopupBuilder<'a> for EventTemplateInput {
    fn build(
        &'a mut self,
        _ctx: &'a egui::Context,
    ) -> Box<dyn FnOnce(&mut egui::Ui) -> egui::Response + 'a> {
        self.signals.clear();
        Box::new(|ui| {
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

                ui.add(egui::Slider::new(
                    &mut self.access_level,
                    RangeInclusive::new(0, self.max_access_level),
                ));

                ui.with_layout(Layout::right_to_left(Align::TOP), |ui| {
                    // RTL

                    if ui.button("Create").clicked() {
                        self.signals.push(AppSignal::StateSignal(
                            StateSignal::InsertEventTemplate(NewEventTemplate {
                                user_id: self.user_id,
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
                    if ui.button("Cancel").clicked() {
                        self.closed = true;
                    }
                });
            })
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
