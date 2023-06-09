use super::{CalendarApp, CalendarView, EventsView};
use crate::{
    requests::AppRequestResponse,
    ui::{
        event_card::EventCard, event_template_card::EventTemplateCard, layout_info::GridLayoutInfo,
        schedule_card::ScheduleCard, utils::UiUtils,
    },
    utils::*,
};
use chrono::{Days, Months, NaiveDate};
use egui::{Align, Layout, RichText, Sense};
use num_traits::FromPrimitive;

impl CalendarApp {
    fn top_panel(&mut self, ui: &mut egui::Ui) {
        ui.with_layout(Layout::left_to_right(Align::TOP), |ui| {
            ui.heading("Calendar");

            ui.with_layout(Layout::right_to_left(Align::TOP), |ui| {
                // RTL
                if let Some(me) = &self.state.get_me() {
                    let profile = egui::Label::new(&me.user.name);
                    if self.popup_manager.is_open_profile() {
                        ui.add(profile);
                    } else {
                        if ui.add(profile.sense(Sense::click())).clicked() {
                            self.popup_manager.open_profile();
                        }
                    }
                    if ui.button("Logout").clicked() {
                        self.logout();
                    }
                } else {
                    if ui
                        .add_enabled(
                            !self.popup_manager.is_open_login(),
                            egui::Button::new("Login"),
                        )
                        .clicked()
                    {
                        self.popup_manager.open_login();
                    }
                    if ui
                        .add_enabled(
                            !self.popup_manager.is_open_sign_up(),
                            egui::Button::new("Sign Up"),
                        )
                        .clicked()
                    {
                        self.popup_manager.open_sign_up();
                    }
                }

                if self.state.connector.any_request_in_progress() {
                    ui.spinner();
                }
            });
        });
    }

    fn view_picker(&mut self, ui: &mut egui::Ui) {
        ui.with_layout(Layout::left_to_right(Align::TOP), |ui| {
            ui.selectable_header("Events", self.view.is_events(), || {
                self.view = CalendarView::Events(EventsView::Days(
                    chrono::Local::now().naive_local().date(),
                ))
            });
            ui.selectable_header("Schedules", self.view.is_schedules(), || {
                self.view = CalendarView::Schedules
            });
            ui.selectable_header("Templates", self.view.is_event_templates(), || {
                self.view = CalendarView::EventTemplates
            });

            ui.with_layout(Layout::right_to_left(Align::TOP), |ui| match self.view {
                CalendarView::Events(_) => {
                    if ui
                        .add_enabled(
                            !self.popup_manager.is_open_new_event(),
                            egui::Button::new("Add Event"),
                        )
                        .clicked()
                    {
                        self.popup_manager.open_new_event();
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
                        self.popup_manager.open_new_schedule();
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
                        self.popup_manager.open_new_event_template();
                    }
                }
            });
        });

        match &self.view {
            CalendarView::Events(_) => self.events_view_picker(ui),
            _ => {}
        }
        ui.add_space(8.);
    }

    fn events_view_picker(&mut self, ui: &mut egui::Ui) {
        if let CalendarView::Events(view) = &mut self.view {
            ui.with_layout(Layout::left_to_right(Align::TOP), |ui| {
                let today = chrono::Local::now().naive_local().date();
                ui.selectable_header("Month", view.is_month(), || {
                    *view = EventsView::Month(today)
                });
                ui.selectable_header("Week", view.is_week(), || *view = EventsView::Week(today));
                ui.selectable_header("Day", view.is_day(), || *view = EventsView::Day(today));
                ui.selectable_header("Events", view.is_days(), || *view = EventsView::Days(today));

                ui.add_space(16.);
                match view {
                    EventsView::Month(date) => {
                        if ui.small_button("<").clicked() {
                            *date = date.checked_sub_months(Months::new(1)).unwrap();
                        }
                        ui.label(date.format("%B %Y").to_string());
                        if ui.small_button(">").clicked() {
                            *date = date.checked_add_months(Months::new(1)).unwrap();
                        }
                    }
                    EventsView::Week(date) => {
                        if ui.small_button("<").clicked() {
                            *date = date.checked_sub_days(Days::new(7)).unwrap();
                        }
                        ui.label(date.format("%W week %Y").to_string());
                        if ui.small_button(">").clicked() {
                            *date = date.checked_add_days(Days::new(7)).unwrap();
                        }
                    }
                    EventsView::Day(date) => {
                        if ui.small_button("<").clicked() {
                            *date = date.checked_sub_days(Days::new(1)).unwrap();
                        }
                        ui.label(date.format("%x").to_string());
                        if ui.small_button(">").clicked() {
                            *date = date.checked_add_days(Days::new(1)).unwrap();
                        }
                    }
                    EventsView::Days(_) => {}
                }
            });
        }
    }

    fn main_view(&mut self, ui: &mut egui::Ui) {
        match &self.view {
            CalendarView::Events(view) => match view {
                EventsView::Month(date) => self.month_view(ui, *date),
                EventsView::Week(date) => self.week_view(ui, *date),
                EventsView::Day(date) => self.day_view(ui, *date),
                EventsView::Days(date) => self.events_view(ui, *date),
            },
            CalendarView::Schedules => self.schedules_view(ui),
            CalendarView::EventTemplates => self.event_templates_view(ui),
        }
    }

