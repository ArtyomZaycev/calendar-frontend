use super::{
    utils::{AdminPanelUserDataView, AdminPanelView, AppView},
    CalendarApp, CalendarView, EventsView,
};
use crate::{
    db::{aliases::UserUtils, request::RequestDescription},
    requests::AppRequestResponse,
    tables::{DbTable, DbTableGetById},
    ui::{
        event_card::EventCard,
        event_template_card::EventTemplateCard,
        layout_info::GridLayoutInfo,
        schedule_card::ScheduleCard,
        table_view::{TableView, TableViewActions},
        utils::UiUtils,
    },
    utils::*,
};
use calendar_lib::api::{
    event_templates::types::EventTemplate, events::types::Event, roles::types::Role,
    schedules::types::Schedule, utils::User,
};
use chrono::{Days, Months, NaiveDate};
use egui::{Align, Layout, RichText, Sense};

use num_traits::FromPrimitive;

impl CalendarApp {
    fn set_view(&mut self, view: impl Into<AppView>) {
        self.view = view.into();
    }
}

impl CalendarApp {
    fn top_panel(&mut self, ui: &mut egui::Ui) {
        ui.with_layout(Layout::left_to_right(Align::TOP), |ui| {
            ui.heading("Calendar");

            if let Some(me) = self.state.get_me() {
                if me.has_role(Role::SuperAdmin) {}
            }

            ui.with_layout(Layout::right_to_left(Align::TOP), |ui| {
                // RTL
                if let Some(me) = self.state.get_me() {
                    let profile = egui::Label::new(&me.name);
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

                if self.state.any_pending_requests() {
                    ui.spinner();
                }
            });
        });
    }

    fn calendar_view_picker(&mut self, ui: &mut egui::Ui, view: CalendarView) {
        ui.with_layout(Layout::left_to_right(Align::TOP), |ui| {
            ui.selectable_header("Events", view.is_events(), || {
                self.set_view(EventsView::Days(chrono::Local::now().naive_local().date()));
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
                        self.popup_manager.open_new_event(None);
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
                        self.popup_manager.open_new_schedule(None);
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
                        self.popup_manager.open_new_event_template(None);
                    }
                }
            });
        });
    }

    fn events_view_picker(&mut self, ui: &mut egui::Ui, view: EventsView) {
        ui.with_layout(Layout::left_to_right(Align::TOP), |ui| {
            let today = chrono::Local::now().naive_local().date();
            ui.selectable_header("Month", view.is_month(), || {
                self.set_view(EventsView::Month(today))
            });
            ui.selectable_header("Week", view.is_week(), || {
                self.set_view(EventsView::Week(today))
            });
            ui.selectable_header("Day", view.is_day(), || {
                self.set_view(EventsView::Day(today))
            });
            ui.selectable_header("Events", view.is_days(), || {
                self.set_view(EventsView::Days(today))
            });

            ui.add_space(16.);
            match view {
                EventsView::Month(date) => {
                    if ui.small_button("<").clicked() {
                        self.set_view(EventsView::Month(
                            date.checked_sub_months(Months::new(1)).unwrap(),
                        ));
                    }
                    ui.label(date.format("%B %Y").to_string());
                    if ui.small_button(">").clicked() {
                        self.set_view(EventsView::Month(
                            date.checked_add_months(Months::new(1)).unwrap(),
                        ));
                    }
                }
                EventsView::Week(date) => {
                    if ui.small_button("<").clicked() {
                        self.set_view(EventsView::Week(
                            date.checked_sub_days(Days::new(7)).unwrap(),
                        ));
                    }
                    ui.label(date.format("%W week %Y").to_string());
                    if ui.small_button(">").clicked() {
                        self.set_view(EventsView::Week(
                            date.checked_add_days(Days::new(7)).unwrap(),
                        ));
                    }
                }
                EventsView::Day(date) => {
                    if ui.small_button("<").clicked() {
                        self.set_view(EventsView::Day(
                            date.checked_sub_days(Days::new(1)).unwrap(),
                        ));
                    }
                    ui.label(date.format("%x").to_string());
                    if ui.small_button(">").clicked() {
                        self.set_view(EventsView::Day(
                            date.checked_add_days(Days::new(1)).unwrap(),
                        ));
                    }
                }
                EventsView::Days(_) => {}
            }
        });
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
                    .checked_add_signed(chrono::Duration::try_days(day).unwrap())
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
                .user_state
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
                .user_state
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

impl CalendarApp {
    fn view_dispatcher(&mut self, ui: &mut egui::Ui) {
        match self.view {
            AppView::Calendar(calendar_view) => {
                self.calendar_view(ui, calendar_view);
                match calendar_view {
                    CalendarView::Events(events_view) => {
                        self.calendar_events_view(ui, events_view);
                        match events_view {
                            EventsView::Month(date) => self.calendar_events_month_view(ui, date),
                            EventsView::Week(date) => self.calendar_events_week_view(ui, date),
                            EventsView::Day(date) => self.calendar_events_day_view(ui, date),
                            EventsView::Days(date) => self.calendar_events_days_view(ui, date),
                        }
                    }
                    CalendarView::Schedules => {
                        self.calendar_schedules_view(ui);
                    }
                    CalendarView::EventTemplates => {
                        self.calendar_event_templates_view(ui);
                    }
                }
                self.calendar_view_end(ui, calendar_view);
            }
            AppView::AdminPanel(admin_panel_view) => {
                self.admin_panel_view(ui, admin_panel_view);
                match admin_panel_view {
                    AdminPanelView::Users { table } => {
                        self.admin_panel_users_view(ui, table);
                    }
                    AdminPanelView::UserData {
                        user_id,
                        view: admin_panel_user_data_view,
                    } => {
                        self.admin_panel_user_data_view(ui, user_id, admin_panel_user_data_view);
                        match admin_panel_user_data_view {
                            AdminPanelUserDataView::Events { table } => {
                                self.admin_panel_events_view(ui, user_id, table)
                            }
                            AdminPanelUserDataView::EventTemplates { table } => {
                                self.admin_panel_event_templates_view(ui, user_id, table)
                            }
                            AdminPanelUserDataView::Schedules { table } => {
                                self.admin_panel_schedules_view(ui, user_id, table)
                            }
                        }
                    }
                }
            }
        }
    }

    fn calendar_view(&mut self, ui: &mut egui::Ui, view: CalendarView) {
        self.calendar_view_picker(ui, view);
    }

    fn calendar_view_end(&mut self, ui: &mut egui::Ui, _view: CalendarView) {
        ui.add_space(8.);
    }

    fn calendar_events_view(&mut self, ui: &mut egui::Ui, view: EventsView) {
        self.events_view_picker(ui, view);
    }

    fn calendar_events_month_view(&mut self, ui: &mut egui::Ui, date: NaiveDate) {
        self.month_view(ui, date);
    }

    fn calendar_events_week_view(&mut self, ui: &mut egui::Ui, date: NaiveDate) {
        self.week_view(ui, date);
    }

    fn calendar_events_day_view(&mut self, ui: &mut egui::Ui, date: NaiveDate) {
        self.day_view(ui, date);
    }

    fn calendar_events_days_view(&mut self, ui: &mut egui::Ui, date: NaiveDate) {
        self.events_view(ui, date);
    }

    fn calendar_schedules_view(&mut self, ui: &mut egui::Ui) {
        self.schedules_view(ui);
    }

    fn calendar_event_templates_view(&mut self, ui: &mut egui::Ui) {
        self.event_templates_view(ui);
    }

    fn admin_panel_view(&mut self, ui: &mut egui::Ui, _view: AdminPanelView) {
        ui.horizontal(|ui| {
            ui.heading("Admin Panel");
            /*egui::ComboBox::from_id_source("admin panel view picker")
            .selected_text(match view {
                AdminPanelView::Users { table: _ } => "Users",
            })
            .show_ui(ui, |ui| {
                let mut view = view;
                ui.selectable_value(
                    &mut view,
                    AdminPanelView::Users {
                        table: TableView::new("users_table"),
                    },
                    "Users",
                );
                self.set_view(view);
            });*/
        });
    }

    fn admin_panel_users_view(&mut self, ui: &mut egui::Ui, table: TableView<User>) {
        let actions = table
            .show(
                ui,
                self.state.admin_state.users.get_table().get(),
                Some(TableViewActions::new(
                    vec![(0, "Data".to_owned())],
                    |user: &User| user.id,
                )),
            )
            .inner;

        actions
            .actions
            .into_iter()
            .for_each(|(act, user_id)| match act {
                0 => {
                    self.set_view(AdminPanelView::UserData {
                        user_id,
                        view: AdminPanelUserDataView::Events {
                            table: TableView::new("admin_events_table"),
                        },
                    });
                }
                _ => {}
            });
    }

    fn admin_panel_user_data_view(
        &mut self,
        ui: &mut egui::Ui,
        user_id: i32,
        view: AdminPanelUserDataView,
    ) {
        ui.horizontal(|ui| {
            let user = self.state.admin_state.users.get_table().get_by_id(user_id);
            let user_name = user.map(|u| u.name.clone()).unwrap_or_default();
            if ui.button("Back").clicked() {
                self.set_view(AdminPanelView::Users {
                    table: TableView::new("users_table"),
                });
            }
            ui.label(format!("'{user_name}' User Data"));

            egui::ComboBox::from_id_source("admin_panel_user_data_view_combobox")
                .selected_text(match view {
                    AdminPanelUserDataView::Events { .. } => "Events",
                    AdminPanelUserDataView::EventTemplates { .. } => "Event Templates",
                    AdminPanelUserDataView::Schedules { .. } => "Schedules",
                })
                .show_ui(ui, |ui| {
                    let mut view = view;
                    ui.selectable_value(
                        &mut view,
                        AdminPanelUserDataView::Events {
                            table: TableView::new("events_table"),
                        },
                        "Events",
                    );
                    ui.selectable_value(
                        &mut view,
                        AdminPanelUserDataView::EventTemplates {
                            table: TableView::new("event_templates_table"),
                        },
                        "Event Templates",
                    );
                    ui.selectable_value(
                        &mut view,
                        AdminPanelUserDataView::Schedules {
                            table: TableView::new("schedules_table"),
                        },
                        "Schedules",
                    );
                    self.set_view(AdminPanelView::UserData { user_id, view });
                });

            if ui.button("Reload").clicked() {
                self.state.admin_state.load_user_state(user_id);
            }
        });
    }

    fn admin_panel_events_view(
        &mut self,
        ui: &mut egui::Ui,
        user_id: i32,
        table: TableView<Event>,
    ) {
        if ui
            .add_enabled(
                !self.popup_manager.is_open_new_event(),
                egui::Button::new("Add Event"),
            )
            .clicked()
        {
            self.popup_manager.open_new_event(Some(user_id));
        }
        if let Some(user_state) = self.state.admin_state.users_data.get(&user_id) {
            let actions = table
                .show(
                    ui,
                    user_state.events.get_table().get(),
                    Some(TableViewActions::new(
                        vec![(0, "Delete".to_owned())],
                        |event: &Event| event.id,
                    )),
                )
                .inner;

            actions.actions.into_iter().for_each(|(act, id)| match act {
                0 => {
                    user_state.events.delete(id);
                }
                _ => {}
            });
        } else {
            self.state.admin_state.load_user_state(user_id);
            todo!("This will load state multiple times");
        }
    }

    fn admin_panel_event_templates_view(
        &mut self,
        ui: &mut egui::Ui,
        user_id: i32,
        table: TableView<EventTemplate>,
    ) {
        if ui
            .add_enabled(
                !self.popup_manager.is_open_new_event_template(),
                egui::Button::new("Add Template"),
            )
            .clicked()
        {
            self.popup_manager.open_new_event_template(Some(user_id));
        }
        if let Some(user_state) = self.state.admin_state.users_data.get(&user_id) {
            let actions = table
                .show(
                    ui,
                    user_state.event_templates.get_table().get(),
                    Some(TableViewActions::new(
                        vec![(0, "Delete".to_owned())],
                        |template: &EventTemplate| template.id,
                    )),
                )
                .inner;

            actions.actions.into_iter().for_each(|(act, id)| match act {
                0 => {
                    user_state.event_templates.delete(id);
                }
                _ => {}
            });
        } else {
            self.state.admin_state.load_user_state(user_id);
            todo!("This will load state multiple times");
        }
    }

    fn admin_panel_schedules_view(
        &mut self,
        ui: &mut egui::Ui,
        user_id: i32,
        table: TableView<Schedule>,
    ) {
        if ui
            .add_enabled(
                !self.popup_manager.is_open_new_schedule(),
                egui::Button::new("Add Schedule"),
            )
            .clicked()
        {
            self.popup_manager.open_new_schedule(Some(user_id));
        }
        if let Some(user_state) = self.state.admin_state.users_data.get(&user_id) {
            let actions: crate::ui::table_view::TableViewResponse = table
                .show(
                    ui,
                    user_state.schedules.get_table().get(),
                    Some(TableViewActions::new(
                        vec![(0, "Delete".to_owned())],
                        |schedule: &Schedule| schedule.id,
                    )),
                )
                .inner;

            actions.actions.into_iter().for_each(|(act, id)| match act {
                0 => {
                    user_state.schedules.delete(id);
                }
                _ => {}
            });
        } else {
            self.state.admin_state.load_user_state(user_id);
            todo!("This will load state multiple times");
        }
    }
}

impl eframe::App for CalendarApp {
    fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
        // Admins have different view
        if self.state.get_me().is_some_and(|me| me.is_admin()) && self.view.is_calendar() {
            self.view = AppView::AdminPanel(AdminPanelView::Users {
                table: TableView::new("users_table"),
            });
        }

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
                    self.view_dispatcher(ui);
                });
            }
        });

        self.state.update();
        /* TODO
        polled.iter().for_each(
            |&request_id| match self.state.connector.get_response(request_id) {
                Some(AppRequestResponse::Login(response)) => {
                    self.local_storage.store_jwt(response.jwt);
                }
                _ => {}
            },
        );*/
    }
}
