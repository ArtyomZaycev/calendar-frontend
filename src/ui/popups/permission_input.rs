use super::popup_content::{ContentInfo, PopupContent};
use crate::{
    app::CalendarApp,
    db::request::RequestIdentifier,
    state::table_requests::{TableInsertRequest, TableUpdateRequest},
    tables::DbTable,
    utils::is_valid_email,
};
use calendar_lib::api::{
    auth::types::AccessLevel,
    permissions::{self, types::*},
    utils::*,
};
use egui::Checkbox;
use std::hash::Hash;

pub struct PermissionInput {
    eid: egui::Id,
    pub giver_user_id: i32,

    pub id: Option<i32>,
    pub receiver_name: String,
    pub receiver_email: String,

    pub access_level: i32,
    pub events_view: bool,
    pub events_edit: bool,
    pub event_templates_view: bool,
    pub event_templates_edit: bool,
    pub schedules_view: bool,
    pub schedules_edit: bool,
    pub sharing: bool,
    pub access_levels_edit: bool,

    email_not_found: Option<String>,
    update_request: Option<RequestIdentifier<TableUpdateRequest<GrantedPermission>>>,
    insert_request: Option<RequestIdentifier<TableInsertRequest<GrantedPermission>>>,
}

impl PermissionInput {
    pub fn new(eid: impl Hash, giver_user_id: i32) -> Self {
        Self {
            eid: egui::Id::new(eid),

            giver_user_id,
            receiver_name: String::default(),
            receiver_email: String::default(),
            id: None,

            access_level: AccessLevel::MAX_LEVEL,
            events_view: false,
            events_edit: false,
            event_templates_view: false,
            event_templates_edit: false,
            schedules_view: false,
            schedules_edit: false,
            sharing: false,
            access_levels_edit: false,

            email_not_found: None,
            update_request: None,
            insert_request: None,
        }
    }

    pub fn change(eid: impl Hash, permissions: &GrantedPermission, user: &User) -> Self {
        Self {
            eid: egui::Id::new(eid),

            giver_user_id: permissions.giver_user_id,
            receiver_name: user.name.clone(),
            receiver_email: user.email.clone(),
            id: Some(permissions.id),

            access_level: permissions.permissions.access_level,
            events_view: permissions.permissions.events.view,
            events_edit: permissions.permissions.events.edit,
            event_templates_view: permissions.permissions.event_templates.view,
            event_templates_edit: permissions.permissions.event_templates.edit,
            schedules_view: permissions.permissions.schedules.view,
            schedules_edit: permissions.permissions.schedules.edit,
            sharing: permissions.permissions.allow_share,
            access_levels_edit: permissions.permissions.access_levels.edit,

            email_not_found: None,
            update_request: None,
            insert_request: None,
        }
    }

    fn make_permissions(&self) -> Permissions {
        Permissions {
            access_level: self.access_level,
            access_levels: TablePermissions {
                view: self.access_levels_edit,
                edit: self.access_levels_edit,
                create: self.access_levels_edit,
                delete: self.access_levels_edit,
            },
            events: TablePermissions {
                view: self.events_view || self.events_edit,
                edit: self.events_edit,
                create: self.events_edit,
                delete: self.events_edit,
            },
            event_templates: TablePermissions {
                view: self.event_templates_view
                    || self.event_templates_edit
                    || self.schedules_view
                    || self.schedules_edit,
                edit: self.event_templates_edit,
                create: self.event_templates_edit,
                delete: self.event_templates_edit,
            },
            schedules: TablePermissions {
                view: self.schedules_view || self.schedules_edit,
                edit: self.schedules_edit,
                create: self.schedules_edit,
                delete: self.schedules_edit,
            },
            allow_share: self.sharing,
        }
    }
}

impl PopupContent for PermissionInput {
    fn init_frame(&mut self, app: &CalendarApp, info: &mut ContentInfo) {
        if let Some(identifier) = self.update_request.as_ref() {
            if let Some(response_info) = app.state.get_response(&identifier) {
                match response_info {
                    Ok(_) => info.close(),
                    Err(err) => match *err {
                        permissions::update::BadRequestResponse::NotFound => {}
                        permissions::update::BadRequestResponse::UserEmailNotFound => {
                            self.email_not_found = Some(identifier.info.info.1.clone());
                        }
                    },
                }
                self.update_request = None;
            }
        }
        if let Some(identifier) = self.insert_request.as_ref() {
            if let Some(response_info) = app.state.get_response(&identifier) {
                match response_info {
                    Ok(_) => info.close(),
                    Err(err) => match *err {
                        permissions::insert::BadRequestResponse::UserEmailNotFound => {
                            self.email_not_found = Some(identifier.info.info.clone());
                        }
                    },
                }
                self.insert_request = None;
            }
        }
    }

    fn get_title(&mut self) -> Option<String> {
        if self.id.is_some() {
            Some(format!("Change {} Permissions", &self.receiver_name))
        } else {
            Some("Grant Permission".to_owned())
        }
    }

