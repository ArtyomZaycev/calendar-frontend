use super::{
    popup::PopupType,
    popup_content::{ContentInfo, PopupContent},
};
use crate::{
    app::CalendarApp,
    db::request::RequestIdentifier,
    state::table_requests::{TableInsertRequest, TableUpdateRequest},
    tables::DbTable,
    ui::{access_level_picker::AccessLevelPicker, time_picker::TimePicker},
    utils::weekday_human_name,
};
use calendar_lib::api::{schedules::types::*, utils::*};
use chrono::{Days, Local, NaiveDate, NaiveTime, Weekday};
use egui::{Button, TextEdit};
use egui_extras::DatePickerButton;
use itertools::Itertools;
use num_traits::FromPrimitive;
use std::hash::Hash;

pub struct ScheduleInput {
    eid: egui::Id,
    pub orig_name: String,
    pub user_id: i32,

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

    update_request: Option<RequestIdentifier<TableUpdateRequest<Schedule>>>,
    insert_request: Option<RequestIdentifier<TableInsertRequest<Schedule>>>,
}

impl ScheduleInput {
    pub fn new(eid: impl Hash, user_id: TableId) -> Self {
        let now = Local::now().naive_local();
        let minutes = now
            .time()
            .signed_duration_since(Default::default())
            .num_minutes() as u32;
        let now_time = NaiveTime::from_hms_opt(minutes / 60, minutes % 60, 0).unwrap();

        Self {
            eid: egui::Id::new(eid),
            orig_name: String::default(),
            user_id,
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

            update_request: None,
            insert_request: None,
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
            orig_name: schedule.name.clone(),
            user_id: schedule.user_id,
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
                    let weekday_ind = event.weekday.num_days_from_monday() as usize;
                    acc[weekday_ind].push(NewEventPlan {
                        weekday: event.weekday,
                        time: event.time,
                    });
                    acc[weekday_ind].sort_by_key(|e| e.time);
                    acc
                }),

            update_request: None,
            insert_request: None,
        }
    }
}

impl PopupContent for ScheduleInput {
    fn get_type(&self) -> PopupType {
        if self.id.is_some() {
            PopupType::UpdateSchedule
        } else {
            PopupType::NewSchedule
        }
    }

    fn init_frame(&mut self, app: &CalendarApp, info: &mut ContentInfo) {
        if let Some(identifier) = self.update_request.as_ref() {
            if let Some(response_info) = app.state.get_response(&identifier) {
                self.update_request = None;
                if !response_info.is_err() {
                    info.close();
                }
            }
        }
        if let Some(identifier) = self.insert_request.as_ref() {
            if let Some(response_info) = app.state.get_response(&identifier) {
                self.insert_request = None;
                if !response_info.is_err() {
                    info.close();
                }
            }
        }

        if self.access_level == -1 {
            self.access_level = app.get_selected_access_level();
        }
    }

    fn get_title(&mut self) -> Option<String> {
        if self.id.is_some() {
            Some(format!("Change '{}' Schedule", self.orig_name))
        } else {
            Some("New Schedule".to_owned())
        }
    }

