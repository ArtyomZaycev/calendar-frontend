use super::super::{CalendarApp, CalendarView, EventsView};
use crate::{
    tables::DbTable,
    ui::{
        event_card::EventCard, event_template_card::EventTemplateCard, layout_info::*,
        schedule_card::ScheduleCard, utils::UiUtils,
    },
    utils::*,
};
use chrono::{Datelike, Days, Months, NaiveDate};
use egui::{Align, Layout, RichText};

use num_traits::FromPrimitive;

impl CalendarApp {
    pub(super) fn calendar_view_picker(&mut self, ui: &mut egui::Ui, view: CalendarView) {
        ui.with_layout(Layout::left_to_right(Align::TOP), |ui| {
            ui.selectable_header("Events", view.is_events(), || {
                self.set_view(EventsView::Days);
            });
            ui.selectable_header("Schedules", view.is_schedules(), || {
                self.set_view(CalendarView::Schedules);
            });
            ui.selectable_header("Templates", view.is_event_templates(), || {
                self.set_view(CalendarView::EventTemplates);
            });

            ui.with_layout(Layout::right_to_left(Align::TOP), |ui| match view {
                CalendarView::Events(_) => {
                    if ui
                        .add_enabled(
                            !self.popup_manager.is_open_new_event(),
                            egui::Button::new("Add Event"),
                        )
                        .clicked()
                    {
                        self.popup_manager.open_new_event(self.state.get_me().id);
                    }
                }
                CalendarView::Schedules => {
                    if ui
                        .add_enabled(
                            !self.popup_manager.is_open_new_schedule(),
                            egui::Button::new("Add Schedule"),
                        )
                        .clicked()
                    {
                        self.popup_manager.open_new_schedule(self.state.get_me().id);
                    }
                }
                CalendarView::EventTemplates => {
                    if ui
                        .add_enabled(
                            !self.popup_manager.is_open_new_event_template(),
                            egui::Button::new("Add Template"),
                        )
                        .clicked()
                    {
                        self.popup_manager
                            .open_new_event_template(self.state.get_me().id);
                    }
                }
            });
        });
    }

    pub(super) fn events_view_picker(&mut self, ui: &mut egui::Ui, view: EventsView) {
        ui.with_layout(Layout::left_to_right(Align::TOP), |ui| {
            let view_chooser_response = ui
                .horizontal(|ui| {
                    ui.selectable_header("Month", view.is_month(), || {
                        self.set_view(EventsView::Month)
                    });
                    ui.selectable_header("Week", view.is_week(), || {
                        self.set_view(EventsView::Week)
                    });
                    ui.selectable_header("Day", view.is_day(), || self.set_view(EventsView::Day));
                    ui.selectable_header("Events", view.is_days(), || {
                        self.set_view(EventsView::Days)
                    });
                })
                .response;
            let height = view_chooser_response.rect.height();

            ui.add_space(16.);
            ui.allocate_ui_with_layout(
                egui::Vec2::new(f32::INFINITY, height),
                Layout::left_to_right(Align::Center),
                |ui| match view {
                    EventsView::Month => {
                        if ui.small_button("<").clicked() {
                            self.selected_date = self
                                .selected_date
                                .checked_sub_months(Months::new(1))
                                .unwrap();
                        }
                        if ui.small_button(">").clicked() {
                            self.selected_date = self
                                .selected_date
                                .checked_add_months(Months::new(1))
                                .unwrap();
                        }
                        ui.label(self.selected_date.format("%B %Y").to_string());
                        if ui.button("Today").clicked() {
                            self.selected_date = chrono::Local::now().naive_local().date();
                        }
                    }
                    EventsView::Week => {
                        if ui.small_button("<").clicked() {
                            self.selected_date =
                                self.selected_date.checked_sub_days(Days::new(7)).unwrap();
                        }
                        if ui.small_button(">").clicked() {
                            self.selected_date =
                                self.selected_date.checked_add_days(Days::new(7)).unwrap();
                        }
                        let week = self.selected_date.week(chrono::Weekday::Mon);
                        ui.label(format!(
                            "{} - {} {}",
                            week.first_day().format("%B %d"),
                            week.first_day()
                                .checked_add_days(Days::new(7))
                                .unwrap()
                                .format("%B %d"),
                            week.first_day().format("%Y"),
                        ));
                        if ui.button("Today").clicked() {
                            self.selected_date = chrono::Local::now().naive_local().date();
                        }
                    }
                    EventsView::Day => {
                        if ui.small_button("<").clicked() {
                            self.selected_date =
                                self.selected_date.checked_sub_days(Days::new(1)).unwrap();
                        }
                        if ui.small_button(">").clicked() {
                            self.selected_date =
                                self.selected_date.checked_add_days(Days::new(1)).unwrap();
                        }
                        ui.label(self.selected_date.format("%x").to_string());
                        if ui.button("Today").clicked() {
                            self.selected_date = chrono::Local::now().naive_local().date();
                        }
                    }
                    EventsView::Days => {}
                },
            )
        });
    }

