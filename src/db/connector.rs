use super::request::{RequestBuilder, RequestDescription, RequestId};
use super::request_parser::{FromResponse, RequestParser};
use super::requests_container::RequestCounter;
use crate::config::Config;
use bytes::Bytes;
use reqwest::StatusCode;
use serde::Serialize;

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

pub struct DbConnector<RequestResponse, RequestInfo, RequestResponseInfo, 
Callback>
where
    RequestResponse: Clone,
    RequestInfo: Clone + Default,
    RequestResponseInfo: Clone + FromResponse<RequestResponse>,
{
    client: reqwest::Client,
    server_url: String,

    requests: RequestCounter<RequestResponse, RequestInfo, RequestResponseInfo, Callback>,
    sender: Sender<RequestResult>,
    reciever: Receiver<RequestResult>,

    pub error_handler: Box<dyn FnMut(reqwest::Error)>,
}

impl<RequestResponse, RequestInfo, RequestResponseInfo, Callback>
    DbConnector<RequestResponse, RequestInfo, RequestResponseInfo, Callback>
where
    RequestResponse: Clone,
    RequestInfo: Clone + Default,
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

    pub fn request2<Q: Serialize, B: Serialize>(
        &self,
        request: RequestBuilder<Q, B, Callback, RequestResponse, RequestInfo>,
        jwt: &str,
        description: RequestDescription,
    ) -> Result<RequestId, ()> {
        let (request, parser, info, callback) =
            request.build(self.client.clone(), &self.server_url, jwt)?;
        let request = request.build().map_err(|_| ())?;

        Ok(self.request(request, parser, info, description, callback))
    }

    pub fn request(
        &self,
        request: reqwest::Request,
        parser: RequestParser<RequestResponse>,
        info: RequestInfo,
        description: RequestDescription,
        callback: Option<Callback>,
    ) -> RequestId {
        use crate::utils::easy_spawn;

        println!("{request:?}");

        let client = self.client.clone();
        let sender = self.sender.clone();

        let request_id = self.requests.push(parser, info, description, callback);

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
    pub fn take_callback(&mut self, request_id: RequestId) -> Option<Callback> {
        self.requests.take_callback(request_id)
    }

    pub fn any_request_in_progress(&self) -> bool {
        self.requests.any_request_in_progress()
    }
}
