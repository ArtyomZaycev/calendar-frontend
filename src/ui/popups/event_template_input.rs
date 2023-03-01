use std::ops::RangeInclusive;

use calendar_lib::api::event_templates::types::NewEventTemplate;
use chrono::Duration;

use crate::ui::widget_signal::{AppSignal, StateSignal};

use super::popup_builder::PopupBuilder;

pub struct EventTemplateInput {
    pub max_access_level: i32,
    pub user_id: i32,

    pub name: String,
    pub event_name: String,
    pub event_description_enabled: bool,
    pub event_description: String,
    pub duration: Duration,
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
            event_description_enabled: false,
            event_description: String::default(),
            duration: Duration::minutes(30),
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
                ui.text_edit_singleline(&mut self.name);
                ui.text_edit_singleline(&mut self.event_name);

                ui.checkbox(&mut self.event_description_enabled, "Event Description");
                if self.event_description_enabled {
                    ui.text_edit_multiline(&mut self.event_description);
                }

                let mut dur = self.duration.num_minutes();
                ui.add(egui::Slider::new(&mut dur, RangeInclusive::new(0, 180)));
                self.duration = Duration::minutes(dur);

                ui.add(egui::Slider::new(
                    &mut self.access_level,
                    RangeInclusive::new(0, self.max_access_level),
                ));

                ui.horizontal(|ui| {
                    if ui.button("Cancel").clicked() {
                        self.closed = true;
                    }
                    if ui.button("Create").clicked() {
                        self.signals.push(AppSignal::StateSignal(
                            StateSignal::InsertEventTemplate(NewEventTemplate {
                                user_id: self.user_id,
                                name: self.name.clone(),
                                event_name: self.event_name.clone(),
                                event_description: self
                                    .event_description_enabled
                                    .then_some(self.event_description.clone()),
                                duration: self.duration.to_std().unwrap(),
                                access_level: self.access_level,
                            }),
                        ));
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
