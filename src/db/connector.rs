use super::request_parser::RequestParser;
use crate::config::Config;
use bytes::Bytes;
use reqwest::StatusCode;
use std::collections::HashMap;
use std::sync::mpsc::{channel, Receiver, Sender};

type RequestIndex = u16;

pub struct RequestDescriptor<Request, RequestDescription> {
    description: RequestDescription,
    parser: RequestParser<Request>,
}

impl<Request, RequestDescription> RequestDescriptor<Request, RequestDescription> {
    pub fn new(description: RequestDescription, parser: RequestParser<Request>) -> Self {
        Self {
            description,
            parser,
        }
    }

    pub fn no_description(parser: RequestParser<Request>) -> Self
    where
        RequestDescription: Default,
    {
        Self {
            description: Default::default(),
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
        Self { id, result }
    }
}

struct RequestCounter<Request, RequestDescription> {
    request_id: RequestIndex,
    requests: HashMap<RequestIndex, RequestDescriptor<Request, RequestDescription>>,
}

impl<T, U> RequestCounter<T, U> {
    fn new() -> Self {
        Self {
            request_id: 0,
            requests: HashMap::new(),
        }
    }

    fn put(&mut self, request: RequestDescriptor<T, U>) -> RequestIndex {
        let cur_request_id = self.request_id;
        self.request_id += 1;

        self.requests.insert(cur_request_id, request);

        cur_request_id
    }

    fn take(&mut self, id: &RequestIndex) -> Option<RequestDescriptor<T, U>> {
        self.requests.remove(id)
    }

    fn get_requests_descriptions(&self) -> Vec<U>
    where
        U: Clone,
    {
        self.requests
            .iter()
            .map(|(_, descriptor)| descriptor.description.clone())
            .collect()
    }
}

pub struct Connector<Request, RequestDescription> {
    client: reqwest::Client,
    server_url: String,

    requests: RequestCounter<Request, RequestDescription>,
    sender: Sender<RequestResult>,
    reciever: Receiver<RequestResult>,

    pub error_handler: Box<dyn FnMut(reqwest::Error)>,
}

impl<T, U> Connector<T, U> {
    pub fn new(config: &Config) -> Self {
        let (sender, reciever) = channel();
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
        client
            .request(method, self.server_url.clone() + op)
            .header("Access-Control-Allow-Origin", "*")
    }

    pub fn request(&mut self, request: reqwest::Request, descriptor: RequestDescriptor<T, U>) {
        use crate::utils::easy_spawn;

        println!("{request:?}");

        let client = self.client.clone();
        let sender = self.sender.clone();

        let request_id = self.requests.put(descriptor);

        easy_spawn(async move {
            let res = client.execute(request).await;

            match res {
                Ok(res) => {
                    let status_code = res.status();
                    // TODO: Investigate unwrap
                    let bytes = res.bytes().await.unwrap();
                    sender
                        .send(RequestResult::new(request_id, Ok((status_code, bytes))))
                        //.await
                        .expect("Unable to send success response");
                }
                Err(err) => {
                    sender
                        .send(RequestResult::new(request_id, Err(err)))
                        //.await
                        .expect("Unable to send error response");
                }
            };
        });
    }

    pub fn poll(&mut self) -> Vec<(T, U)> {
        let mut polled = Vec::new();
        while let Ok(res) = self.reciever.try_recv() {
            let descriptor = self.requests.take(&res.id).expect("Parser not found");

            match res.result {
                Ok((status_code, bytes)) => polled.push((
                    descriptor.parser.parse(status_code, bytes),
                    descriptor.description,
                )),
                Err(error) => (self.error_handler)(error),
            }
        }
        polled
    }

    pub fn get_active_requests_descriptions(&self) -> Vec<U>
    where
        U: Clone,
    {
        self.requests.get_requests_descriptions()
    }
}
