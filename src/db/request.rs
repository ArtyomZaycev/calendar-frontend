use reqwest::{Response, Request};
use serde::de::DeserializeOwned;
use serde_json::Value;


pub type OnSuccessJson = Box<dyn FnOnce(Value) + Send>;
pub type OnSuccess<T> = Box<dyn FnOnce(T) + Send>;
pub type OnError = Box<dyn FnOnce(Response) + Send>;

pub struct AppRequest {
    pub request: Request,
    pub on_success: Option<OnSuccessJson>,
    pub on_error: Option<OnError>,
}

impl AppRequest {
    pub fn new_json(request: Request, on_success: impl Into<Option<OnSuccessJson>>, on_error: impl Into<Option<OnError>>) -> Self
    {
        Self {
            request,
            on_success: on_success.into(),
            on_error: on_error.into(),
        }
    }

    pub fn new<O>(request: Request, on_success: impl Into<Option<OnSuccess<O>>>, on_error: impl Into<Option<OnError>>) -> Self where
        O: DeserializeOwned + 'static
    {
        Self {
            request,
            on_success: on_success.into().map::<OnSuccessJson, _>(|on_success| 
                Box::new(|j| 
                    on_success(serde_json::from_value(j).unwrap())
                )
            ),
            on_error: on_error.into(),
        }
    }
}