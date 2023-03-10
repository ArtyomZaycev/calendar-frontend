use calendar_lib::api::{schedules::types::*, utils::*};
use chrono::{Days, Local, NaiveDate, NaiveTime, Weekday};
use egui::{Button, InnerResponse, TextEdit};
use num_traits::FromPrimitive;
use std::hash::Hash;

use crate::{
    state::State,
    ui::{
        access_level_picker::AccessLevelPicker,
        date_picker::DatePicker,
        time_picker::TimePicker,
        widget_signal::{AppSignal, StateSignal},
    },
};

use super::popup_builder::{PopupBuilder, ContentUiInfo};

pub struct ScheduleInput {
    pub eid: egui::Id,

    pub id: Option<i32>,
    pub template_id: Option<i32>,
    pub name: String,
    pub description: String,
    pub first_day: NaiveDate,
    pub last_day_enabled: bool,
    pub last_day: NaiveDate,
    pub access_level: i32,

    pub init_events: Option<Vec<EventPlan>>,
    pub new_event_start: NaiveTime,
    pub events: [Vec<NewEventPlan>; 7],
}

impl ScheduleInput {
    pub fn new(eid: impl Hash) -> Self {
        let now = Local::now().naive_local();
        let minutes = now
            .time()
            .signed_duration_since(Default::default())
            .num_minutes() as u32;
        let now_time = NaiveTime::from_hms_opt(minutes / 60, minutes % 60, 0).unwrap();

        Self {
            eid: egui::Id::new(eid),
            id: None,
            template_id: None,
            name: String::default(),
            description: String::default(),
            first_day: now.date(),
            last_day_enabled: false,
            last_day: now.date() + Days::new(1),
            access_level: -1,

            init_events: None,
            new_event_start: now_time,
            events: Default::default(),
        }
    }

    pub fn change(eid: impl Hash, schedule: &Schedule) -> Self {
        let now = Local::now().naive_local();
        let minutes = now
            .time()
            .signed_duration_since(Default::default())
            .num_minutes() as u32;
        let now_time = NaiveTime::from_hms_opt(minutes / 60, minutes % 60, 0).unwrap();

        Self {
            eid: egui::Id::new(eid),
            id: Some(schedule.id),
            template_id: Some(schedule.template_id),
            name: schedule.name.clone(),
            description: schedule.description.clone().unwrap_or_default(),
            first_day: schedule.first_day,
            last_day_enabled: schedule.last_day.is_some(),
            last_day: schedule.last_day.unwrap_or(now.date() + Days::new(1)),
            access_level: schedule.access_level,

            init_events: Some(schedule.event_plans.clone()),
            new_event_start: now_time,
            events: schedule
                .event_plans
                .iter()
                .fold(Default::default(), |mut acc, event| {
                    acc[event.weekday.num_days_from_monday() as usize].push(NewEventPlan {
                        weekday: event.weekday,
                        time: event.time,
                    });
                    acc
                }),
        }
    }
}

impl<'a> PopupBuilder<'a> for ScheduleInput {
    fn content(
        &'a mut self,
        ui: &mut egui::Ui,
        _ctx: &'a egui::Context,
        state: &'a State,
    ) -> InnerResponse<ContentUiInfo<'a>> {
        if self.access_level == -1 {
            self.access_level = state.me.as_ref().unwrap().current_access_level;
        }

        ui.vertical(|ui| {
            ui.add(TextEdit::singleline(&mut self.name).hint_text("Name"));
            ui.add(TextEdit::multiline(&mut self.description).hint_text("Description"));

            if self.id.is_none() {
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
            }

            ui.add(DatePicker::new("schedule_first_day", &mut self.first_day));

            ui.horizontal(|ui| {
                ui.checkbox(&mut self.last_day_enabled, "Last Day");
                if self.last_day_enabled {
                    ui.add(DatePicker::new("schedule_last_day", &mut self.last_day));
                }
            });

            ui.add(
                AccessLevelPicker::new(
                    self.eid.with("access_level"),
                    &mut self.access_level,
                    &state.me.as_ref().unwrap().access_levels,
                )
                .with_label("Access level: "),
            );

            ui.separator();

            ui.add(TimePicker::new(
                "schedule_event_start",
                &mut self.new_event_start,
            ));

            (0..7).for_each(|weekday| {
                let to_delete = ui
                    .horizontal(|ui| {
                        if ui
                            .add_enabled(
                                !self.events[weekday]
                                    .iter()
                                    .any(|e| e.time == self.new_event_start),
                                Button::new("Add"),
                            )
                            .clicked()
                        {
                            dbg!(&self.events[weekday]);
                            dbg!(&self.new_event_start);
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
            ContentUiInfo::new().close_button("Cancel")
                .error(
                    (self.id.is_none() && self.template_id.is_none())
                        .then_some("Template must be set".to_owned()),
                )
                .button(|ui, builder, is_error| {
                    if let Some(id) = self.id {
                        let response = ui.add_enabled(!is_error, egui::Button::new("Create"));
                        if response.clicked() {
                            let events = self.events.iter().flatten().collect::<Vec<_>>();
                            let init_events = self.init_events.clone().unwrap_or(vec![]);
                            let delete_events = init_events
                                .iter()
                                .filter_map(|event_plan| {
                                    (!events.iter().any(|new_event_plan| {
                                        event_plan.weekday == new_event_plan.weekday
                                            && event_plan.time == new_event_plan.time
                                    }))
                                    .then_some(event_plan.id)
                                })
                                .collect::<Vec<_>>();
                            let new_events = events
                                .iter()
                                .filter_map(|&new_event_plan| {
                                    (!init_events.iter().any(|event_plan| {
                                        event_plan.weekday == new_event_plan.weekday
                                            && event_plan.time == new_event_plan.time
                                    }))
                                    .then_some(new_event_plan.clone())
                                })
                                .collect::<Vec<_>>();
                            builder.signal(AppSignal::StateSignal(StateSignal::UpdateSchedule(
                                    UpdateSchedule {
                                        id,
                                        name: USome(self.name.clone()),
                                        description: USome(
                                            (!self.description.is_empty())
                                                .then_some(self.description.clone()),
                                        ),
                                        first_day: USome(self.first_day),
                                        last_day: USome(
                                            self.last_day_enabled.then_some(self.last_day),
                                        ),
                                        access_level: USome(self.access_level),
                                        delete_events,
                                        new_events,
                                    },
                                )));
                        }
                        response
                    } else {
                        let response = ui.add_enabled(!is_error, egui::Button::new("Create"));
                        if response.clicked() {
                            builder.signal(AppSignal::StateSignal(StateSignal::InsertSchedule(
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
                        response
                    }
                })
        })
    }
}
