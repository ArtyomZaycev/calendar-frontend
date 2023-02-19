use calendar_lib::api::events::types::Event;
use egui::{Align, Layout};
use serde::{Deserialize, Serialize};

use crate::{
    config::Config,
    db::{state::State, state_action::HasStateAction},
    ui::{
        event_card::EventCard,
        popups::{
            event_input::EventInput,
            login::Login,
            popup::{Popup, PopupType},
            sign_up::SignUp,
        },
        widget_builder::WidgetBuilder,
    },
};

#[derive(Deserialize, Serialize)]
#[serde(default)]
pub struct CalendarApp {
    #[serde(skip)]
    pub state: State,

    #[serde(skip)]
    popups: Vec<Popup>,
}

impl Default for CalendarApp {
    fn default() -> Self {
        let config = Config::load();
        Self {
            state: State::new(&config),
            popups: Vec::default(),
        }
    }
}

impl CalendarApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl CalendarApp {
    pub fn get_login_popup<'a>(&'a mut self) -> Option<&'a mut Login> {
        self.popups.iter_mut().find_map(|p| {
            if let PopupType::Login(v) = p.get_type_mut() {
                Some(v)
            } else {
                None
            }
        })
    }
    pub fn get_sign_up_popup<'a>(&'a mut self) -> Option<&'a mut SignUp> {
        self.popups.iter_mut().find_map(|p| {
            if let PopupType::SignUp(v) = p.get_type_mut() {
                Some(v)
            } else {
                None
            }
        })
    }
    pub fn get_new_event_popup<'a>(&'a mut self) -> Option<&'a mut EventInput> {
        self.popups.iter_mut().find_map(|p| {
            if let PopupType::NewEvent(v) = p.get_type_mut() {
                Some(v)
            } else {
                None
            }
        })
    }
    pub fn get_update_event_popup<'a>(&'a mut self) -> Option<&'a mut EventInput> {
        self.popups.iter_mut().find_map(|p| {
            if let PopupType::UpdateEvent(v) = p.get_type_mut() {
                Some(v)
            } else {
                None
            }
        })
    }

    pub fn is_open_login(&self) -> bool {
        self.popups.iter().any(|p| p.get_type().is_login())
    }

    pub fn is_open_sign_up(&self) -> bool {
        self.popups.iter().any(|p| p.get_type().is_sign_up())
    }

    pub fn is_open_new_event(&self) -> bool {
        self.popups.iter().any(|p| p.get_type().is_new_event())
    }

    pub fn open_login(&mut self) {
        self.popups.push(PopupType::Login(Login::new()).popup());
    }
    pub fn open_sign_up(&mut self) {
        self.popups.push(PopupType::SignUp(SignUp::new()).popup());
    }
    pub fn open_new_event(&mut self) {
        self.popups
            .push(PopupType::NewEvent(EventInput::new()).popup());
    }
    pub fn open_change_event(&mut self, event: &Event) {
        self.popups
            .push(PopupType::UpdateEvent(EventInput::change(event)).popup());
    }
}

impl eframe::App for CalendarApp {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
        let polled = self.state.poll();

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.spacing_mut().item_spacing = egui::vec2(16.0, 8.0);

            self.popups
                .iter_mut()
                .enumerate()
                .filter_map(|(i, popup)| (!popup.show(&mut self.state, ctx, ui)).then_some(i))
                .collect::<Vec<_>>()
                .iter()
                .rev()
                .for_each(|&i| {
                    self.popups.swap_remove(i);
                });

            // TOP PANEL
            ui.with_layout(Layout::left_to_right(Align::TOP), |ui| {
                ui.heading("Calendar");

                ui.with_layout(Layout::right_to_left(Align::TOP), |ui| {
                    // RTL, so add widgets in the reversed order

                    if let Some(me) = &self.state.me {
                        ui.label(me.user.id.to_string());
                        if ui.button("Logout").clicked() {
                            println!("Not implemented");
                        }
                    } else {
                        if ui
                            .add_enabled(!self.is_open_login(), egui::Button::new("Login"))
                            .clicked()
                        {
                            self.open_login();
                        }
                        if ui
                            .add_enabled(!self.is_open_sign_up(), egui::Button::new("Sign Up"))
                            .clicked()
                        {
                            self.open_sign_up();
                        }
                    }

                    if self.state.get_active_requests_descriptions().len() > 0 {
                        // TODO: icon
                        ui.label("xxx");
                    }
                });
            });
            ui.separator();

            if let Some(popup) = self.get_login_popup() {
                if polled.has_login() {
                    popup.closed = true;
                }
            }
            if let Some(popup) = self.get_sign_up_popup() {
                if polled.has_register() {
                    popup.closed = true;
                }
            }
            if let Some(popup) = self.get_new_event_popup() {
                if polled.has_insert_event() {
                    popup.closed = true;
                }
            }
            if let Some(popup) = self.get_update_event_popup() {
                if polled.has_update_event() {
                    popup.closed = true;
                }
            }

            // CALENDAR
            if let Some(_me) = &self.state.me {
                ui.with_layout(Layout::top_down_justified(Align::LEFT), |ui| {
                    ui.with_layout(Layout::left_to_right(Align::TOP), |ui| {
                        ui.heading("Events");
                        ui.with_layout(Layout::right_to_left(Align::TOP), |ui| {
                            if ui
                                .add_enabled(
                                    !self.is_open_new_event(),
                                    egui::Button::new("Add Event"),
                                )
                                .clicked()
                            {
                                self.open_new_event();
                            }
                        });
                    });

                    egui::ScrollArea::vertical().show(ui, |ui| {
                        // TODO: Remove clone somehow
                        let events = self.state.events.clone();
                        // TODO: Use array_chunks, once it becomes stable
                        // https://github.com/rust-lang/rust/issues/100450
                        events
                            .into_iter()
                            .enumerate()
                            .fold(Vec::default(), |mut acc, (i, event)| {
                                if i % 7 == 0 {
                                    acc.push(Vec::default());
                                }
                                acc.last_mut().unwrap().push(event);
                                acc
                            })
                            .into_iter()
                            .for_each(|events| {
                                ui.with_layout(Layout::left_to_right(Align::TOP), |ui| {
                                    events.into_iter().for_each(|event| {
                                        ui.add(EventCard::new(self, &event));
                                    });
                                });
                            });
                    });
                });
            }
        });
    }
}
