use calendar_lib::api::{
    auth::{types::NewPassword, *},
    user_state,
};

use crate::tables::TableId;

use super::{
    custom_requests::*,
    db_connector::DbConnectorData,
    main_state::{AdminState, RequestIdentifier, RequestType, State, UserState},
    requests_holder::RequestData,
};

impl State {
    pub fn logout(&mut self) -> RequestIdentifier<LogoutRequest> {
        // TODO: Clear user all data
        self.requests
            .make_typical_request((), |connector| connector.make_request::<LoginRequest>())
    }

    pub fn login(&self, email: String, password: String) -> RequestIdentifier<LoginRequest> {
        let request = self.requests.make_typical_request((), |connector| {
            connector
                .make_request::<LoginRequest>()
                .json(&login::Body { email, password })
        });
        self.request_parsers.borrow_mut().push(Box::new(move |connector| {
            let response = connector.take_response::<<LoginRequest as RequestType>::Response, <LoginRequest as RequestType>::BadResponse>(request.id);
            response.map(|response| {
                let func: Box<dyn FnOnce(&mut State)> = Box::new(move |state: &mut State| {
                    if let Ok(response) = response {
                        match response {
                            Ok(response) => LoginRequest::push_to_state(*response, request.info, state),
                            Err(response) => LoginRequest::push_bad_to_state(*response, request.info, state),
                        }
                    }
                });
                func
            })
        }));

        request
    }

    pub fn login_by_jwt(&self, key: String) -> RequestIdentifier<LoginByKeyRequest> {
        self.requests.make_typical_request((), |connector| {
            connector
                .make_request::<LoginByKeyRequest>()
                .bearer_auth(key)
        })
    }

    pub fn register(
        &self,
        name: String,
        email: String,
        password: String,
    ) -> RequestIdentifier<RegisterRequest> {
        self.requests.make_typical_request((), |connector| {
            connector
                .make_request::<RegisterRequest>()
                .json(&register::Body {
                    name,
                    email,
                    password,
                })
        })
    }
}

impl UserState {
    pub fn insert_password(
        &self,
        user_id: i32,
        access_level: i32,
        viewer_password: Option<NewPassword>,
        editor_password: Option<NewPassword>,
    ) -> RequestIdentifier<NewPasswordRequest> {
        self.requests.make_typical_request((), |connector| {
            connector
                .make_request::<NewPasswordRequest>()
                .json(&new_password::Body {
                    user_id,
                    access_level,
                    viewer_password,
                    editor_password,
                })
        })
    }

    pub fn load_state(&self) -> RequestIdentifier<LoadStateRequest> {
        self.requests.make_typical_request(None, |connector| {
            connector
                .make_request::<LoadStateRequest>()
                .query(&user_state::load::Args { user_id: None })
        })
    }
}

impl AdminState {
    pub fn load_state(&self, user_id: TableId) -> RequestIdentifier<LoadStateRequest> {
        self.requests
            .make_typical_request(Some(user_id), |connector| {
                connector
                    .make_request::<LoadStateRequest>()
                    .query(&user_state::load::Args {
                        user_id: Some(user_id),
                    })
            })
    }
}
