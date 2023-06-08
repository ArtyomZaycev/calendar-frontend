use super::request::{RequestDescription, RequestId};
use super::request_parser::{FromResponse, RequestParser};
use super::requests_container::RequestCounter;
use crate::config::Config;
use bytes::Bytes;
use reqwest::StatusCode;

use std::sync::mpsc::{channel, Receiver, Sender};

#[derive(Debug)]
struct RequestResult {
    id: RequestId,
    result: reqwest::Result<(StatusCode, Bytes)>,
}
impl RequestResult {
    fn new(id: RequestId, result: reqwest::Result<(StatusCode, Bytes)>) -> Self {
        Self { id, result }
    }
}

pub struct Connector<RequestResponse, RequestInfo, RequestResponseInfo>
where
    RequestResponse: Clone,
    RequestInfo: Clone,
    RequestResponseInfo: Clone + FromResponse<RequestResponse>,
{
    client: reqwest::Client,
    server_url: String,

    requests: RequestCounter<RequestResponse, RequestInfo, RequestResponseInfo>,
    sender: Sender<RequestResult>,
    reciever: Receiver<RequestResult>,

    pub error_handler: Box<dyn FnMut(reqwest::Error)>,
}

impl<RequestResponse, RequestInfo, RequestResponseInfo>
    Connector<RequestResponse, RequestInfo, RequestResponseInfo>
where
    RequestResponse: Clone,
    RequestInfo: Clone,
    RequestResponseInfo: Clone + FromResponse<RequestResponse>,
{
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

    pub fn reserve_request_id(&self) -> RequestId {
        self.requests.reserve_id()
    }

    pub fn request(
        &mut self,
        request: reqwest::Request,
        parser: RequestParser<RequestResponse>,
        info: RequestInfo,
        description: RequestDescription,
    ) -> RequestId {
        use crate::utils::easy_spawn;

        println!("{request:?}");

        let client = self.client.clone();
        let sender = self.sender.clone();

        let request_id = self.requests.push(parser, info, description);

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

    pub fn poll(&mut self) -> Vec<RequestId> {
        let mut polled = Vec::new();
        while let Ok(res) = self.reciever.try_recv() {
            match res.result {
                Ok((status_code, bytes)) => {
                    self.requests.parse(res.id, status_code, bytes);
                    polled.push(res.id)
                }
                Err(error) => (self.error_handler)(error),
            }
        }
        polled
    }

    pub fn get_request_info(&self, request_id: RequestId) -> Option<RequestInfo> {
        self.requests.get_info(request_id)
    }
    pub fn get_response(&mut self, request_id: RequestId) -> Option<RequestResponse> {
        self.requests.get_response(request_id)
    }
    pub fn get_response_info(&self, request_id: RequestId) -> Option<RequestResponseInfo> {
        self.requests.get_response_info(request_id)
    }
    pub fn clone_response(&self, request_id: RequestId) -> Option<RequestResponse> {
        self.requests.clone_response(request_id)
    }

    pub fn any_request_in_progress(&self) -> bool {
        self.requests.any_request_in_progress()
    }
}
