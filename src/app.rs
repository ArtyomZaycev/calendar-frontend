use egui::{Button, TextEdit};
use reqwest::Method;
use std::sync::{Arc, Mutex};

use crate::db::{
    aliases::EchoStruct,
    request::{self, AppRequest},
    state::State,
};

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct CalendarApp {
    #[serde(skip)]
    state: State,

    echo_input: String,

    #[serde(skip)]
    echo_recieved: Arc<Mutex<Option<String>>>,

    login_info: Option<(String, String)>,
}

impl Default for CalendarApp {
    fn default() -> Self {
        Self {
            state: State::new(),
            echo_input: "Hello API!".into(),
            echo_recieved: Arc::default(),
            login_info: Some((String::default(), String::default())),
        }
    }
}

impl CalendarApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }

    // Bad coding style, redo
    fn make_login<'a>(
        &'a mut self,
    ) -> Option<(egui::TextEdit<'a>, egui::TextEdit<'a>, egui::Button)> {
        self.login_info.as_mut().map(|(login, password)| {
            (
                TextEdit::singleline(login),
                TextEdit::singleline(password),
                Button::new("Login"),
            )
        })
    }
}

impl eframe::App for CalendarApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.state.poll();

        #[cfg(not(target_arch = "wasm32"))] // no File->Quit on web pages!
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        _frame.close();
                    }
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            if let Some((login_edit, pass_edit, login_button)) = self.make_login() {
                ui.add(login_edit);
                ui.add(pass_edit);
                if ui.add(login_button).clicked() {
                    self.state.login(
                        &self.login_info.as_ref().unwrap().0,
                        &self.login_info.as_ref().unwrap().1,
                    );
                }
            }

            if let Some(me) = &self.state.me {
                ui.label(format!("{:?}", me.user.key));
                ui.label(format!("{:?}", me.roles));
                if ui.button("Load roles").clicked() {
                    self.state.load_user_roles();
                }
            }

            ui.text_edit_singleline(&mut self.echo_input);

            /*if ui.button("Echo").clicked() {
                let echo_recieved = self.echo_recieved.clone();
                let on_success : request::OnSuccess<EchoStruct> = Box::new(move |s| {
                    let mut rec = echo_recieved.lock().unwrap();
                    *rec = Some(s.echo.clone());
                    println!("request::OnSuccess {:?}", s);
                });
                let on_error : request::OnError = Box::new(|e| {
                    println!("request::OnError {:?}", e);
                });
                self.state.connector.request(AppRequest::new(
                    self.state.connector.client.request(Method::POST, "http://127.0.0.1:8080/echo_struct")
                        .body(self.echo_input.clone())
                        .build().unwrap(),
                    on_success,
                    on_error
                ))
            }*/

            // The central panel the region left after adding TopPanel's and SidePanel's

            let rec = if let Ok(val) = self.echo_recieved.try_lock() {
                val.clone().unwrap_or_default()
            } else {
                String::default()
            };
            ui.label(rec);
            ui.heading("eframe template");
            ui.hyperlink("https://github.com/emilk/eframe_template");
            ui.add(egui::github_link_file!(
                "https://github.com/emilk/eframe_template/blob/master/",
                "Source code."
            ));
            egui::warn_if_debug_build(ui);
        });

        if false {
            egui::Window::new("Window").show(ctx, |ui| {
                ui.label("Windows can be moved by dragging them.");
                ui.label("They are automatically sized based on contents.");
                ui.label("You can turn on resizing and scrolling if you like.");
                ui.label("You would normally choose either panels OR windows.");
            });
        }
    }
}
