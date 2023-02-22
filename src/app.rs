use calendar_lib::api::{auth::register, events::types::Event};
use chrono::NaiveDate;
use egui::{Align, Layout, RichText, Sense};
use num_traits::FromPrimitive;
use serde::{Deserialize, Serialize};

use crate::{
    config::Config,
    db::{
        state::State,
        state_action::{GetStateAction, HasStateAction, StateAction},
    },
    ui::{
        event_card::EventCard,
        popups::{
            event_input::EventInput,
            login::Login,
            popup::{Popup, PopupType},
            profile::Profile,
            sign_up::SignUp,
        },
        widget_builder::WidgetBuilder,
        widget_signal::AppSignal,
    },
    utils::{get_first_month_day_date, get_last_month_day_date, get_monday, weekday_human_name},
};

#[derive(Debug, Clone, Deserialize, Serialize)]
enum CalendarView {
    Month(NaiveDate),
    Week(NaiveDate),
    Day(NaiveDate),
    Events(NaiveDate),
}

#[derive(Deserialize, Serialize)]
#[serde(default)]
pub struct CalendarApp {
    #[serde(skip)]
    state: State,

    view: CalendarView,

    #[serde(skip)]
    popups: Vec<Popup>,
}

impl Default for CalendarApp {
    fn default() -> Self {
        let config = Config::load();
        Self {
            state: State::new(&config),
            view: CalendarView::Events(chrono::Local::now().naive_local().date()),
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

    pub fn is_open_profile(&self) -> bool {
        self.popups.iter().any(|p| p.get_type().is_profile())
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

    pub fn open_profile(&mut self) {
        self.popups.push(
            PopupType::Profile(Profile::new(self.state.me.as_ref().unwrap().clone())).popup(),
        );
    }
    pub fn open_login(&mut self) {
        self.popups.push(PopupType::Login(Login::new()).popup());
    }
    pub fn open_sign_up(&mut self) {
        self.popups.push(PopupType::SignUp(SignUp::new()).popup());
    }
    pub fn open_new_event(&mut self) {
        self.popups.push(
            PopupType::NewEvent(EventInput::new(
                self.state.me.as_ref().unwrap().get_access_level().level,
            ))
            .popup(),
        );
    }
    pub fn open_change_event(&mut self, event: &Event) {
        self.popups.push(
            PopupType::UpdateEvent(EventInput::change(
                self.state.me.as_ref().unwrap().get_access_level().level,
                event,
            ))
            .popup(),
        );
    }
}

impl CalendarApp {
    fn parse_signal(&mut self, signal: AppSignal) {
        match signal {
            AppSignal::StateSignal(signal) => self.state.parse_signal(signal),
            AppSignal::ChangeEvent(event_id) => {
                if let Some(event) = self.state.events.iter().find(|event| event.id == event_id) {
                    self.open_change_event(&event.clone());
                }
            }
        }
    }

    fn parse_signals(&mut self, signals: Vec<AppSignal>) {
        signals
            .into_iter()
            .for_each(|signal| self.parse_signal(signal));
    }

    fn parse_polled(&mut self, polled: Vec<StateAction>) {
        if let Some(popup) = self.get_login_popup() {
            if polled.has_login() {
                popup.closed = true;
            }
        }
        if let Some(popup) = self.get_sign_up_popup() {
            if polled.has_register() {
                popup.closed = true;
            } else if let Some(error) = polled.get_register_error() {
                match error {
                    register::BadRequestResponse::EmailAlreadyUsed => {
                        popup.email_taken = true;
                    }
                }
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
    }
}

impl CalendarApp {
    fn top_panel(&mut self, ui: &mut egui::Ui) {
        ui.with_layout(Layout::left_to_right(Align::TOP), |ui| {
            ui.heading("Calendar");

            ui.with_layout(Layout::right_to_left(Align::TOP), |ui| {
                // RTL
                if let Some(me) = &self.state.me {
                    let profile = egui::Label::new(&me.user.name);
                    if self.is_open_profile() {
                        ui.add(profile);
                    } else {
                        if ui.add(profile.sense(Sense::click())).clicked() {
                            self.open_profile();
                        }
                    }
                    if ui.button("Logout").clicked() {
                        self.state.logout();
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
    }

    fn month_view(&mut self, ui: &mut egui::Ui, date: NaiveDate) {
        let first_day = get_first_month_day_date(&date);
        let last_day = get_last_month_day_date(&date);
        let first_monday = get_monday(&first_day);
        let column_width = (ui.available_width() - ui.spacing().item_spacing.x * 6.) / 7.;

        let mut signals = vec![];
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
                            let events = self
                                .state
                                .events
                                .iter()
                                .filter(|e| e.start.date() == date)
                                .count();
                            ui.vertical(|ui| {
                                ui.label(date.to_string());
                                if first_day <= date && date <= last_day {
                                    ui.label(events.to_string());
                                } else {
                                    ui.add_space(0.);
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
            let column_width = (ui.available_width() - ui.spacing().item_spacing.x * 6.) / 7.;
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
                        self.state
                            .events
                            .iter()
                            .filter(|e| e.start.date() == date)
                            .for_each(|event| {
                                ui.add(
                                    EventCard::new(
                                        &mut signals,
                                        egui::Vec2::new(column_width, 200.),
                                        &event,
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
            let num_columns = 7usize;
            let column_width = (ui.available_width()
                - ui.spacing().item_spacing.x * (num_columns - 1) as f32)
                / num_columns as f32;

            let mut signals = vec![];

            // TODO: Use array_chunks, once it becomes stable
            // https://github.com/rust-lang/rust/issues/100450
            self.state
                .events
                .iter()
                .filter(|e| e.start.date() == date)
                .enumerate()
                .fold(Vec::default(), |mut acc, (i, event)| {
                    if i % num_columns == 0 {
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
                            ));
                        });
                    });
                });

            self.parse_signals(signals);
        });
    }

    fn events_view(&mut self, ui: &mut egui::Ui, date: NaiveDate) {
        egui::ScrollArea::vertical().show(ui, |ui| {
            let num_columns = 7usize;
            let column_width = (ui.available_width()
                - ui.spacing().item_spacing.x * (num_columns - 1) as f32)
                / num_columns as f32;

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
                        // TODO: Use array_chunks, once it becomes stable
                        // https://github.com/rust-lang/rust/issues/100450
                        self.state
                            .events
                            .iter()
                            .filter(|e| e.start.date() == date)
                            .enumerate()
                            .fold(Vec::default(), |mut acc, (i, event)| {
                                if i % num_columns == 0 {
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
                                        ));
                                    });
                                });
                            });
                    });
            });

            self.parse_signals(signals);
        });
    }
}

impl eframe::App for CalendarApp {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
        let polled = self.state.poll();

        egui::CentralPanel::default().show(ctx, |ui| {
            let signals = self
                .popups
                .iter_mut()
                .flat_map(|popup| {
                    ui.add(popup.build(ctx));
                    popup.signals()
                })
                .collect::<Vec<_>>();
            self.parse_signals(signals);

            self.popups
                .iter_mut()
                .enumerate()
                .filter_map(|(i, popup)| popup.is_closed().then_some(i))
                .collect::<Vec<_>>()
                .iter()
                .rev()
                .for_each(|&i| {
                    self.popups.swap_remove(i);
                });

            self.top_panel(ui);
            ui.separator();

            self.parse_polled(polled);

            // CALENDAR
            if let Some(_me) = &self.state.me {
                ui.with_layout(Layout::top_down_justified(Align::LEFT), |ui| {
                    ui.with_layout(Layout::left_to_right(Align::TOP), |ui| {
                        let mut month_text = RichText::new("Month").heading();
                        let mut week_text = RichText::new("Week").heading();
                        let mut day_text = RichText::new("Day").heading();
                        let mut events_text = RichText::new("Events").heading();
                        match self.view {
                            // This is pathetic
                            CalendarView::Month(_) => month_text = month_text.underline(),
                            CalendarView::Week(_) => week_text = week_text.underline(),
                            CalendarView::Day(_) => day_text = day_text.underline(),
                            CalendarView::Events(_) => events_text = events_text.underline(),
                        };
                        if ui
                            .add(egui::Label::new(month_text).sense(Sense::click()))
                            .clicked()
                        {
                            self.view =
                                CalendarView::Month(chrono::Local::now().naive_local().date());
                        }
                        if ui
                            .add(egui::Label::new(week_text).sense(Sense::click()))
                            .clicked()
                        {
                            self.view =
                                CalendarView::Week(chrono::Local::now().naive_local().date());
                        }
                        if ui
                            .add(egui::Label::new(day_text).sense(Sense::click()))
                            .clicked()
                        {
                            self.view =
                                CalendarView::Day(chrono::Local::now().naive_local().date());
                        }
                        if ui
                            .add(egui::Label::new(events_text).sense(Sense::click()))
                            .clicked()
                        {
                            self.view =
                                CalendarView::Events(chrono::Local::now().naive_local().date());
                        }
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
                    ui.add_space(8.);

                    match self.view {
                        CalendarView::Month(date) => self.month_view(ui, date),
                        CalendarView::Week(date) => self.week_view(ui, date),
                        CalendarView::Day(date) => self.day_view(ui, date),
                        CalendarView::Events(date) => self.events_view(ui, date),
                    }
                });
            }
        });
    }
}
