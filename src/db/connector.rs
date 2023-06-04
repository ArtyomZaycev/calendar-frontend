use super::request_parser::RequestParser;
use crate::config::Config;
use bytes::Bytes;
use reqwest::StatusCode;
use std::collections::HashMap;
use std::sync::mpsc::{channel, Receiver, Sender};

type RequestIndex = u16;

pub struct RequestDescriptor<RequestInfo, RequestResponse> {
    description: RequestInfo,
    parser: RequestParser<RequestResponse>,
}

impl<RequestInfo, RequestResponse> RequestDescriptor<RequestInfo, RequestResponse> {
    pub fn new(description: RequestInfo, parser: RequestParser<RequestResponse>) -> Self {
        Self {
            description,
            parser,
        }
    }

    pub fn no_description(parser: RequestParser<RequestResponse>) -> Self
    where
        RequestInfo: Default,
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

struct RequestCounter<RequestInfo, RequestResponse> {
    request_id: RequestIndex,
    requests: HashMap<RequestIndex, RequestDescriptor<RequestInfo, RequestResponse>>,
}

impl<RequestInfo, RequestResponse> RequestCounter<RequestInfo, RequestResponse> {
    fn new() -> Self {
        Self {
            request_id: 0,
            requests: HashMap::new(),
        }
    }

    fn put(&mut self, request: RequestDescriptor<RequestInfo, RequestResponse>) -> RequestIndex {
        let cur_request_id = self.request_id;
        self.request_id += 1;

        self.requests.insert(cur_request_id, request);

        cur_request_id
    }

    fn take(&mut self, id: &RequestIndex) -> Option<RequestDescriptor<RequestInfo, RequestResponse>> {
        self.requests.remove(id)
    }

    fn get_requests_descriptions(&self) -> Vec<RequestInfo>
    where
        RequestInfo: Clone,
    {
        self.requests
            .iter()
            .map(|(_, descriptor)| descriptor.description.clone())
            .collect()
    }
}

pub struct Connector<RequestInfo, RequestResponse> {
    client: reqwest::Client,
    server_url: String,

    requests: RequestCounter<RequestInfo, RequestResponse>,
    sender: Sender<RequestResult>,
    reciever: Receiver<RequestResult>,

    pub error_handler: Box<dyn FnMut(reqwest::Error)>,
}

impl<RequestInfo, RequestResponse> Connector<RequestInfo, RequestResponse> {
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

    pub fn request(&mut self, request: reqwest::Request, descriptor: RequestDescriptor<RequestInfo, RequestResponse>) {
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

    pub fn poll(&mut self) -> Vec<(RequestInfo, RequestResponse)> {
        let mut polled = Vec::new();
        while let Ok(res) = self.reciever.try_recv() {
            let descriptor = self.requests.take(&res.id).expect("Parser not found");

            match res.result {
                Ok((status_code, bytes)) => polled.push((
                    descriptor.description,
                    descriptor.parser.parse(status_code, bytes),
                )),
                Err(error) => (self.error_handler)(error),
            }
        }
        polled
    }

    pub fn get_active_requests_descriptions(&self) -> Vec<RequestInfo>
    where
    RequestInfo: Clone,
    {
        self.requests.get_requests_descriptions()
    }
}
