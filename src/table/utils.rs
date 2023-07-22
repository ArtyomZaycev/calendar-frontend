use reqwest::StatusCode;
use serde::de::DeserializeOwned;

use crate::{
    db::request_parser::RequestParser,
    requests::*
};

/// Use for testing only
#[cfg(debug_assertions)]
#[allow(dead_code)]
pub(super) fn make_empty_parser<RequestResponse: Default>() -> RequestParser<AppRequestResponse> {
    RequestParser::new_split(
        |_| AppRequestResponse::None,
        |_, _| AppRequestResponse::None,
    )
}

pub(super) fn make_parser<U, F>(on_success: F) -> RequestParser<AppRequestResponse>
where
    U: DeserializeOwned,
    F: FnOnce(U) -> AppRequestResponse + 'static,
{
    RequestParser::new_complex(on_success, |code, s| AppRequestResponse::Error(code, s))
}

#[allow(dead_code)]
pub(super) fn make_bad_request_parser<T, F1, F2>(
    on_success: F1,
    on_bad_request: F2,
) -> RequestParser<AppRequestResponse>
where
    T: DeserializeOwned,
    F1: FnOnce(T) -> AppRequestResponse + 'static,
    F2: FnOnce(String) -> AppRequestResponse + 'static,
{
    RequestParser::new_complex(on_success, |code, msg| {
        if code == StatusCode::BAD_REQUEST {
            on_bad_request(msg)
        } else {
            AppRequestResponse::Error(code, msg)
        }
    })
}

pub(super) fn make_typed_bad_request_parser<T, U, F1, F2>(
    on_success: F1,
    on_bad_request: F2,
) -> RequestParser<AppRequestResponse>
where
    T: DeserializeOwned,
    U: DeserializeOwned,
    F1: FnOnce(T) -> AppRequestResponse + 'static,
    F2: FnOnce(U) -> AppRequestResponse + 'static,
{
    RequestParser::new_complex(on_success, |code, msg| {
        if code == StatusCode::BAD_REQUEST {
            on_bad_request(serde_json::from_str(&msg).unwrap())
        } else {
            AppRequestResponse::Error(code, msg)
        }
    })
}
