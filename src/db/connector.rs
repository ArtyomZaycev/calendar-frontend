use std::{collections::HashMap, cell::{Cell, RefCell}};

use bytes::Bytes;
use reqwest::StatusCode;
use tokio::sync::mpsc::{channel, Receiver, Sender};

use crate::config::Config;

use super::request_parser::RequestParser;

type RequestIndex = u16;


pub struct RequestDescriptor<T> {
    parser: RequestParser<T>,
}

impl<T> RequestDescriptor<T> {
    pub fn new(parser: RequestParser<T>) -> Self {
        Self {
            parser,
        }
    }
}


#[derive(Debug)]
struct RequestResult {
    id: RequestIndex,
    result: reqwest::Result<(StatusCode, Bytes)>,
}
impl RequestResult {
    fn new(id: RequestIndex, result: reqwest::Result<(StatusCode, Bytes)>) -> Self {
        Self {
            id,
            result,
        }
    }
}

struct RequestCounter<T> {
    request_id: Cell<RequestIndex>,
    requests: RefCell<HashMap<RequestIndex, RequestDescriptor<T>>>,
}

impl<T> RequestCounter<T> {
    fn new() -> Self {
        Self {
            request_id: Cell::new(0),
            requests: RefCell::new(HashMap::new()),
        }
    }

    fn put(&self, request: RequestDescriptor<T>) -> RequestIndex {
        let request_id = self.request_id.get();
        self.request_id.set(request_id + 1);

        let mut requests = self.requests.borrow_mut();
        requests.insert(request_id, request);

        request_id
    }

    fn take(&mut self, id: &RequestIndex) -> Option<RequestDescriptor<T>> {
        self.requests.get_mut().remove(id)
    }

    fn get_requests_descriptions(&self) -> Vec<()> {
        self.requests.borrow().iter().map(|(_, _descriptor)| {
            ()
        }).collect()
    }
}

pub struct Connector<T> {
    client: reqwest::Client,
    server_url: String,

    requests: RequestCounter<T>,
    sender: Sender<RequestResult>,
    reciever: Receiver<RequestResult>,

    pub error_handler: Box<dyn FnMut(reqwest::Error)>,
}

impl<T> Connector<T> {
    pub fn new(config: &Config) -> Self {
        let (sender, reciever) = channel(5);
        Self {
            client: reqwest::Client::new(),
            server_url: config.api_url.clone(),
            requests: RequestCounter::new(),
            sender,
            reciever,
            error_handler: Box::new(|error| println!("ConnectorError: {error:?}")),
        }
    }

    pub fn make_request(&self, method: reqwest::Method, op: &str) -> reqwest::RequestBuilder {
        let client = self.client.clone();
        client.request(method, self.server_url.clone() + op)
    }

    pub fn request(&self, request: reqwest::Request, descriptor: RequestDescriptor<T>) {
        let client = self.client.clone();
        let sender = self.sender.clone();

        let request_id = self.requests.put(descriptor);

        tokio::spawn(async move {
            let res = client.execute(request).await;

            match res {
                Ok(res) => {
                    let status_code = res.status();
                    // TODO: Investigate unwrap
                    let bytes = res.bytes().await.unwrap();
                    sender.send(RequestResult::new(
                        request_id,
                        Ok((status_code, bytes))
                    )).await.expect("Unable to send success response");
                },
                Err(err) => {
                    sender.send(RequestResult::new(request_id, Err(err))).await.expect("Unable to send error response");
                },
            };
        });
    }

    pub fn poll(&mut self) -> Vec<T> {
        let mut polled = Vec::new();
        while let Ok(res) = self.reciever.try_recv() {
            let descriptor = self.requests.take(&res.id).expect("Parser not found");

            match res.result {
                Ok((status_code, bytes)) => polled.push(descriptor.parser.parse(status_code, bytes)),
                Err(error) => (self.error_handler)(error),
            }
        }
        polled
    }

    pub fn get_active_requests_descriptions(&self) -> Vec<()> {
        self.requests.get_requests_descriptions()
    }
}