use reqwest::{Request, Response};
use serde::de::DeserializeOwned;
use serde_json::Value;

pub type OnSuccessJson<T> = Box<dyn FnOnce(Value) -> T + Send>;
pub type OnSuccess<T, U> = Box<dyn FnOnce(U) -> T + Send>;
pub type OnError<T> = Box<dyn FnOnce(Response) -> T + Send>;

pub struct AppRequest<T> {
    pub request: Request,
    pub on_success: Option<OnSuccessJson<T>>,
    pub on_error: Option<OnError<T>>,
}

impl<T: 'static> AppRequest<T> {
    pub fn new_json(
        request: Request,
        on_success: impl Into<Option<OnSuccessJson<T>>>,
        on_error: impl Into<Option<OnError<T>>>,
    ) -> Self {
        Self {
            request,
            on_success: on_success.into(),
            on_error: on_error.into(),
        }
    }

    pub fn new<O>(
        request: Request,
        on_success: impl Into<Option<OnSuccess<T, O>>>,
        on_error: impl Into<Option<OnError<T>>>,
    ) -> Self
    where
        O: DeserializeOwned + 'static,
    {
        Self {
            request,
            on_success: on_success.into().map::<OnSuccessJson<T>, _>(|on_success| {
                Box::new(|j| on_success(serde_json::from_value(j).unwrap()))
            }),
            on_error: on_error.into(),
        }
    }

    pub fn new_ignore(request: Request, on_error: impl Into<Option<OnError<T>>>) -> Self {
        Self {
            request,
            on_success: None,
            on_error: on_error.into(),
        }
    }
}
