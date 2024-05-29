use calendar_lib::api::auth::login;

use super::popup_content::{ContentInfo, PopupContent};
use crate::{
    app::CalendarApp,
    db::request::RequestIdentifier,
    state::custom_requests::LoginRequest,
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
}

impl PopupContent for Login {
    fn init_frame(&mut self, app: &CalendarApp, info: &mut ContentInfo) {
        if let Some(identifier) = self.request.as_ref() {
            if let Some(response_info) = app.state.get_response(identifier) {
                match response_info {
                    Ok(_) => info.close(),
                    Err(error_info) => match &*error_info {
                        login::BadRequestResponse::UserNotFound => {
                            self.email_not_found = Some(identifier.info.email.clone());
                            self.password_not_found = Some(identifier.info.password.clone());
                        }
                    },
                }
                self.request = None;
            }
        }
    }

    fn get_title(&mut self) -> Option<String> {
        Some("Авторизация".to_owned())
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
            show_input_field(ui, &mut self.email, "E-mail", false);
            show_input_field(ui, &mut self.password, "Пароль", true);

            info.error(!is_valid_email(&self.email), "Некорректный формат E-mail");
            info.error(!is_password_valid(&self.password), "Password is too long");
            info.error(
                self.email_not_found
                    .as_ref()
                    .map_or(false, |e| e == &self.email)
                    && self
                        .password_not_found
                        .as_ref()
                        .map_or(false, |e| e == &self.password),
                "Неизвестный пользователь",
            );
        });
    }

    fn show_buttons(&mut self, app: &CalendarApp, ui: &mut egui::Ui, info: &mut ContentInfo) {
        if ui
            .add_enabled(!info.is_error(), egui::Button::new("Вход"))
            .clicked()
        {
            self.request = Some(app.state.login(self.email.clone(), self.password.clone()));
        }
        if ui.button("Отмена").clicked() {
            info.close();
        }
    }
}