    fn show_content(&mut self, app: &CalendarApp, ui: &mut egui::Ui, info: &mut ContentInfo) {
        ui.vertical(|ui| {
            ui.add(TextEdit::singleline(&mut self.name).hint_text("Name"));
            ui.add(TextEdit::multiline(&mut self.description).hint_text("Description"));

            if self.id.is_none() {
                egui::ComboBox::from_id_source("schedule_template_list")
                    .selected_text(
                        match self.template_id.and_then(|template_id| {
                            app.state
                                .get_user_state(self.user_id)
                                .event_templates
                                .get_table()
                                .get()
                                .iter()
                                .find(|t| t.id == template_id)
                        }) {
                            Some(template) => &template.name,
                            None => "Template",
                        },
                    )
                    .show_ui(ui, |ui| {
                        app.state
                            .get_user_state(self.user_id)
                            .event_templates
                            .get_table()
                            .get()
                            .iter()
                            .for_each(|template| {
                                ui.selectable_value(
                                    &mut self.template_id,
                                    Some(template.id),
                                    &template.name,
                                );
                            });
                    });
            }

            egui::Grid::new(self.eid.with("time_grid")).show(ui, |ui| {
                ui.label("First day:");
                ui.add(
                    DatePickerButton::new(&mut self.first_day)
                        .id_source("first_day")
                        .show_icon(false),
                );
                ui.end_row();

                if self.last_day_enabled && self.first_day > self.last_day {
                    self.last_day = self.first_day;
                }

                ui.label("Last day:");
                ui.add_enabled(
                    self.last_day_enabled,
                    DatePickerButton::new(&mut self.last_day)
                        .id_source("last_day")
                        .show_icon(false),
                );
                ui.checkbox(&mut self.last_day_enabled, "");
                ui.end_row();
            });

            ui.horizontal(|ui| {
                ui.label("Access level: ");
                ui.add(AccessLevelPicker::new(
                    self.eid.with("access_level"),
                    &mut self.access_level,
                    app.state
                        .get_user_state(self.user_id)
                        .access_levels
                        .get_table()
                        .get(),
                ));
            });

            ui.separator();

            ui.add(TimePicker::new(
                "schedule_event_start",
                &mut self.new_event_start,
            ));

            egui::Grid::new(self.eid.with("weekday_grid"))
                .min_col_width(0.)
                .show(ui, |ui| {
                    (0..7).for_each(|weekday_ind| {
                        let weekday = Weekday::from_usize(weekday_ind).unwrap();
                        let mut to_delete = vec![];

                        ui.label(weekday_human_name(weekday));
                        if ui
                            .add_enabled(
                                !self.events[weekday_ind]
                                    .iter()
                                    .any(|e| e.time == self.new_event_start),
                                Button::new("Add"),
                            )
                            .clicked()
                        {
                            self.events[weekday_ind].push(NewEventPlan {
                                weekday,
                                time: self.new_event_start,
                            });
                            self.events[weekday_ind].sort_by_key(|e| e.time);
                        }
                        ui.add_space(4.);
                        self.events[weekday_ind].iter().enumerate().for_each(
                            |(i, new_event_plan)| {
                                ui.spacing_mut().item_spacing = egui::Vec2::default();
                                ui.label(new_event_plan.time.format("%H:%M").to_string());
                                if ui.small_button("X").clicked() {
                                    to_delete.push(i);
                                }
                            },
                        );

                        to_delete.into_iter().rev().for_each(|i| {
                            self.events[weekday_ind].remove(i);
                        });
                        ui.end_row();
                    });
                });

            info.error(self.name.is_empty(), "Name cannot be empty");
            info.error(self.name.len() > 200, "Name is too long");
            info.error(
                self.id.is_none() && self.template_id.is_none(),
                "Template must be set",
            );
        });
    }

    fn show_buttons(&mut self, app: &CalendarApp, ui: &mut egui::Ui, info: &mut ContentInfo) {
        if let Some(id) = self.id {
            if ui
                .add_enabled(!info.is_error(), egui::Button::new("Save"))
                .clicked()
            {
                let events = self.events.iter().flatten().collect_vec();
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
                    .collect_vec();
                let new_events = events
                    .iter()
                    .filter_map(|&new_event_plan| {
                        (!init_events.iter().any(|event_plan| {
                            event_plan.weekday == new_event_plan.weekday
                                && event_plan.time == new_event_plan.time
                        }))
                        .then_some(new_event_plan.clone())
                    })
                    .collect_vec();
                self.update_request = Some(
                    app.state
                        .get_user_state(self.user_id)
                        .schedules
                        .update(UpdateSchedule {
                            id,
                            name: USome(self.name.clone()),
                            description: USome(
                                (!self.description.is_empty()).then_some(self.description.clone()),
                            ),
                            first_day: USome(self.first_day),
                            last_day: USome(self.last_day_enabled.then_some(self.last_day)),
                            access_level: USome(self.access_level),
                            delete_events,
                            new_events,
                        }),
                );
            }
        } else {
            if ui
                .add_enabled(!info.is_error(), egui::Button::new("Create"))
                .clicked()
            {
                self.insert_request = Some(
                    app.state
                        .get_user_state(self.user_id)
                        .schedules
                        .insert(NewSchedule {
                            user_id: self.user_id,
                            template_id: self.template_id.unwrap(),
                            name: self.name.clone(),
                            description: (!self.description.is_empty())
                                .then_some(self.description.clone()),
                            first_day: self.first_day,
                            last_day: self.last_day_enabled.then_some(self.last_day),
                            access_level: self.access_level,
                            events: self.events.clone().into_iter().flatten().collect(),
                        }),
                );
            }
        }
        if ui.button("Cancel").clicked() {
            info.close();
        }
    }
}
