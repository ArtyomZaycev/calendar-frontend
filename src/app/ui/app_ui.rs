use super::super::{
    view::{AdminPanelUserDataView, AdminPanelView, AppView},
    CalendarApp, CalendarView, EventsView,
};
use crate::{
    app::ManageAccessView,
    db::aliases::UserUtils,
    state::custom_requests::LoginRequest,
    tables::DbTable,
    ui::{
        popups::{popup::PopupType, popup_manager::PopupManager},
        table_view::TableView,
        utils::DirectionSymbol,
    },
};
use chrono::NaiveDate;
use egui::{Align, CollapsingHeader, Direction, Label, Layout, Sense};

impl CalendarApp {
    fn top_panel(&mut self, ui: &mut egui::Ui) {
        ui.with_layout(Layout::left_to_right(Align::TOP), |ui| {
            let calendar_name = if self.state.try_get_me().is_none() {
                "Calendar".to_owned()
            } else if self.selected_user_id == self.state.get_me().id {
                "Your Calendar".to_owned()
            } else {
                match self
                    .state
                    .user_state
                    .users
                    .get_table()
                    .get()
                    .iter()
                    .find(|u| u.id == self.selected_user_id)
                {
                    Some(user) => format!("{} Calendar", user.name),
                    None => "Other Calendar".to_owned(),
                }
            };
            let height = ui.heading(calendar_name).rect.height();

            ui.allocate_ui_with_layout(
                egui::Vec2::new(ui.available_width(), height),
                Layout::right_to_left(Align::Center),
                |ui| {
                    // RTL
                    ui.add_space(8.);

                    if let Some(me) = self.state.try_get_me() {
                        if self.selected_user_id == me.id {
                            let profile = egui::Label::new(&me.name);
                            if PopupManager::get().is_open(PopupType::is_profile) {
                                ui.add(profile);
                            } else {
                                if ui.add(profile.sense(Sense::click())).clicked() {
                                    PopupManager::get().open_profile();
                                }
                            }
                        }
                    } else {
                        if ui
                            .add_enabled(
                                !PopupManager::get().is_open(PopupType::is_login),
                                egui::Button::new("Login"),
                            )
                            .clicked()
                        {
                            PopupManager::get().open_login();
                        }
                        if ui
                            .add_enabled(
                                !PopupManager::get().is_open(PopupType::is_sign_up),
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
                },
            );
        });
    }

    fn burger_menu_collapsed(&mut self, ctx: &egui::Context) {
        egui::SidePanel::left("burger_menu")
            .resizable(false)
            .show_separator_line(true)
            .exact_width(4.)
            .show(ctx, |ui| {
                //ui.add_space(ui.ctx().style().spacing.item_spacing.x * 1.5);
                ui.with_layout(
                    Layout::centered_and_justified(Direction::LeftToRight),
                    |ui| {
                        if ui
                            .add(DirectionSymbol::new(Direction::LeftToRight))
                            .clicked()
                        {
                            self.burger_menu_expanded = true;
                        }
                    },
                );
            });
    }

    fn burger_menu_expanded(&mut self, ctx: &egui::Context) {
        let width = 160.;
        egui::SidePanel::left("burger_menu")
            .resizable(false)
            .show_separator_line(true)
            .exact_width(width)
            .show(ctx, |ui| {
                ui.add_space(ui.ctx().style().spacing.item_spacing.x * 1.5);
                ui.with_layout(Layout::top_down_justified(Align::LEFT), |ui| {
                    ui.with_layout(Layout::left_to_right(Align::TOP), |ui| {
                        let response = ui.add(Label::new("YOUR CALENDAR").sense(Sense::click()));
                        if response.clicked() {
                            self.selected_user_id = self.state.get_me().id;
                            self.state.clear_events(self.selected_user_id);
                            self.view = EventsView::Month.into();
                        }
                        let height = response.rect.height();
                        ui.allocate_ui_with_layout(
                            egui::Vec2::new(ui.available_width(), height),
                            Layout::right_to_left(Align::Center),
                            |ui| {
                                if ui
                                    .add(DirectionSymbol::new(Direction::RightToLeft))
                                    .clicked()
                                {
                                    self.burger_menu_expanded = false;
                                }
                            },
                        );
                    });
                    ui.separator();

                    if !self.state.granted_states.is_empty() {
                        CollapsingHeader::new("SHARED CALENDARS").show(ui, |ui| {
                            let mut changed = false;
                            self.state.granted_states.iter().for_each(|shared_state| {
                                let user_response = ui
                                    .add(Label::new(&shared_state.user.name).sense(Sense::click()));
                                if user_response.clicked() {
                                    self.selected_user_id = shared_state.user.id;
                                    self.view = if shared_state.permissions.events.view {
                                        EventsView::Month.into()
                                    } else if shared_state.permissions.schedules.view {
                                        CalendarView::Schedules.into()
                                    } else if shared_state.permissions.event_templates.view {
                                        CalendarView::EventTemplates.into()
                                    } else if shared_state.permissions.allow_share {
                                        ManageAccessView::Sharing.into()
                                    } else if shared_state.permissions.access_levels.view {
                                        ManageAccessView::AccessLevels.into()
                                    } else {
                                        EventsView::Month.into()
                                    };
                                    changed = true;
                                }
                                if shared_state.permissions.allow_share
                                    || shared_state.permissions.access_levels.view
                                {
                                    user_response.context_menu(|ui| {
                                        if ui.button("Manage Access").clicked() {
                                            self.selected_user_id = shared_state.user.id;
                                            self.view = if shared_state.permissions.allow_share {
                                                ManageAccessView::Sharing
                                            } else {
                                                ManageAccessView::AccessLevels
                                            }
                                            .into();
                                            ui.close_menu();
                                        }
                                    });
                                }
                            });
                            if changed {
                                self.state.clear_events(self.selected_user_id);
                            }
                        });
                        ui.separator();
                    }

                    if ui
                        .add(Label::new("MANAGE ACCESS").sense(Sense::click()))
                        .clicked()
                    {
                        self.selected_user_id = self.state.get_me().id;
                        self.view = AppView::ManageAccess(ManageAccessView::Sharing);
                    }
                    ui.separator();

                    if ui.add(Label::new("LOGOUT").sense(Sense::click())).clicked() {
                        self.logout();
                    }
                });
            });
    }

    fn burger_menu(&mut self, ctx: &egui::Context) {
        if self.burger_menu_expanded {
            self.burger_menu_expanded(ctx);
        } else {
            self.burger_menu_collapsed(ctx);
        }
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
                                self.set_view(EventsView::Month);
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
            AppView::ManageAccess(manage_access_view) => {
                self.manage_access_view(ui, manage_access_view);
                match manage_access_view {
                    ManageAccessView::Sharing => {
                        self.manage_access_sharing_view(ui);
                    }
                    ManageAccessView::AccessLevels => {
                        self.manage_access_access_levels_view(ui);
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
        if self.state.get_me().id != -1 {
            if self.selected_user_id == -1 {
                self.selected_user_id = self.state.get_me().id;
            }
        }

        // Admins have different view
        if self.state.get_me().is_admin() && self.view.is_calendar() {
            self.view = AppView::AdminPanel(AdminPanelView::Users {
                table: TableView::new("users_table"),
            });
        }

        if self.state.try_get_me().is_some() {
            self.burger_menu(ctx);
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            PopupManager::get().show(&self, ctx);
            PopupManager::get().update();

            self.top_panel(ui);
            ui.separator();

            ui.horizontal_top(|ui| {
                if self.state.try_get_me().is_some() {
                    ui.with_layout(Layout::top_down_justified(Align::LEFT), |ui| {
                        self.view_dispatcher(ui);
                    });
                }
            });
        });

        self.state.update();
        if let Some(Ok(login_response)) = self.state.find_response_by_type::<LoginRequest>() {
            self.local_storage.store_jwt(login_response.jwt.clone());
        }
    }
}
