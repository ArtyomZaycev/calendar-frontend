use calendar_lib::api::auth::register;

use super::{
    popup::PopupType,
    popup_content::{ContentInfo, PopupContent},
};
use crate::{
    app::CalendarApp,
    db::request::RequestIdentifier,
    state::custom_requests::RegisterRequest,
    utils::{is_password_strong_enough, is_valid_email},
};

pub struct SignUp {
    pub name: String,
    pub email: String,
    pub password: String,
    pub password2: String,
    email_taken: Option<String>,

    request: Option<RequestIdentifier<RegisterRequest>>,
}

impl SignUp {
    pub fn new() -> Self {
        Self {
            name: String::default(),
            email: String::default(),
            password: String::default(),
            password2: String::default(),
            email_taken: None,
            request: None,
        }
    }

    pub fn email_taken(&mut self) {
        self.email_taken = Some(self.email.clone());
    }
}

impl PopupContent for SignUp {
    fn get_type(&self) -> PopupType {
        PopupType::SignUp
    }

    fn init_frame(&mut self, app: &CalendarApp, info: &mut ContentInfo) {
        if let Some(identifier) = self.request.as_ref() {
            if let Some(response_info) = app.state.get_response(identifier) {
                self.request = None;
                match response_info {
                    Ok(_) => info.close(),
                    Err(error_info) => match &*error_info {
                        register::BadRequestResponse::EmailAlreadyUsed => self.email_taken(),
                    },
                }
            }
        }
    }

    fn get_title(&mut self) -> Option<String> {
        Some("Sign Up".to_owned())
    }

    fn show_content(&mut self, _app: &CalendarApp, ui: &mut egui::Ui, info: &mut ContentInfo) {
        let show_input_field =
            |ui: &mut egui::Ui, value: &mut String, hint: &str, password: bool| {
                ui.add(
                    egui::TextEdit::singleline(value)
                        .desired_width(f32::INFINITY)
                        .hint_text(hint)
                        .password(password),
                );
            };

        ui.vertical_centered(|ui| {
            show_input_field(ui, &mut self.name, "Name", false);
            show_input_field(ui, &mut self.email, "Email", false);
            show_input_field(ui, &mut self.password, "Password", true);
            show_input_field(ui, &mut self.password2, "Confirm Password", true);

            info.error(self.name.is_empty(), "Name cannot be empty");
            info.error(self.name.len() > 30, "Name must be at most 30 symbols");
            info.error(!is_valid_email(&self.email), "Email is not valid");
            info.error(
                self.email_taken
                    .as_ref()
                    .map_or(false, |e| e == &self.email),
                "Account with this email is already registered",
            );
            info.error(
                !is_password_strong_enough(&self.password),
                "Password is not strong enough",
            );
            info.error(
                self.password != self.password2,
                "Passwords must be the same",
            );
        });
    }

    fn show_buttons(&mut self, app: &CalendarApp, ui: &mut egui::Ui, info: &mut ContentInfo) {
        if ui
            .add_enabled(!info.is_error(), egui::Button::new("Sign Up"))
            .clicked()
        {
            self.request = Some(app.state.register(
                self.name.clone(),
                self.email.clone(),
                self.password.clone(),
            ));
        }
        if ui.button("Cancel").clicked() {
            info.close();
        }
    }
}
