use super::super::{
    utils::{AdminPanelUserDataView, AdminPanelView, AppView},
    CalendarApp, CalendarView, EventsView,
};
use crate::{
    db::aliases::UserUtils,
    state::custom_requests::LoginRequest,
    ui::{popups::popup_manager::PopupManager, table_view::TableView},
};
use chrono::NaiveDate;
use egui::{Align, Layout, Sense};

impl CalendarApp {
    fn top_panel(&mut self, ui: &mut egui::Ui) {
        ui.with_layout(Layout::left_to_right(Align::TOP), |ui| {
            ui.heading("Calendar");

            ui.with_layout(Layout::right_to_left(Align::TOP), |ui| {
                // RTL
                if let Some(me) = self.state.try_get_me() {
                    let profile = egui::Label::new(&me.name);
                    if PopupManager::get().is_open_profile() {
                        ui.add(profile);
                    } else {
                        if ui.add(profile.sense(Sense::click())).clicked() {
                            PopupManager::get().open_profile();
                        }
                    }
                    if ui.button("Logout").clicked() {
                        self.logout();
                    }
                } else {
                    if ui
                        .add_enabled(
                            !PopupManager::get().is_open_login(),
                            egui::Button::new("Login"),
                        )
                        .clicked()
                    {
                        PopupManager::get().open_login();
                    }
                    if ui
                        .add_enabled(
                            !PopupManager::get().is_open_sign_up(),
                            egui::Button::new("Sign Up"),
                        )
                        .clicked()
                    {
                        PopupManager::get().open_sign_up();
                    }
                }

                if self.state.any_pending_requests() {
                    ui.spinner();
                }
            });
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
                            EventsView::Month => {
                                self.calendar_events_month_view(ui, self.selected_date)
                            }
                            EventsView::Week => {
                                self.calendar_events_week_view(ui, self.selected_date)
                            }
                            EventsView::Day => {
                                self.calendar_events_day_view(ui, self.selected_date)
                            }
                            EventsView::Days => {
                                self.calendar_events_days_view(ui, self.selected_date)
                            }
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
}

impl eframe::App for CalendarApp {
    fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
        // Admins have different view
        if self.state.get_me().is_admin() && self.view.is_calendar() {
            self.view = AppView::AdminPanel(AdminPanelView::Users {
                table: TableView::new("users_table"),
            });
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            PopupManager::get().show(&self, ctx);
            let signals = PopupManager::get().get_signals();
            self.parse_signals(signals);
            PopupManager::get().update();

            self.top_panel(ui);
            ui.separator();

            // CALENDAR
            if self.state.try_get_me().is_some() {
                ui.with_layout(Layout::top_down_justified(Align::LEFT), |ui| {
                    self.view_dispatcher(ui);
                });
            }
        });

        self.state.update();
        if let Some(Ok(login_response)) = self.state.find_response_by_type::<LoginRequest>() {
            self.local_storage.store_jwt(login_response.jwt.clone());
        }
    }
}
