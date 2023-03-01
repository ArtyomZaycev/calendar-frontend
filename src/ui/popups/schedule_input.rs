use std::ops::RangeInclusive;

use calendar_lib::api::{
    event_templates::types::EventTemplate,
    schedules::types::{NewEventPlan, NewSchedule},
};
use chrono::{Days, Local, NaiveDate};

use crate::ui::{
    date_picker::DatePicker,
    widget_signal::{AppSignal, StateSignal},
};

use super::popup_builder::PopupBuilder;

pub struct ScheduleInput {
    pub user_id: i32,
    pub max_access_level: i32,
    pub templates: Vec<EventTemplate>,

    pub template_id: Option<i32>,
    pub name: String,
    pub description_enabled: bool,
    pub description: String,
    pub first_day: NaiveDate,
    pub last_day_enabled: bool,
    pub last_day: NaiveDate,
    pub access_level: i32,
    pub events: Vec<NewEventPlan>,

    pub closed: bool,
    pub signals: Vec<AppSignal>,
}

impl ScheduleInput {
    pub fn new(user_id: i32, max_access_level: i32, templates: Vec<EventTemplate>) -> Self {
        let now = Local::now().naive_local();

        Self {
            user_id,
            max_access_level,
            templates,
            template_id: None,
            name: String::default(),
            description_enabled: false,
            description: String::default(),
            first_day: now.date(),
            last_day_enabled: false,
            last_day: now.date() + Days::new(1),
            access_level: 0,
            events: vec![],
            closed: false,
            signals: vec![],
        }
    }
}

impl<'a> PopupBuilder<'a> for ScheduleInput {
    fn build(
        &'a mut self,
        _ctx: &'a egui::Context,
    ) -> Box<dyn FnOnce(&mut egui::Ui) -> egui::Response + 'a> {
        self.signals.clear();
        Box::new(|ui| {
            ui.vertical(|ui| {
                ui.text_edit_singleline(&mut self.name);
                ui.checkbox(&mut self.description_enabled, "Description");

                if self.description_enabled {
                    ui.text_edit_multiline(&mut self.description);
                }

                ui.add(DatePicker::new("schedule_first_day", &mut self.first_day));

                ui.checkbox(&mut self.last_day_enabled, "Description");

                if self.last_day_enabled {
                    ui.add(DatePicker::new("schedule_last_day", &mut self.last_day));
                }

                ui.add(egui::Slider::new(
                    &mut self.access_level,
                    RangeInclusive::new(0, self.max_access_level),
                ));

                ui.horizontal(|ui| {
                    if ui.button("Cancel").clicked() {
                        self.closed = true;
                    }
                    if ui
                        .add_enabled(self.template_id.is_some(), egui::Button::new("Create"))
                        .clicked()
                    {
                        self.signals
                            .push(AppSignal::StateSignal(StateSignal::InsertSchedule(
                                NewSchedule {
                                    user_id: self.user_id, // TODO
                                    template_id: self.template_id.unwrap(),
                                    name: self.name.clone(),
                                    description: self
                                        .description_enabled
                                        .then(|| self.description.clone()),
                                    first_day: self.first_day,
                                    last_day: self.last_day_enabled.then_some(self.last_day),
                                    access_level: self.access_level,
                                    events: self.events.clone(),
                                },
                            )));
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