    pub(super) fn month_view(&mut self, ui: &mut egui::Ui, date: NaiveDate) {
        let month = date.month();
        let first_day = get_first_month_day_date(&date);
        let first_monday = get_monday(&first_day);

        let column_width = get_width_from_columns(ui, 7);

        let get_weekday_name = if column_width < 120. {
            weekday_human_name_short
        } else {
            weekday_human_name
        };

        let mut signals = vec![];
        ui.horizontal(|ui| {
            (0..7).for_each(|weekday| {
                let weekday = chrono::Weekday::from_u64(weekday).unwrap();
                let weekday_name = get_weekday_name(&weekday);

                ui.vertical(|ui| {
                    ui.set_width(column_width);
                    ui.vertical_centered(|ui| ui.heading(weekday_name));
                });
            });
        });
        let level = self.state.get_access_level().level;
        egui::ScrollArea::vertical().show(ui, |ui| {
            egui::Grid::new("month")
                .num_columns(7)
                .min_col_width(column_width)
                .max_col_width(column_width)
                .show(ui, |ui| {
                    (0..6).for_each(|week| {
                        let monday = first_monday + chrono::Days::new(7 * week);
                        (0..7).for_each(|weekday| {
                            let date = monday + chrono::Days::new(weekday);

                            self.state.prepare_date(self.selected_user_id, date);
                            let events = self.state.get_events_for_date(date);
                            ui.vertical(|ui| {
                                ui.label(
                                    date.format(if date.month() == month { "%e" } else { "%e %b" })
                                        .to_string(),
                                );
                                events.iter().for_each(|event| {
                                    EventCard::new(
                                        &self.state,
                                        &mut signals,
                                        egui::Vec2::new(column_width, 200.),
                                        event,
                                        level,
                                    );
                                });
                            });
                        });
                        ui.end_row();
                    });
                });
        });
        self.parse_signals(signals);
    }

    pub(super) fn week_view(&mut self, ui: &mut egui::Ui, date: NaiveDate) {
        egui::ScrollArea::vertical().show(ui, |ui| {
            let monday = get_monday(&date);
            let column_width = get_width_from_columns(ui, 7);

            let get_weekday_name = if column_width < 120. {
                weekday_human_name_short
            } else {
                weekday_human_name
            };

            ui.horizontal_top(|ui| {
                let mut signals = vec![];
                (0..7).for_each(|weekday| {
                    let date = monday + chrono::Days::new(weekday);
                    let weekday = chrono::Weekday::from_u64(weekday).unwrap();

                    let weekday_name = get_weekday_name(&weekday);

                    ui.vertical(|ui| {
                        ui.set_width(column_width);
                        ui.vertical_centered(|ui| ui.heading(weekday_name));
                        ui.add_space(4.);

                        let level = self.state.get_access_level().level;
                        self.state.prepare_date(self.selected_user_id, date);
                        self.state
                            .get_events_for_date(date)
                            .iter()
                            .for_each(|event| {
                                ui.add(
                                    EventCard::new(
                                        &self.state,
                                        &mut signals,
                                        egui::Vec2::new(column_width, 200.),
                                        &event,
                                        level,
                                    )
                                    .hide_date(),
                                );
                            });
                    });
                });
                self.parse_signals(signals);
            });
        });
    }

    pub(super) fn day_view(&mut self, ui: &mut egui::Ui, date: NaiveDate) {
        egui::ScrollArea::vertical().show(ui, |ui| {
            let column_width = 200.;
            let num_of_columns = get_columns_from_width(ui, column_width);

            let mut signals = vec![];

            let level = self.state.get_access_level().level;
            self.state.prepare_date(self.selected_user_id, date);
            // TODO: Use array_chunks, once it becomes stable
            // https://github.com/rust-lang/rust/issues/100450
            self.state
                .get_events_for_date(date)
                .iter()
                .enumerate()
                .fold(Vec::default(), |mut acc, (i, event)| {
                    if i % num_of_columns as usize == 0 {
                        acc.push(Vec::default());
                    }
                    acc.last_mut().unwrap().push(event);
                    acc
                })
                .into_iter()
                .for_each(|events| {
                    ui.with_layout(Layout::left_to_right(Align::TOP), |ui| {
                        events.into_iter().for_each(|event| {
                            ui.add(EventCard::new(
                                &self.state,
                                &mut signals,
                                egui::Vec2::new(column_width, 200.),
                                &event,
                                level,
                            ));
                        });
                    });
                });

            self.parse_signals(signals);
        });
    }

