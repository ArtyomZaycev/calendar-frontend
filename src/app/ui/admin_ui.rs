use super::super::{
    utils::{AdminPanelUserDataView, AdminPanelView},
    CalendarApp,
};
use crate::{
    tables::{DbTable, DbTableGetById},
    ui::{
        popups::popup_manager::PopupManager,
        table_view::{TableView, TableViewActions},
    },
};
use calendar_lib::api::{
    event_templates::types::EventTemplate, events::types::Event, schedules::types::Schedule,
    utils::User,
};

impl CalendarApp {
    pub(super) fn admin_panel_view(&mut self, ui: &mut egui::Ui, _view: AdminPanelView) {
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

    pub(super) fn admin_panel_users_view(&mut self, ui: &mut egui::Ui, table: TableView<User>) {
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
                    self.state.admin_state.load_user_state(user_id);
                }
                _ => {}
            });
    }

    pub(super) fn admin_panel_user_data_view(
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

    pub(super) fn admin_panel_events_view(
        &mut self,
        ui: &mut egui::Ui,
        user_id: i32,
        table: TableView<Event>,
    ) {
        if ui
            .add_enabled(
                !PopupManager::get().is_open_new_event(),
                egui::Button::new("Add Event"),
            )
            .clicked()
        {
            PopupManager::get().open_new_event(user_id);
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
            // TODO: Some visual that load is in progress
        }
    }

    pub(super) fn admin_panel_event_templates_view(
        &mut self,
        ui: &mut egui::Ui,
        user_id: i32,
        table: TableView<EventTemplate>,
    ) {
        if ui
            .add_enabled(
                !PopupManager::get().is_open_new_event_template(),
                egui::Button::new("Add Template"),
            )
            .clicked()
        {
            PopupManager::get().open_new_event_template(user_id);
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
            // TODO: Some visual that load is in progress
        }
    }

    pub(super) fn admin_panel_schedules_view(
        &mut self,
        ui: &mut egui::Ui,
        user_id: i32,
        table: TableView<Schedule>,
    ) {
        if ui
            .add_enabled(
                !PopupManager::get().is_open_new_schedule(),
                egui::Button::new("Add Schedule"),
            )
            .clicked()
        {
            PopupManager::get().open_new_schedule(user_id);
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
            // TODO: Some visual that load is in progress
        }
    }
}
