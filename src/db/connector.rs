use reqwest::{Client, RequestBuilder, Response, StatusCode};
use serde_json::Value;
use tokio::sync::oneshot::{channel, Receiver};

use super::request::AppRequest;

#[derive(Debug)]
enum ReqRes {
    Success(Value),
    BadResponse(Response),
    Error(reqwest::Error),
}
type RequestResult<T> = (AppRequest<T>, Receiver<ReqRes>);

pub struct Connector<T> {
    requests: Vec<RequestResult<T>>,
    client: Client,

    pub api_url: String,

    pub error_handler: Box<dyn FnMut(reqwest::Error)>,
}

impl<T> Connector<T> {
    pub fn new() -> Self {
        Self {
            requests: Vec::default(),
            client: Client::new(),
            api_url: "http://127.0.0.1:8080/".into(),
            error_handler: Box::new(|e| println!("Request error: {:?}", e)),
        }
    }

    pub fn get_client(&self) -> Client {
        self.client.clone()
    }

    pub fn make_request(&self, method: reqwest::Method, op: &str) -> RequestBuilder {
        let client = self.get_client();
        client.request(method, self.api_url.clone() + op)
    }

    pub fn make_request_authorized(
        &self,
        method: reqwest::Method,
        op: &str,
        uid: i32,
        key: &[u8],
    ) -> RequestBuilder {
        let client = self.get_client();
        client
            .request(method, self.api_url.clone() + op)
            .basic_auth(uid, Some(std::str::from_utf8(key).expect("parse error")))
    }

    pub fn request(&mut self, request: AppRequest<T>) {
        let client = self.get_client();
        let (s, r) = channel::<ReqRes>();
        // TODO: Remove clone
        let req = request.request.try_clone().unwrap();
        tokio::spawn(async move {
            let res = client.execute(req).await;
            if let Ok(res) = res {
                if res.status() == StatusCode::OK {
                    s.send(ReqRes::Success(res.json().await.unwrap_or_default()))
                        .unwrap();
                } else {
                    s.send(ReqRes::BadResponse(res)).unwrap();
                }
            } else {
                let err = res.unwrap_err();
                s.send(ReqRes::Error(err)).unwrap();
            }
        });
        self.requests.push((request, r));
    }

    pub fn poll(&mut self) -> Vec<T> {
        let requests = &mut self.requests;
        let indicies = requests.iter_mut().enumerate().filter_map(|(i, (_, res))| {
            if let Ok(res) = res.try_recv() {
                Some((i, res))
            } else {
                None
            }
        });

        let error_handler = &mut self.error_handler;
        indicies
            .collect::<Vec<_>>()
            .into_iter()
            .rev()
            .filter_map(|(i, res)| {
                let (req, _) = requests.swap_remove(i);
                match res {
                    ReqRes::Success(value) => {
                        if let Some(on_success) = req.on_success {
                            Some(on_success(value))
                        } else {
                            None
                        }
                    }
                    ReqRes::BadResponse(response) => {
                        if let Some(on_error) = req.on_error {
                            Some(on_error(response))
                        } else {
                            None
                        }
                    }
                    ReqRes::Error(err) => {
                        error_handler(err);
                        None
                    }
                }
            })
            .collect()
    }
}
