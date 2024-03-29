use calendar_lib::api::auth::register;

use super::popup_content::PopupContent;
use crate::{
    db::request::{RequestDescription, RequestId},
    requests::AppRequestResponseInfo,
    state::State,
    ui::signal::RequestSignal,
    utils::{is_password_strong_enough, is_valid_email},
};

pub struct SignUp {
    pub name: String,
    pub email: String,
    pub password: String,
    pub password2: String,
    email_taken: Option<String>,

    request_id: Option<RequestId>,
}

impl SignUp {
    pub fn new() -> Self {
        Self {
            name: String::default(),
            email: String::default(),
            password: String::default(),
            password2: String::default(),
            email_taken: None,
            request_id: None,
        }
    }

    pub fn email_taken(&mut self) {
        self.email_taken = Some(self.email.clone());
    }
}

impl PopupContent for SignUp {
    fn init_frame(&mut self, state: &State, info: &mut super::popup_content::ContentInfo) {
        if let Some(request_id) = self.request_id {
            if let Some(response_info) = state.connector.get_response_info(request_id) {
                self.request_id = None;
                if let AppRequestResponseInfo::RegisterError(error_info) = response_info {
                    match error_info {
                        register::BadRequestResponse::EmailAlreadyUsed => self.email_taken(),
                    }
                } else if !response_info.is_error() {
                    info.close();
                }
            }
        }
    }

    fn get_title(&mut self) -> Option<String> {
        Some("Sign Up".to_owned())
    }

    fn show_content(
        &mut self,
        _state: &State,
        ui: &mut egui::Ui,
        info: &mut super::popup_content::ContentInfo,
    ) {
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

            info.error(self.name.len() < 6, "Name must be at least 6 symbols");
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

    fn show_buttons(
        &mut self,
        state: &State,
        ui: &mut egui::Ui,
        info: &mut super::popup_content::ContentInfo,
    ) {
        if ui
            .add_enabled(!info.is_error(), egui::Button::new("Sign Up"))
            .clicked()
        {
            let request_id = state.connector.reserve_request_id();
            self.request_id = Some(request_id);
            info.signal(
                RequestSignal::Register(
                    self.name.clone(),
                    self.email.clone(),
                    self.password.clone(),
                )
                .with_description(RequestDescription::new().with_request_id(request_id)),
            );
        }
        if ui.button("Cancel").clicked() {
            info.close();
        }
    }
}
