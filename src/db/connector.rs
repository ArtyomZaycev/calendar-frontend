use std::{collections::HashMap, cell::{Cell, RefCell}};

use bytes::Bytes;
use reqwest::StatusCode;
use tokio::sync::mpsc::{channel, Receiver, Sender};

use super::request_parser::RequestParser;

type RequestIndex = u16;

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
    requests: RefCell<HashMap<RequestIndex, RequestParser<T>>>,
}

impl<T> RequestCounter<T> {
    fn new() -> Self {
        Self {
            request_id: 0.into(),
            requests: RefCell::new(HashMap::new()),
        }
    }

    fn put(&self, parser: RequestParser<T>) -> RequestIndex {
        let request_id = self.request_id.get();
        self.request_id.set(request_id + 1);

        let mut requests = self.requests.borrow_mut();
        requests.insert(request_id, parser);

        request_id
    }

    fn take(&mut self, id: &RequestIndex) -> Option<RequestParser<T>> {
        self.requests.get_mut().remove(id)
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
    pub fn new() -> Self {
        let (sender, reciever) = channel(5);
        Self {
            client: reqwest::Client::new(),
            server_url: "http://127.0.0.1:8080/".into(),
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

    pub fn request(&self, request: reqwest::Request, parser: RequestParser<T>) {
        let client = self.client.clone();
        let sender = self.sender.clone();

        let request_id = self.requests.put(parser);

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
            let parser = self.requests.take(&res.id).expect("Parser not found");

            match res.result {
                Ok((status_code, bytes)) => polled.push(parser.parse(status_code, bytes)),
                Err(error) => (self.error_handler)(error),
            }
        }
        polled
    }
}