    fn month_view(&mut self, ui: &mut egui::Ui, date: NaiveDate) {
        let first_day = get_first_month_day_date(&date);
        let last_day = get_last_month_day_date(&date);
        let first_monday = get_monday(&first_day);

        let GridLayoutInfo { column_width, .. } = GridLayoutInfo::from_columns(ui, 7);

        let signals = vec![];
        ui.horizontal(|ui| {
            (0..7).for_each(|weekday| {
                let weekday = chrono::Weekday::from_u64(weekday).unwrap();
                let weekday_name = weekday_human_name(&weekday);

                ui.vertical(|ui| {
                    ui.set_width(column_width);
                    ui.vertical_centered(|ui| ui.heading(weekday_name));
                });
            });
        });
        egui::ScrollArea::vertical().show(ui, |ui| {
            egui::Grid::new("month")
                .num_columns(7)
                .min_col_width(column_width)
                .max_col_width(column_width)
                .show(ui, |ui| {
                    (0..5).for_each(|week| {
                        let monday = first_monday + chrono::Days::new(7 * week);
                        (0..7).for_each(|weekday| {
                            let date = monday + chrono::Days::new(weekday);

                            let events = self.state.get_prepared_events_for_date(date).len();
                            ui.vertical(|ui| {
                                ui.label(date.to_string());
                                if first_day <= date && date <= last_day {
                                    ui.label(events.to_string());
                                } else {
                                    ui.label("");
                                }
                            });
                        });
                        ui.end_row();
                    });
                });
        });
        self.parse_signals(signals);
    }

    fn week_view(&mut self, ui: &mut egui::Ui, date: NaiveDate) {
        egui::ScrollArea::vertical().show(ui, |ui| {
            let monday = get_monday(&date);
            let GridLayoutInfo { column_width, .. } = GridLayoutInfo::from_columns(ui, 7);
            ui.horizontal_top(|ui| {
                let mut signals = vec![];
                (0..7).for_each(|weekday| {
                    let date = monday + chrono::Days::new(weekday);
                    let weekday = chrono::Weekday::from_u64(weekday).unwrap();

                    let weekday_name = weekday_human_name(&weekday);

                    ui.vertical(|ui| {
                        ui.set_width(column_width);
                        ui.vertical_centered(|ui| ui.heading(weekday_name));
                        ui.add_space(4.);

                        let level = self.state.get_access_level().level;
                        self.state
                            .get_prepared_events_for_date(date)
                            .iter()
                            .for_each(|event| {
                                ui.add(
                                    EventCard::new(
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

    fn day_view(&mut self, ui: &mut egui::Ui, date: NaiveDate) {
        egui::ScrollArea::vertical().show(ui, |ui| {
            let GridLayoutInfo {
                num_of_columns,
                column_width,
            } = GridLayoutInfo::from_desired_width(ui, 200.);

            let mut signals = vec![];

            let level = self.state.get_access_level().level;
            // TODO: Use array_chunks, once it becomes stable
            // https://github.com/rust-lang/rust/issues/100450
            self.state
                .get_prepared_events_for_date(date)
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

    fn events_view(&mut self, ui: &mut egui::Ui, date: NaiveDate) {
        egui::ScrollArea::vertical().show(ui, |ui| {
            let GridLayoutInfo {
                num_of_columns,
                column_width,
            } = GridLayoutInfo::from_desired_width(ui, 200.);

            let mut signals = vec![];

            (-1i64..4).for_each(|day| {
                let date = date
                    .checked_add_signed(chrono::Duration::days(day))
                    .unwrap();

                let header_text = match day {
                    -1 => date.format("Yesterday (%A %Y-%m-%d)").to_string(),
                    0 => date.format("Today (%A %Y-%m-%d)").to_string(),
                    1 => date.format("Tomorrow (%A %Y-%m-%d)").to_string(),
                    _ => date.format("%A %Y-%m-%d").to_string(),
                };

                egui::CollapsingHeader::new(RichText::new(header_text).heading())
                    .default_open(day >= 0)
                    .show_unindented(ui, |ui| {
                        let level = self.state.get_access_level().level;
                        // TODO: Use array_chunks, once it becomes stable
                        // https://github.com/rust-lang/rust/issues/100450
                        self.state
                            .get_prepared_events_for_date(date)
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

    fn schedules_view(&mut self, ui: &mut egui::Ui) {
        egui::ScrollArea::vertical().show(ui, |ui| {
            let GridLayoutInfo {
                num_of_columns,
                column_width,
            } = GridLayoutInfo::from_desired_width(ui, 200.);

            let mut signals = vec![];

            let level = self.state.get_access_level().level;
            // TODO: Use array_chunks, once it becomes stable
            // https://github.com/rust-lang/rust/issues/100450
            self.state
                .get_schedules()
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

    fn event_templates_view(&mut self, ui: &mut egui::Ui) {
        egui::ScrollArea::vertical().show(ui, |ui| {
            let GridLayoutInfo {
                num_of_columns,
                column_width,
            } = GridLayoutInfo::from_desired_width(ui, 200.);

            let mut signals = vec![];

            let level = self.state.get_access_level().level;
            // TODO: Use array_chunks, once it becomes stable
            // https://github.com/rust-lang/rust/issues/100450
            self.state
                .get_event_templates()
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

impl eframe::App for CalendarApp {
    fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
        let polled = self.state.poll();
        // TODO: separate function
        polled.iter().for_each(
            |&request_id| match self.state.connector.get_response(request_id) {
                Some(AppRequestResponse::Login(response)) => {
                    self.local_storage.store_user_id(response.user.id);
                    self.local_storage.store_key(&response.key);
                }
                _ => {}
            },
        );

        egui::CentralPanel::default().show(ctx, |ui| {
            self.popup_manager.show(&self.state, ctx);
            let signals = self.popup_manager.get_signals();
            self.parse_signals(signals);
            self.popup_manager.update();

            self.top_panel(ui);
            ui.separator();

            // CALENDAR
            if let Some(_me) = &self.state.get_me() {
                ui.with_layout(Layout::top_down_justified(Align::LEFT), |ui| {
                    self.view_picker(ui);
                    self.main_view(ui);
                });
            }
        });
    }
}