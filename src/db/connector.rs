use super::request_parser::{RequestParser, FromResponse};
use crate::config::Config;
use bytes::Bytes;
use reqwest::StatusCode;
use std::collections::HashMap;
use std::sync::mpsc::{channel, Receiver, Sender};

pub type RequestIndex = u16;

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

struct RequestCounter<RequestResponse, RequestInfo, RequestResponseInfo> where RequestInfo: Clone, RequestResponseInfo: Clone+FromResponse<RequestResponse> {
    next_request_id: RequestIndex,
    parsers: HashMap<RequestIndex, RequestParser<RequestResponse>>,
    infos: HashMap<RequestIndex, RequestInfo>,
    responses: HashMap<RequestIndex, RequestResponse>,
    response_infos: HashMap<RequestIndex, RequestResponseInfo>,
}

impl<RequestResponse, RequestInfo, RequestResponseInfo> RequestCounter<RequestResponse, RequestInfo, RequestResponseInfo> where RequestInfo: Clone, RequestResponseInfo: Clone+FromResponse<RequestResponse> {
    fn new() -> Self {
        Self {
            next_request_id: 0,
            parsers: HashMap::new(),
            infos: HashMap::new(),
            responses: HashMap::new(),
            response_infos: HashMap::new(),
        }
    }

    fn push(&mut self, parser: RequestParser<RequestResponse>, info: RequestInfo) -> RequestIndex {
        let request_id = self.next_request_id;
        self.next_request_id += 1;

        self.parsers.insert(request_id, parser);
        self.infos.insert(request_id, info);

        request_id
    }
    
    fn get_info(&mut self, id: RequestIndex) -> Option<RequestInfo> {
        self.infos.get(&id).cloned()
    }
    fn get_response_info(&mut self, id: RequestIndex) -> Option<RequestResponseInfo> {
        self.response_infos.get(&id).cloned()
    }

    fn take_response(&mut self, id: RequestIndex) -> Option<RequestResponse> {
        self.responses.remove(&id)
    }

    fn parse(&mut self, id: RequestIndex, status_code: StatusCode, bytes: Bytes) {
        if let Some(parser) = self.parsers.remove(&id) {
            let response = parser.parse(status_code, bytes);
            let response_info = RequestResponseInfo::from_response(&response);
            self.responses.insert(id, response);
            self.response_infos.insert(id, response_info);
        }
    }
}

pub struct Connector<RequestResponse, RequestInfo, RequestResponseInfo> where RequestInfo: Clone, RequestResponseInfo: Clone+FromResponse<RequestResponse> {
    client: reqwest::Client,
    server_url: String,

    requests: RequestCounter<RequestResponse, RequestInfo, RequestResponseInfo>,
    sender: Sender<RequestResult>,
    reciever: Receiver<RequestResult>,

    pub error_handler: Box<dyn FnMut(reqwest::Error)>,
}

impl<RequestResponse, RequestInfo, RequestResponseInfo> Connector<RequestResponse, RequestInfo, RequestResponseInfo> where RequestInfo: Clone, RequestResponseInfo: Clone+FromResponse<RequestResponse> {
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

    pub fn request(&mut self, request: reqwest::Request, parser: RequestParser<RequestResponse>, info: RequestInfo) -> RequestIndex {
        use crate::utils::easy_spawn;

        println!("{request:?}");

        let client = self.client.clone();
        let sender = self.sender.clone();

        let request_id = self.requests.push(parser, info);

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

        request_id
    }
    
    pub fn poll(&mut self) -> Vec<(RequestInfo, RequestResponse)> {
        let mut polled = Vec::new();
        while let Ok(res) = self.reciever.try_recv() {
            match res.result {
                Ok((status_code, bytes)) => {
                    self.requests.parse(res.id, status_code, bytes);
                    polled.push(res.id)
                },
                Err(error) => (self.error_handler)(error),
            }
        }
        polled.into_iter().filter_map(|id| {
            self.requests.get_info(id).and_then(|info| {
                self.requests.take_response(id).map(|response| (info, response))
            })
        }).collect()
    }
}
