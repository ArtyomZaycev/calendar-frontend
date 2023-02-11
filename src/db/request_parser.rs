use bytes::Bytes;
use reqwest::StatusCode;
use serde::de::DeserializeOwned;

pub struct RequestParser<T> {
    parser: Box<dyn FnOnce(StatusCode, Bytes) -> T>
}

impl<T> RequestParser<T> {
    pub fn new<F>(parser: F) -> Self where
        F: FnOnce(StatusCode, Bytes) -> T + 'static,
    {
        Self {
            parser: Box::new(parser),
        }
    }

    pub fn new_split<F1, F2>(on_success: F1, on_error: F2) -> Self where
        F1: FnOnce(Bytes) -> T + 'static,
        F2: FnOnce(StatusCode, Bytes) -> T + 'static,
    {
        Self::new(|status_code, bytes| {
            if status_code == StatusCode::OK {
                on_success(bytes)
            } else {
                on_error(status_code, bytes)
            }
        })
    }

    pub fn new_complex<U, F1, F2>(on_success: F1, on_error: F2) -> Self where
        U: DeserializeOwned,
        F1: FnOnce(U) -> T + 'static,
        F2: FnOnce(StatusCode, String) -> T + 'static,
    {
        Self::new_split(
            |bytes| {
                on_success(serde_json::from_slice(&bytes).unwrap())
            },
            |status_code, bytes| {
                on_error(status_code, String::from_utf8_lossy(&bytes).to_string())
            }
        )
    }

    pub fn parse(self, status_code: StatusCode, bytes: Bytes) -> T {
        (self.parser)(status_code, bytes)
    }
}