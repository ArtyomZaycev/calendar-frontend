use std::path::Path;

use egui::{Align, Layout};

use crate::{
    db::state::State,
    ui::{
        event_card::EventCard,
        popups::{
            event_input::EventInput,
            login::Login,
            popup::{Popup, PopupType},
            sign_up::SignUp,
        },
        widget_builder::WidgetBuilder,
    }, config::Config,
};

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct CalendarApp {
    #[serde(skip)]
    state: State,

    #[serde(skip)]
    popups: Vec<Popup>,
}

impl Default for CalendarApp {
    fn default() -> Self {
        let config = Config::load(&Path::new("./config.json"));
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
    pub fn is_open_login(&self) -> bool {
        self.popups.iter().any(|p| p.get_type().is_login())
    }

    pub fn is_open_sign_up(&self) -> bool {
        self.popups.iter().any(|p| p.get_type().is_sign_up())
    }

    pub fn is_open_new_event(&self) -> bool {
        self.popups.iter().any(|p| p.get_type().is_new_event())
    }
}

impl eframe::App for CalendarApp {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
        self.state.poll();

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
                            self.popups.push(PopupType::Login(Login::new()).popup());
                        }
                        if ui
                            .add_enabled(!self.is_open_sign_up(), egui::Button::new("Sign Up"))
                            .clicked()
                        {
                            self.popups.push(PopupType::SignUp(SignUp::new()).popup());
                        }
                    }
                });
            });
            ui.separator();

            // CALENDAR
            if let Some(_) = &self.state.me {
                ui.with_layout(Layout::top_down_justified(Align::LEFT), |ui| {
                    ui.with_layout(Layout::left_to_right(Align::TOP), |ui| {
                        ui.heading("Events");
                        ui.with_layout(Layout::right_to_left(Align::TOP), |ui| {
                            if ui
                                .add_enabled(
                                    !self.is_open_new_event(),
                                    egui::Button::new("New Event"),
                                )
                                .clicked()
                            {
                                self.popups
                                    .push(PopupType::NewEvent(EventInput::new()).popup());
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
                                        ui.add(EventCard::new(&mut self.state, &event));
                                    });
                                });
                            });
                    });
                });
            }
        });
    }
}
