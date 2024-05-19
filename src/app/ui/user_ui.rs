use super::super::{CalendarApp, CalendarView, EventsView};
use crate::{
    tables::DbTable,
    ui::{
        event_card::EventCard, event_template_card::EventTemplateCard, layout_info::*,
        popups::popup_manager::PopupManager, schedule_card::ScheduleCard, utils::UiUtils,
    },
    utils::*,
};
use chrono::{Datelike, Days, Months, NaiveDate, Weekday};
use egui::{Align, Layout, RichText};

use num_traits::FromPrimitive;

impl CalendarApp {
    pub(super) fn calendar_view_picker(&mut self, ui: &mut egui::Ui, view: CalendarView) {
        let permissions = self.get_selected_user_permissions();
        ui.with_layout(Layout::left_to_right(Align::TOP), |ui| {
            let height = ui
                .horizontal(|ui| {
                    ui.enabled_selectable_header(
                        "Events",
                        permissions.events.view,
                        view.is_events(),
                        || {
                            self.set_view(EventsView::Days);
                        },
                    );
                    ui.enabled_selectable_header(
                        "Schedules",
                        permissions.schedules.view,
                        view.is_schedules(),
                        || {
                            self.set_view(CalendarView::Schedules);
                        },
                    );
                    ui.enabled_selectable_header(
                        "Templates",
                        permissions.event_templates.view,
                        view.is_event_templates(),
                        || {
                            self.set_view(CalendarView::EventTemplates);
                        },
                    );
                })
                .response
                .rect
                .height();

            ui.allocate_ui_with_layout(
                egui::Vec2::new(ui.available_width(), height),
                Layout::right_to_left(Align::Center),
                |ui| match view {
                    CalendarView::Events(_) => {
                        if permissions.events.create {
                            if ui
                                .add_enabled(
                                    !PopupManager::get().is_open_new_event(),
                                    egui::Button::new("Create Event"),
                                )
                                .clicked()
                            {
                                PopupManager::get().open_new_event(self.selected_user_id);
                            }
                        }
                    }
                    CalendarView::Schedules => {
                        if permissions.schedules.create {
                            if ui
                                .add_enabled(
                                    !PopupManager::get().is_open_new_schedule(),
                                    egui::Button::new("Create Schedule"),
                                )
                                .clicked()
                            {
                                PopupManager::get().open_new_schedule(self.selected_user_id);
                            }
                        }
                    }
                    CalendarView::EventTemplates => {
                        if permissions.event_templates.create {
                            if ui
                                .add_enabled(
                                    !PopupManager::get().is_open_new_event_template(),
                                    egui::Button::new("Create Template"),
                                )
                                .clicked()
                            {
                                PopupManager::get().open_new_event_template(self.selected_user_id);
                            }
                        }
                    }
                },
            );
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

        ui.horizontal(|ui| {
            (0..7).for_each(|weekday| {
                let weekday = chrono::Weekday::from_u64(weekday).unwrap();
                let weekday_name = get_weekday_name(weekday);

                ui.vertical(|ui| {
                    ui.set_width(column_width);
                    ui.vertical_centered(|ui| ui.heading(weekday_name));
                });
            });
        });
        let level = self.get_selected_access_level();
        let num_of_weeks = if month
            == (first_day + chrono::Days::new(7 * 5))
                .week(Weekday::Mon)
                .first_day()
                .month()
        {
            6
        } else {
            5
        };
        let row_height = get_height_from_rows(ui, num_of_weeks);
        egui::ScrollArea::vertical().show(ui, |ui| {
            egui::Grid::new("month")
                .num_columns(7)
                .min_col_width(column_width)
                .max_col_width(column_width)
                .min_row_height(row_height)
                .show(ui, |ui| {
                    (0..num_of_weeks as u64).for_each(|week| {
                        let monday = first_monday + chrono::Days::new(7 * week);
                        (0..7).for_each(|weekday| {
                            let date = monday + chrono::Days::new(weekday);

                            self.prepare_date(date);
                            let events = self.state.get_events_for_date(date);
                            ui.vertical_centered_justified(|ui| {
                                ui.label(
                                    date.format(if date.month() == month { "%e" } else { "%e %b" })
                                        .to_string(),
                                );
                                let available_height = ui.available_height();
                                let card_height = 24.;
                                let number_of_cards = events.len() as f32;
                                let spacing = ui.style().spacing.item_spacing.y;
                                let need_height = number_of_cards * card_height
                                    + (number_of_cards - 1.).max(0.) * spacing;

                                let hide_some = need_height > available_height;
                                let show_number_of_cards = if hide_some {
                                    (available_height / (card_height + spacing) - 1.).max(0.)
                                        as usize
                                } else {
                                    number_of_cards as usize
                                };

                                events[..show_number_of_cards].iter().for_each(|event| {
                                    ui.add(
                                        EventCard::new(
                                            &self,
                                            egui::Vec2::new(column_width, 200.),
                                            event,
                                            level,
                                            self.get_selected_user_permissions().events,
                                        )
                                        .small(),
                                    );
                                });

                                if hide_some {
                                    ui.menu_button(
                                        format!("{} more", events.len() - show_number_of_cards),
                                        |ui| {
                                            events[show_number_of_cards..].iter().for_each(
                                                |event| {
                                                    ui.add(
                                                        EventCard::new(
                                                            &self,
                                                            egui::Vec2::new(column_width, 200.),
                                                            event,
                                                            level,
                                                            self.get_selected_user_permissions()
                                                                .events,
                                                        )
                                                        .small(),
                                                    );
                                                },
                                            );
                                        },
                                    );
                                }
                            });
                        });
                        ui.end_row();
                    });
                });
        });
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
                (0..7).for_each(|weekday| {
                    let date = monday + chrono::Days::new(weekday);
                    let weekday = chrono::Weekday::from_u64(weekday).unwrap();

                    let weekday_name = get_weekday_name(weekday);

                    ui.vertical(|ui| {
                        ui.set_width(column_width);
                        ui.vertical_centered(|ui| ui.heading(weekday_name));
                        ui.add_space(4.);

                        let level = self.get_selected_access_level();
                        self.prepare_date(date);
                        self.state
                            .get_events_for_date(date)
                            .iter()
                            .for_each(|event| {
                                ui.add(
                                    EventCard::new(
                                        &self,
                                        egui::Vec2::new(column_width, 200.),
                                        &event,
                                        level,
                                        self.get_selected_user_permissions().events,
                                    )
                                    .hide_date(),
                                );
                            });
                    });
                });
            });
        });
    }

    pub(super) fn day_view(&mut self, ui: &mut egui::Ui, date: NaiveDate) {
        egui::ScrollArea::vertical().show(ui, |ui| {
            let column_width = 200.;
            let num_of_columns = get_columns_from_width(ui, column_width);

            let level = self.get_selected_access_level();
            self.prepare_date(date);
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
                                &self,
                                egui::Vec2::new(column_width, 200.),
                                &event,
                                level,
                                self.get_selected_user_permissions().events,
                            ));
                        });
                    });
                });
        });
    }

    pub(super) fn events_view(&mut self, ui: &mut egui::Ui, date: NaiveDate) {
        egui::ScrollArea::vertical().show(ui, |ui| {
            let column_width = 240.;
            let num_of_columns = get_columns_from_width(ui, column_width);

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
                        let level = self.get_selected_access_level();
                        self.prepare_date(date);
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
                                        ui.add(
                                            EventCard::new(
                                                &self,
                                                egui::Vec2::new(column_width, 200.),
                                                &event,
                                                level,
                                                self.get_selected_user_permissions().events,
                                            )
                                            .hide_date(),
                                        );
                                    });
                                });
                            });
                    });
            });
        });
    }

    pub(super) fn schedules_view(&mut self, ui: &mut egui::Ui) {
        egui::ScrollArea::vertical().show(ui, |ui| {
            let column_width = 240.;
            let num_of_columns = get_columns_from_width(ui, column_width);

            let level = self.get_selected_access_level();
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
                                &self,
                                egui::Vec2::new(column_width, 200.),
                                &schedule,
                                self.get_selected_access_level(),
                                self.get_selected_user_permissions().schedules,
                            ));
                        });
                    });
                });
        });
    }

    pub(super) fn event_templates_view(&mut self, ui: &mut egui::Ui) {
        egui::ScrollArea::vertical().show(ui, |ui| {
            let column_width = 240.;
            let num_of_columns = get_columns_from_width(ui, column_width);

            let level = self.get_selected_access_level();
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
                                &self,
                                egui::Vec2::new(column_width, 200.),
                                &template,
                                self.get_selected_access_level(),
                                self.get_selected_user_permissions().event_templates,
                            ));
                        });
                    });
                });
        });
    }
}
