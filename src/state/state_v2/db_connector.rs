use std::any::Any;
use std::cell::Ref;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::{cell::RefCell, rc::Rc};

use bytes::Bytes;
use reqwest::StatusCode;
use serde::de::DeserializeOwned;

use crate::config::Config;

use super::request::RequestId;

struct RequestResult<T> {
    id: RequestId,
    result: reqwest::Result<(reqwest::StatusCode, T)>,
}

impl<T> RequestResult<T> {
    fn new(id: RequestId, result: reqwest::Result<(reqwest::StatusCode, T)>) -> Self {
        Self { id, result }
    }
}

impl RequestResult<Bytes> {
    fn map_to_any<T: DeserializeOwned + 'static, E: DeserializeOwned + 'static>(
        self,
    ) -> RequestResult<Box<dyn Any>> {
        RequestResult::new(
            self.id,
            self.result.map(|(status, bytes)| {
                let b: Box<dyn Any> = if status == StatusCode::OK {
                    Box::new(serde_json::from_slice::<T>(&bytes).unwrap())
                } else if status == StatusCode::BAD_REQUEST {
                    Box::new(serde_json::from_slice::<E>(&bytes).unwrap())
                } else {
                    Box::new(String::from_utf8_lossy(&bytes.to_vec()).to_string())
                };
                (status, b)
            }),
        )
    }
}
/*
pub enum RequestResultTyped<T, E> {
    OnSuccess(Box<T>),
    OnBadRequest(Box<E>),
    Other(String),
}

impl RequestResult<Box<dyn Any>> {
    fn take_typed<T, E>(self) -> RequestResultTyped<T, E> {
        if self.status == StatusCode::OK {
            OnSuccess(T)
        } else if status == StatusCode::BAD_REQUEST {
            Box::new(serde_json::from_slice::<E>(&bytes).unwrap())
        } else {
            Box::new(bytes.as_bytes())
        }
    }
}
 */
pub struct DbConnector {
    client: reqwest::Client,
    server_url: String,

    sender: Sender<RequestResult<Bytes>>,
    reciever: Receiver<RequestResult<Bytes>>,

    // We should store 2 arrays:
    // Array of bytes. Just recieved responses, we still don't know the type
    // Array of Any. Recieved, and were retrieved by ref, so we converted from array of bytes.
    // And we have to wrap them, to convert from one to another in &self
    results: Rc<RefCell<Vec<RequestResult<Bytes>>>>,
    typed_results: Rc<RefCell<Vec<RequestResult<Box<dyn Any>>>>>,

    pub error_handler: Box<dyn FnMut(reqwest::Error)>,

    next_request_id: RequestId,
}

impl DbConnector {
    pub fn new(config: &Config) -> Self {
        let (sender, reciever) = channel();
        Self {
            client: reqwest::Client::new(),
            server_url: config.api_url.clone(),
            sender,
            reciever,
            results: Rc::new(RefCell::new(Vec::new())),
            typed_results: Rc::new(RefCell::new(Vec::new())),
            error_handler: Box::new(|error| println!("ConnectorError: {error:?}")),
            next_request_id: RequestId::default(),
        }
    }

    pub fn request(&mut self, request: reqwest::Request) -> RequestId {
        use crate::utils::easy_spawn;

        let request_id = self.next_request_id;
        self.next_request_id += 1;

        let client = self.client.clone();
        let sender = self.sender.clone();
        easy_spawn(async move {
            let res: Result<reqwest::Response, reqwest::Error> = client.execute(request).await;

            match res {
                Ok(res) => {
                    let status_code = res.status();
                    // TODO: Investigate unwrap
                    let bytes = res.bytes().await.unwrap();
                    sender
                        .send(RequestResult::new(request_id, Ok((status_code, bytes))))
                        .expect("Unable to send success response");
                }
                Err(err) => {
                    sender
                        .send(RequestResult::new(request_id, Err(err)))
                        .expect("Unable to send error response");
                }
            };
        });

        request_id
    }

    pub fn pull(&mut self) {
        let mut pulled = self.reciever.try_iter().collect::<Vec<_>>();
        self.results.borrow_mut().append(&mut pulled);
    }

    pub fn convert_response<T: DeserializeOwned + 'static, E: DeserializeOwned + 'static>(
        &self,
        id: RequestId,
    ) {
        let mut results = self.results.borrow_mut();
        let result = results
            .iter()
            .position(|r| r.id == id)
            .map(|index| results.swap_remove(index));
        if let Some(result) = result {
            self.typed_results
                .borrow_mut()
                .push(result.map_to_any::<T, E>());
        }
    }

    pub fn get_response<'a, T: 'static, E: 'static>(
        &'a self,
        id: RequestId,
    ) -> Option<Result<Result<Ref<'a, T>, Ref<'a, E>>, String>> {
        let typed_results = self.typed_results.borrow();

        // TODO: Rewrite more functional
        let request_result = Ref::filter_map(typed_results, |typed_results| {
            typed_results.iter().find(|r| r.id == id)
        })
        .ok()?;
        Some(match Ref::filter_map(request_result, |result| result.result.as_ref().err()) {
            Ok(error) => Err(error.to_string()),
            Err(request_result) => {
                let (status, response) = Ref::map_split(
                    Ref::map(request_result, |result| result.result.as_ref().unwrap()),
                    |(status, response)| (status, response),
                );
                let status = *status;
                if status == StatusCode::OK {
                    Ok(Ok(Ref::map(response, |response| {
                        response.downcast_ref::<T>().unwrap()
                    })))
                } else if status == StatusCode::BAD_REQUEST {
                    Ok(Err(Ref::map(response, |response| {
                        response.downcast_ref::<E>().unwrap()
                    })))
                } else {
                    Err(Ref::map(response, |response| {
                        response.downcast_ref::<String>().unwrap()
                    })
                    .clone())
                }
            }
        })
    }

    /// Doesn't have to be mutable
    pub fn take_response<T: 'static, E: 'static>(
        &mut self,
        id: RequestId,
    ) -> Option<Result<Result<Box<T>, Box<E>>, String>> {
        let mut typed_results = self.typed_results.borrow_mut();

        let request_result = typed_results
            .iter()
            .position(|result| result.id == id)
            .map(|index| typed_results.swap_remove(index))?;

        Some(match request_result.result {
            Ok((status, response)) => {
                if status == StatusCode::OK {
                    Ok(Ok(response.downcast::<T>().ok().unwrap()))
                } else if status == StatusCode::BAD_REQUEST {
                    Ok(Err(response.downcast::<E>().ok().unwrap()))
                } else {
                    Err(response.downcast_ref::<String>().unwrap().clone())
                }
            }
            Err(err) => Err(err.to_string()),
        })
    }
}
