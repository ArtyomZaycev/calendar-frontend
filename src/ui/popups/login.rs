use calendar_lib::api::auth::login;

use super::popup_content::PopupContent;
use crate::{
    state::{custom_requests::LoginRequest, request::RequestIdentifier, State},
    utils::{is_password_valid, is_valid_email},
};

pub struct Login {
    pub email: String,
    pub password: String,
    email_not_found: Option<String>,
    password_not_found: Option<String>,

    request: Option<RequestIdentifier<LoginRequest>>,
}

impl Login {
    pub fn new() -> Self {
        Self {
            email: String::default(),
            password: String::default(),
            email_not_found: None,
            password_not_found: None,
            request: None,
        }
    }

    pub fn user_not_found(&mut self) {
        self.email_not_found = Some(self.email.clone());
        self.password_not_found = Some(self.password.clone());
    }
}

impl PopupContent for Login {
    fn init_frame(&mut self, state: &State, info: &mut super::popup_content::ContentInfo) {
        if let Some(identifier) = self.request.as_ref() {
            if let Some(response_info) = state.get_response(identifier) {
                self.request = None;
                match response_info {
                    Ok(_) => info.close(),
                    Err(error_info) => match &*error_info {
                        login::BadRequestResponse::UserNotFound => self.user_not_found(),
                    },
                }
            }
        }
    }

    fn get_title(&mut self) -> Option<String> {
        Some("Login".to_owned())
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
            show_input_field(ui, &mut self.email, "Email", false);
            show_input_field(ui, &mut self.password, "Password", true);

            info.error(
                &self.email != "admin" && !is_valid_email(&self.email),
                "Email is not valid",
            );
            info.error(!is_password_valid(&self.password), "Password is too long");
            info.error(
                self.email_not_found
                    .as_ref()
                    .map_or(false, |e| e == &self.email)
                    && self
                        .password_not_found
                        .as_ref()
                        .map_or(false, |e| e == &self.password),
                "Unknown login",
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
            .add_enabled(!info.is_error(), egui::Button::new("Login"))
            .clicked()
        {
            self.request = Some(state.login(self.email.clone(), self.password.clone()));
        }
        if ui.button("Cancel").clicked() {
            info.close();
        }
    }
}
