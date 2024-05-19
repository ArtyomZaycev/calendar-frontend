use super::popup_content::{ContentInfo, PopupContent};
use crate::{
    app::CalendarApp,
    db::request::RequestIdentifier,
    state::{custom_requests::ChangeAccessLevelsRequest, state_table::StateTable},
    tables::DbTable,
};
use calendar_lib::api::{
    auth::types::{AccessLevel, AccessLevelChange},
    utils::*,
};
use egui::{Align, Button, Layout, TextEdit, TextStyle, Vec2};
use itertools::Itertools;
use std::hash::Hash;

#[derive(Debug, Clone)]
struct AccessLevelData {
    origin: AccessLevel,
    name: String,
    deleted: bool,
}

impl AccessLevelData {
    fn new(origin: AccessLevel) -> Self {
        Self {
            name: origin.name.clone(),
            origin: origin,
            deleted: false,
        }
    }
}

pub struct ChangeAccessLevelsPopup {
    #[allow(dead_code)]
    eid: egui::Id,
    pub user_id: TableId,

    access_levels: Vec<AccessLevelData>,

    update_request: Option<RequestIdentifier<ChangeAccessLevelsRequest>>,
}

impl ChangeAccessLevelsPopup {
    pub fn new(eid: impl Hash, user_id: TableId, table: &StateTable<AccessLevel>) -> Self {
        Self {
            eid: egui::Id::new(eid),
            user_id,
            access_levels: table
                .get_table()
                .get()
                .iter()
                .sorted_by_key(|al| -al.level)
                .map(|al| AccessLevelData::new(al.clone()))
                .collect(),
            update_request: None,
        }
    }
}

impl PopupContent for ChangeAccessLevelsPopup {
    fn init_frame(&mut self, app: &CalendarApp, info: &mut ContentInfo) {
        if let Some(identifier) = self.update_request.as_ref() {
            if let Some(response_info) = app.state.get_response(&identifier) {
                if !response_info.is_err() {
                    info.close();
                }
                self.update_request = None;
            }
        }
    }

    fn get_title(&mut self) -> Option<String> {
        Some("Change Access Levels".to_owned())
    }

    fn show_content(&mut self, _app: &CalendarApp, ui: &mut egui::Ui, info: &mut ContentInfo) {
        let access_levels_count = self.access_levels.len();

        let mut move_up = None;
        let mut move_down = None;
        let mut add_after = None;
        let mut delete_at = None;

        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                let mut move_height = 0.;
                ui.vertical(|ui| {
                    self.access_levels.iter().enumerate().for_each(|(i, _)| {
                        move_height = ui
                            .vertical(|ui| {
                                //ui.style_mut().spacing.item_spacing.y = 0.;
                                if ui.add_enabled(i > 1, Button::new("▲").small()).clicked() {
                                    move_up = Some(i);
                                }
                                if ui
                                    .add_enabled(
                                        i > 0 && i + 1 < access_levels_count,
                                        Button::new("▼").small(),
                                    )
                                    .clicked()
                                {
                                    move_down = Some(i);
                                }
                            })
                            .response
                            .rect
                            .height();
                    });
                });

                let response = ui
                    .vertical(|ui| {
                        self.access_levels.iter_mut().for_each(|al| {
                            ui.allocate_ui_with_layout(
                                Vec2::new(160., move_height),
                                Layout::left_to_right(Align::Center),
                                |ui| {
                                    ui.add_enabled(
                                        !al.deleted,
                                        TextEdit::singleline(&mut al.name)
                                            .font(TextStyle::Heading)
                                            .char_limit(40)
                                            .desired_width(160.),
                                    );
                                },
                            );
                        });
                    })
                    .response;
                let height_per_item = response.rect.height() / self.access_levels.len() as f32;

                ui.vertical(|ui| {
                    ui.add_space(height_per_item / 2.);
                    self.access_levels.iter().enumerate().for_each(|(i, _)| {
                        ui.allocate_ui_with_layout(
                            Vec2::new(0., height_per_item),
                            Layout::left_to_right(Align::Center),
                            |ui| {
                                if ui
                                    .add_enabled(access_levels_count < 5, Button::new("Add"))
                                    .clicked()
                                {
                                    add_after = Some(i);
                                }
                            },
                        );
                    });
                });

                ui.vertical(|ui| {
                    self.access_levels.iter().enumerate().for_each(|(i, al)| {
                        ui.allocate_ui_with_layout(
                            Vec2::new(0., height_per_item),
                            Layout::left_to_right(Align::Center),
                            |ui| {
                                let text = if al.deleted { "Restore" } else { "Delete" };
                                if ui.add_enabled(i > 0, Button::new(text)).clicked() {
                                    delete_at = Some(i);
                                }
                            },
                        );
                    });
                });
            });
        });

        info.error(
            self.access_levels.iter().any(|al| al.name.is_empty()),
            "Name cannot be empty",
        );

        if let Some(move_up) = move_up {
            self.access_levels.swap(move_up - 1, move_up);
        }
        if let Some(move_down) = move_down {
            self.access_levels.swap(move_down, move_down + 1);
        }
        if let Some(add_after) = add_after {
            self.access_levels.insert(
                add_after + 1,
                AccessLevelData::new(AccessLevel {
                    id: -1,
                    user_id: self.user_id,
                    level: 0,
                    name: "".to_owned(),
                }),
            );
        }
        if let Some(delete_at) = delete_at {
            if self.access_levels[delete_at].origin.id == -1 {
                self.access_levels.remove(delete_at);
            } else {
                self.access_levels[delete_at].deleted = true;
            }
        }
    }

    fn show_buttons(&mut self, app: &CalendarApp, ui: &mut egui::Ui, info: &mut ContentInfo) {
        if ui
            .add_enabled(self.update_request.is_none(), Button::new("Save"))
            .clicked()
        {
            self.update_request = Some(
                app.state.get_user_state(self.user_id).change_access_levels(
                    self.access_levels
                        .iter()
                        .filter(|al| !al.deleted)
                        .cloned()
                        .enumerate()
                        .map(|(i, al)| AccessLevelChange {
                            id: al.origin.id,
                            old_level: al.origin.level,
                            new_level: AccessLevel::MAX_LEVEL - i as i32,
                            name: al.name,
                        })
                        .collect(),
                ),
            );
        }
        if ui.button("Cancel").clicked() {
            info.close();
        }
    }
}
