use std::any::Any;
use std::cell::Ref;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::{cell::RefCell, marker::PhantomData, rc::Rc};

use bytes::Bytes;
use calendar_lib::api::{
    event_templates::types::EventTemplate, events::types::Event, schedules::types::Schedule,
};
use serde::Deserialize;
use serde::{de::DeserializeOwned, Serialize};

use crate::config::Config;

pub type RequestId = u16;
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
    fn map_to_any<T: DeserializeOwned + 'static>(self) -> RequestResult<Box<dyn Any>> {
        RequestResult::new(
            self.id,
            self.result.map(|(status, bytes)| {
                let b: Box<dyn Any> = Box::new(serde_json::from_slice::<T>(&bytes).unwrap());
                (status, b)
            }),
        )
    }
}

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

    pub fn convert_response<T: DeserializeOwned + 'static>(&self, id: RequestId) {
        let mut results = self.results.borrow_mut();
        let result = results
            .iter()
            .position(|r| r.id == id)
            .map(|index| results.swap_remove(index));
        if let Some(result) = result {
            self.typed_results
                .borrow_mut()
                .push(result.map_to_any::<T>());
        }
    }

    pub fn get_response<'a, T: 'static>(&'a self, id: RequestId) -> Option<Ref<'a, T>> {
        let typed_results = self.typed_results.borrow();
        Ref::filter_map(typed_results, |typed_results| {
            let x = typed_results.iter().find(|r| r.id == id);
            x.and_then(|result| {
                result
                    .result
                    .as_ref()
                    .ok()
                    .map(|(status, result)| result.downcast_ref::<T>().unwrap())
            })
        })
        .ok()
    }

    pub fn take_response<T: 'static>(&mut self, id: RequestId) -> Option<Box<T>> {
        let mut typed_results = self.typed_results.borrow_mut();
        let x = typed_results
            .iter()
            .position(|result| result.id == id)
            .map(|index| typed_results.swap_remove(index));
        let x = if let Some(result) = x {
            result.result.ok().and_then(|(status, result)| {
                if !status.is_success() {
                    return None;
                }

                let x = result.downcast::<T>().ok();

                x
            })
        } else {
            None
        };

        x
    }
}

pub trait RequestType {
    const URL: String;
    const IS_AUTHORIZED: bool;
    const METHOD: reqwest::Method;

    type Query;
    type Body = ();
    type Response: DeserializeOwned;

    // TODO
    //type BadResponse: DeserializeOwned = ();

    /// e.g. update request item.id
    type Info: Clone = ();

    fn push_to_state(response: Self::Response, info: Self::Info, state: &mut State);
}

pub struct RequestIdentifier<T: RequestType> {
    id: RequestId,
    info: T::Info,
    _data: PhantomData<T>,
}

struct RequestData {
    pub id: RequestId,
    pub request: reqwest::Request,
}

/// Holds requests that should be sent at the end of the frame
struct RequestsHolder {
    requests: Rc<RefCell<Vec<RequestData>>>,
}

// Rename?
impl RequestsHolder {}

impl RequestsHolder {
    pub fn new() -> Self {
        Self {
            requests: Rc::new(RefCell::new(Vec::new())),
        }
    }

    pub fn push(&self, request: RequestData) {
        self.requests.borrow_mut().push(request);
    }
    pub fn take(&mut self) -> Vec<RequestData> {
        self.requests.replace(Vec::new())
    }
}

pub struct StateTable<T> {
    data: Vec<T>,
    requests: RequestsHolder,
}

impl<T> StateTable<T> {
    fn new() -> Self {
        Self {
            data: Vec::new(),
            requests: RequestsHolder::new(),
        }
    }
}

pub struct State {
    db_connector: DbConnector,
    pub events: StateTable<Event>,
    pub schedules: StateTable<Schedule>,
    pub event_templates: StateTable<EventTemplate>,
    requests: RequestsHolder,
}

impl State {
    pub fn new(config: &Config) -> Self {
        State {
            db_connector: DbConnector::new(config),
            events: StateTable::new(),
            schedules: StateTable::new(),
            event_templates: StateTable::new(),
            requests: RequestsHolder::new(),
        }
    }

    // TODO: Option<reqwest::Error<T::Response>>, to find out about failder requests
    pub fn get_response<'a, T: RequestType + 'static>(
        &'a self,
        identifier: RequestIdentifier<T>,
    ) -> Option<Ref<'a, T::Response>> {
        self.db_connector
            .convert_response::<T::Response>(identifier.id);
        self.db_connector.get_response::<T::Response>(identifier.id)
    }

    pub fn take_response<T: RequestType + 'static>(
        &mut self,
        identifier: RequestIdentifier<T>,
    ) -> Option<Box<T::Response>> {
        self.db_connector
            .convert_response::<T::Response>(identifier.id);
        self.db_connector
            .take_response::<T::Response>(identifier.id)
    }
}
