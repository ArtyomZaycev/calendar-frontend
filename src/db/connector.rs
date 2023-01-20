use reqwest::{Client, StatusCode};
use serde_json::Value;
use tokio::sync::mpsc::{channel, Sender, Receiver};

use super::request::AppRequest;

type FnSend = Box<dyn FnOnce() + Send>;

pub struct Connector {
    req_sender: Sender<FnSend>,
    req_receiver: Receiver<FnSend>,

    pub client: Client,
    pub api_url: String,

    pub error_handler: Box<dyn FnMut(reqwest::Error) + Send>
}

impl Connector {
    pub fn new() -> Self {
        let (s, r) = channel(5);
        Self {
            req_sender: s,
            req_receiver: r,
            client: Client::new(),
            api_url: "http://127.0.0.1:8080".into(),
            error_handler: Box::new(|e| println!("Request error: {:?}", e)),
        }
    }

    pub fn get_client(&self) -> Client {
        self.client.clone()
    }

    pub fn request(&mut self, request: AppRequest) {
        let client = self.get_client();
        let sender = self.req_sender.clone();
        // TODO: Error handler from self
        let error_handler = |e| {println!("Err: {e:?}")};//self.error_handler.clone();
        tokio::spawn(async move {
            println!("spawn");
            let response = client.execute(request.request).await;
            if let Ok(response) = response {
                if response.status() == StatusCode::OK {
                    if let Some(on_success) = request.on_success {
                        let val = response.json::<Value>().await.unwrap();
                        sender.send(Box::new(|| {
                            on_success(val);
                        })).await.map_err(|_| "TODO").expect("TODO");
                    }
                } else {
                    if let Some(on_error) = request.on_error {
                        sender.send(Box::new(|| {
                            on_error(response);
                        })).await.map_err(|_| "TODO").expect("TODO");
                    }
                }
            } else {
                sender.send(Box::new(move || {
                    error_handler(response.unwrap_err())
                })).await.map_err(|_| "TODO").expect("TODO");
            }
        });
    }

    pub fn poll(&mut self) {
        while let Ok(f) = self.req_receiver.try_recv() {
            f()
        }
    }
}