    fn show_content(&mut self, app: &CalendarApp, ui: &mut egui::Ui, info: &mut ContentInfo) {
        let edit_mode = app
            .state
            .get_user_permissions(self.giver_user_id)
            .allow_share;
        ui.vertical(|ui| {
            if self.id.is_none() {
                ui.add(
                    egui::TextEdit::singleline(&mut self.receiver_email)
                        .desired_width(f32::INFINITY)
                        .hint_text("Email"),
                );
                ui.add_space(2.);
            }

            info.error(
                Some(&self.receiver_email) == self.email_not_found.as_ref(),
                "User with this email does not exist",
            );
            info.error(!is_valid_email(&self.receiver_email), "Invalid Email");

            let access_levels = app
                .state
                .get_user_state(self.giver_user_id)
                .access_levels
                .get_table()
                .get();
            let current_access_level = access_levels
                .iter()
                .find(|al| al.level == self.access_level)
                .unwrap_or(access_levels.first().unwrap());
            ui.add_enabled_ui(!self.access_levels_edit, |ui| {
                egui::ComboBox::new(self.eid.with("access_level"), "Access Level")
                    .selected_text(&current_access_level.name)
                    .show_ui(ui, |ui| {
                        access_levels.iter().for_each(|al| {
                            ui.selectable_value(&mut self.access_level, al.level, &al.name);
                        })
                    });
            });

            let mut full_permissions = self.events_view
                && self.events_edit
                && self.event_templates_view
                && self.event_templates_edit
                && self.schedules_view
                && self.schedules_edit
                && self.sharing
                && self.access_levels_edit;
            if ui
                .add_enabled(
                    edit_mode,
                    Checkbox::new(&mut full_permissions, "Full permissions"),
                )
                .clicked()
            {
                self.events_view = full_permissions;
                self.events_edit = full_permissions;
                self.event_templates_view = full_permissions;
                self.event_templates_edit = full_permissions;
                self.schedules_view = full_permissions;
                self.schedules_edit = full_permissions;
                self.sharing = full_permissions;
                self.access_levels_edit = full_permissions;
            }

            ui.heading("Events");
            ui.separator();
            ui.add_enabled(
                edit_mode && !self.events_edit,
                Checkbox::new(&mut self.events_view, "View Events"),
            );
            ui.add_enabled(
                edit_mode,
                Checkbox::new(&mut self.events_edit, "Create and edit Events"),
            );
            if self.events_edit {
                self.events_view = true;
            }

            ui.heading("Event Templates");
            ui.separator();
            ui.add_enabled(
                edit_mode
                    && !self.event_templates_edit
                    && !self.schedules_view
                    && !self.schedules_edit,
                Checkbox::new(&mut self.event_templates_view, "View Event Templates"),
            );
            ui.add_enabled(
                edit_mode,
                Checkbox::new(
                    &mut self.event_templates_edit,
                    "Create and edit Event Templates",
                ),
            );
            if self.event_templates_edit {
                self.event_templates_view = true;
            }

            ui.heading("Schedules");
            ui.separator();
            ui.add_enabled(
                edit_mode && !self.schedules_edit,
                Checkbox::new(&mut self.schedules_view, "View Schedules"),
            );
            ui.add_enabled(
                edit_mode,
                Checkbox::new(&mut self.schedules_edit, "Create and edit Schedules"),
            );
            if self.schedules_view {
                self.event_templates_view = true;
            }
            if self.schedules_edit {
                self.event_templates_view = true;
                self.schedules_view = true;
            }

            ui.heading("Other");
            ui.separator();
            ui.add_enabled(
                edit_mode,
                Checkbox::new(&mut self.sharing, "Manage Sharing"),
            );
            ui.add_enabled(
                edit_mode,
                Checkbox::new(&mut self.access_levels_edit, "Edit Access Levels"),
            );
            if self.access_levels_edit {
                self.access_level = AccessLevel::MAX_LEVEL;
            }
        });
    }

    fn show_buttons(&mut self, app: &CalendarApp, ui: &mut egui::Ui, info: &mut ContentInfo) {
        if let Some(id) = self.id {
            if ui
                .add_enabled(
                    self.update_request.is_none() && !info.is_error(),
                    egui::Button::new("Update"),
                )
                .clicked()
            {
                self.update_request = Some(
                    app.state
                        .get_user_state(self.giver_user_id)
                        .granted_permissions
                        .update_with_info(
                            UpdateGrantedPermission {
                                id,
                                receiver_email: USome(self.receiver_email.clone()),
                                permissions: USome(self.make_permissions()),
                            },
                            self.receiver_email.clone(),
                        ),
                );
            }
        } else {
            if ui
                .add_enabled(
                    self.insert_request.is_none() && !info.is_error(),
                    egui::Button::new("Create"),
                )
                .clicked()
            {
                self.insert_request = Some(
                    app.state
                        .get_user_state(self.giver_user_id)
                        .granted_permissions
                        .insert_with_info(
                            NewGrantedPermission {
                                giver_user_id: self.giver_user_id,
                                receiver_email: self.receiver_email.clone(),
                                permissions: self.make_permissions(),
                            },
                            self.receiver_email.clone(),
                        ),
                );
            }
        }
        if ui.button("Cancel").clicked() {
            info.close();
        }
    }
}