    pub(super) fn events_view(&mut self, ui: &mut egui::Ui, date: NaiveDate) {
        egui::ScrollArea::vertical().show(ui, |ui| {
            let column_width = 200.;
            let num_of_columns = get_columns_from_width(ui, column_width);

            let mut signals = vec![];

            (-1i64..7).for_each(|day| {
                let date = date
                    .checked_add_signed(chrono::Duration::try_days(day).unwrap())
                    .unwrap();

                let header_text = match day {
                    -1 => date.format("Yesterday (%A %d-%m)").to_string(),
                    0 => date.format("Today (%A %d-%m)").to_string(),
                    1 => date.format("Tomorrow (%A %d-%m)").to_string(),
                    _ => date.format("%A %d-%m").to_string(),
                };

                egui::CollapsingHeader::new(RichText::new(header_text).heading())
                    .default_open(day >= 0)
                    .show_unindented(ui, |ui| {
                        let level = self.state.get_access_level().level;
                        self.state.prepare_date(self.selected_user_id, date);
                        // TODO: Use array_chunks, once it becomes stable
                        // https://github.com/rust-lang/rust/issues/100450
                        self.state
                            .get_events_for_date(date)
                            .iter()
                            .enumerate()
                            .fold(Vec::default(), |mut acc, (i, event)| {
                                if i % num_of_columns as usize == 0 {
                                    acc.push(Vec::default());
                                }
                                acc.last_mut().unwrap().push(event);
                                acc
                            })
                            .into_iter()
                            .for_each(|events| {
                                ui.with_layout(Layout::left_to_right(Align::TOP), |ui| {
                                    events.into_iter().for_each(|event| {
                                        ui.add(EventCard::new(
                                            &self.state,
                                            &mut signals,
                                            egui::Vec2::new(column_width, 200.),
                                            &event,
                                            level,
                                        ));
                                    });
                                });
                            });
                    });
            });

            self.parse_signals(signals);
        });
    }

    pub(super) fn schedules_view(&mut self, ui: &mut egui::Ui) {
        egui::ScrollArea::vertical().show(ui, |ui| {
            let column_width = 200.;
            let num_of_columns = get_columns_from_width(ui, column_width);

            let mut signals = vec![];

            let level = self.state.get_access_level().level;
            // TODO: Use array_chunks, once it becomes stable
            // https://github.com/rust-lang/rust/issues/100450
            self.get_selected_user_state()
                .schedules
                .get_table()
                .get()
                .iter()
                .filter(|s| s.access_level <= level)
                .enumerate()
                .fold(Vec::default(), |mut acc, (i, schedule)| {
                    if i % num_of_columns as usize == 0 {
                        acc.push(Vec::default());
                    }
                    acc.last_mut().unwrap().push(schedule);
                    acc
                })
                .into_iter()
                .for_each(|schedules| {
                    ui.with_layout(Layout::left_to_right(Align::TOP), |ui| {
                        schedules.into_iter().for_each(|schedule| {
                            ui.add(ScheduleCard::new(
                                &self.state,
                                &mut signals,
                                egui::Vec2::new(column_width, 200.),
                                &schedule,
                            ));
                        });
                    });
                });

            self.parse_signals(signals);
        });
    }

    pub(super) fn event_templates_view(&mut self, ui: &mut egui::Ui) {
        egui::ScrollArea::vertical().show(ui, |ui| {
            let column_width = 200.;
            let num_of_columns = get_columns_from_width(ui, column_width);

            let mut signals = vec![];

            let level = self.state.get_access_level().level;
            // TODO: Use array_chunks, once it becomes stable
            // https://github.com/rust-lang/rust/issues/100450
            self.get_selected_user_state()
                .event_templates
                .get_table()
                .get()
                .iter()
                .filter(|s| s.access_level <= level)
                .enumerate()
                .fold(Vec::default(), |mut acc, (i, schedule)| {
                    if i % num_of_columns as usize == 0 {
                        acc.push(Vec::default());
                    }
                    acc.last_mut().unwrap().push(schedule);
                    acc
                })
                .into_iter()
                .for_each(|templates| {
                    ui.with_layout(Layout::left_to_right(Align::TOP), |ui| {
                        templates.into_iter().for_each(|template| {
                            ui.add(EventTemplateCard::new(
                                &self.state,
                                &mut signals,
                                egui::Vec2::new(column_width, 200.),
                                &template,
                            ));
                        });
                    });
                });

            self.parse_signals(signals);
        });
    }
}
