use std::any::Any;
use std::cell::Ref;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::{atomic, Arc, RwLock};
use std::{cell::RefCell, rc::Rc};

use bytes::Bytes;
use reqwest::StatusCode;
use serde::de::DeserializeOwned;

use crate::config::Config;

use super::main_state::RequestType;
use super::request::{RequestId, RequestIdAtomic};
use super::requests_holder::RequestData;

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

// TODO: Rename with DbConnector
pub struct DbConnectorData {
    client: reqwest::Client,
    server_url: String,
    jwt: Arc<RwLock<Option<String>>>,
    next_request_id: Arc<RequestIdAtomic>,
}

impl DbConnectorData {
    fn new(config: &Config) -> Self {
        Self {
            client: reqwest::Client::new(),
            server_url: config.api_url.clone(),
            jwt: Arc::new(RwLock::new(None)),
            next_request_id: Arc::new(RequestIdAtomic::default()),
        }
    }

    pub fn get() -> &'static Self {
        use std::sync::OnceLock;

        static DATA: OnceLock<DbConnectorData> = OnceLock::new();
        DATA.get_or_init(|| DbConnectorData::new(&Config::load()))
    }

    pub(super) fn next_request_id(&self) -> RequestId {
        self.next_request_id
            .fetch_add(1, atomic::Ordering::SeqCst)
            .into()
    }

    pub(super) fn make_request<T: RequestType>(&self) -> reqwest::RequestBuilder {
        let method = T::METHOD;
        let op = T::URL;
        let authorize = T::IS_AUTHORIZED;
        self.make_request2(method, op, authorize)
    }

    pub(super) fn make_request2(
        &self,
        method: reqwest::Method,
        op: &str,
        authorize: bool,
    ) -> reqwest::RequestBuilder {
        let client = self.client.clone();
        let request = client
            .request(method, self.server_url.clone() + op)
            // TODO: Proper value
            .header("Access-Control-Allow-Origin", "*");
        if authorize {
            let jwt = self.jwt.read().unwrap();
            request.bearer_auth(jwt.clone().unwrap_or_default())
        } else {
            request
        }
    }

    pub(super) fn push_jwt(&self, jwt: String) {
        *self.jwt.write().unwrap() = Some(jwt);
    }
}

pub struct DbConnector {
    sender: Sender<RequestResult<Bytes>>,
    reciever: Receiver<RequestResult<Bytes>>,

    // We should store 2 arrays:
    // Array of bytes. Just recieved responses, we still don't know the type
    // Array of Any. Recieved, and were retrieved by ref, so we converted from array of bytes.
    // And we have to wrap them, to convert from one to another in &self
    results: Rc<RefCell<Vec<RequestResult<Bytes>>>>,
    typed_results: Rc<RefCell<Vec<RequestResult<Box<dyn Any>>>>>,

    pub error_handler: Box<dyn FnMut(reqwest::Error)>,
}

impl DbConnector {
    pub fn new() -> Self {
        let (sender, reciever) = channel();
        Self {
            sender,
            reciever,
            results: Rc::new(RefCell::new(Vec::new())),
            typed_results: Rc::new(RefCell::new(Vec::new())),
            error_handler: Box::new(|error| println!("ConnectorError: {error:?}")),
        }
    }

    pub(super) fn request(&mut self, request: RequestData) -> RequestId {
        use crate::utils::easy_spawn;

        let data = DbConnectorData::get();
        let RequestData {
            id: request_id,
            request,
        } = request;

        let client = data.client.clone();
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

    pub fn is_request_completed(&self, id: RequestId) -> bool {
        self.results.borrow().iter().any(|result| result.id == id)
            || self
                .typed_results
                .borrow()
                .iter()
                .any(|result| result.id == id)
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
        Some(
            match Ref::filter_map(request_result, |result| result.result.as_ref().err()) {
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
            },
        )
    }

    pub fn take_response<T: 'static, E: 'static>(
        &self,
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

    /// Can't find requests that failed for unknown reasons, not advisable to use
    pub fn find_response_by_type<'a, T: 'static, E: 'static>(
        &'a self,
    ) -> Option<Result<Ref<'a, T>, Ref<'a, E>>> {
        let typed_results = self.typed_results.borrow();

        let request_result = Ref::filter_map(typed_results, |typed_results| {
            typed_results.iter().find(|r| {
                r.result.as_ref().is_ok_and(|(status, response)| {
                    if *status == StatusCode::OK {
                        response.is::<T>()
                    } else if *status == StatusCode::BAD_REQUEST {
                        response.is::<E>()
                    } else {
                        false
                    }
                })
            })
        })
        .ok()?;

        // This probably can be simplified, but I can't even understand how it works
        Ref::filter_map(request_result, |result| result.result.as_ref().err())
            .err()
            .and_then(|request_result| {
                let (status, response) = Ref::map_split(
                    Ref::map(request_result, |result| result.result.as_ref().unwrap()),
                    |(status, response)| (status, response),
                );
                let status = *status;
                if status == StatusCode::OK {
                    Some(Ok(Ref::map(response, |response| {
                        response.downcast_ref::<T>().unwrap()
                    })))
                } else if status == StatusCode::BAD_REQUEST {
                    Some(Err(Ref::map(response, |response| {
                        response.downcast_ref::<E>().unwrap()
                    })))
                } else {
                    None
                }
            })
    }
}
