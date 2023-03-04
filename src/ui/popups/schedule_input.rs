use std::ops::RangeInclusive;

use calendar_lib::api::schedules::types::{NewEventPlan, NewSchedule};
use chrono::{Days, Local, NaiveDate, NaiveTime, Weekday};
use egui::{Align, Layout, TextEdit};
use num_traits::FromPrimitive;

use crate::{
    state::State,
    ui::{
        date_picker::DatePicker,
        time_picker::TimePicker,
        widget_signal::{AppSignal, StateSignal},
    },
};

use super::popup_builder::PopupBuilder;

pub struct ScheduleInput {
    pub max_access_level: i32,

    pub template_id: Option<i32>,
    pub name: String,
    pub description: String,
    pub first_day: NaiveDate,
    pub last_day_enabled: bool,
    pub last_day: NaiveDate,
    pub access_level: i32,

    pub new_event_start: NaiveTime,
    pub events: [Vec<NewEventPlan>; 7],

    pub closed: bool,
    pub signals: Vec<AppSignal>,
}

impl ScheduleInput {
    pub fn new(max_access_level: i32) -> Self {
        let now = Local::now().naive_local();

        Self {
            max_access_level,
            template_id: None,
            name: String::default(),
            description: String::default(),
            first_day: now.date(),
            last_day_enabled: false,
            last_day: now.date() + Days::new(1),
            access_level: 0,

            new_event_start: now.time(),
            events: Default::default(),

            closed: false,
            signals: vec![],
        }
    }
}

impl<'a> PopupBuilder<'a> for ScheduleInput {
    fn build(
        &'a mut self,
        _ctx: &'a egui::Context,
        state: &'a State,
    ) -> Box<dyn FnOnce(&mut egui::Ui) -> egui::Response + 'a> {
        self.signals.clear();
        Box::new(|ui| {
            ui.vertical(|ui| {
                ui.add(TextEdit::singleline(&mut self.name).hint_text("Name"));
                ui.add(TextEdit::multiline(&mut self.description).hint_text("Description"));

                egui::ComboBox::from_id_source("schedule_template_list")
                    .selected_text(
                        match self.template_id.and_then(|template_id| {
                            state.event_templates.iter().find(|t| t.id == template_id)
                        }) {
                            Some(template) => &template.name,
                            None => "Template",
                        },
                    )
                    .show_ui(ui, |ui| {
                        state.event_templates.iter().for_each(|template| {
                            ui.selectable_value(
                                &mut self.template_id,
                                Some(template.id),
                                &template.name,
                            );
                        });
                    });

                ui.add(DatePicker::new("schedule_first_day", &mut self.first_day));

                ui.horizontal(|ui| {
                    ui.checkbox(&mut self.last_day_enabled, "Last Day");
                    if self.last_day_enabled {
                        ui.add(DatePicker::new("schedule_last_day", &mut self.last_day));
                    }
                });

                ui.add(egui::Slider::new(
                    &mut self.access_level,
                    RangeInclusive::new(0, self.max_access_level),
                ));

                ui.separator();

                ui.add(TimePicker::new(
                    "schedule_event_start",
                    &mut self.new_event_start,
                ));

                (0..7).for_each(|weekday| {
                    let to_delete = ui
                        .horizontal(|ui| {
                            if ui.button("Add").clicked() {
                                self.events[weekday].push(NewEventPlan {
                                    weekday: Weekday::from_usize(weekday).unwrap(),
                                    time: self.new_event_start,
                                });
                            }
                            ui.add_space(4.);
                            self.events[weekday]
                                .iter()
                                .enumerate()
                                .filter_map(|(i, new_event_plan)| {
                                    ui.spacing_mut().item_spacing = egui::Vec2::default();
                                    ui.label(new_event_plan.time.format("%H:%M").to_string());
                                    ui.small_button("X").clicked().then_some(i)
                                })
                                .collect::<Vec<_>>()
                        })
                        .inner;

                    to_delete.into_iter().rev().for_each(|i| {
                        self.events[weekday].remove(i);
                    });
                });

                ui.with_layout(Layout::right_to_left(Align::TOP), |ui| {
                    // RTL

                    if ui
                        .add_enabled(self.template_id.is_some(), egui::Button::new("Create"))
                        .clicked()
                    {
                        self.signals
                            .push(AppSignal::StateSignal(StateSignal::InsertSchedule(
                                NewSchedule {
                                    user_id: -1,
                                    template_id: self.template_id.unwrap(),
                                    name: self.name.clone(),
                                    description: (!self.description.is_empty())
                                        .then_some(self.description.clone()),
                                    first_day: self.first_day,
                                    last_day: self.last_day_enabled.then_some(self.last_day),
                                    access_level: self.access_level,
                                    events: self
                                        .events
                                        .clone()
                                        .into_iter()
                                        .flat_map(|v| v)
                                        .collect(),
                                },
                            )));
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
