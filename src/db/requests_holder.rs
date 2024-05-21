use std::sync::{Arc, Mutex};

use itertools::Itertools;

use super::request::RequestId;

pub(super) struct RequestData {
    pub id: RequestId,
    pub request: reqwest::Request,
}

impl RequestData {
    pub(super) fn new(id: RequestId, request: reqwest::Request) -> Self {
        Self { id, request }
    }
}

/// Keeps count of requests that need to be executed
pub(super) struct RequestsHolder {
    requests: Arc<Mutex<Vec<RequestData>>>,
}

impl RequestsHolder {
    fn new() -> Self {
        Self {
            requests: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn get() -> &'static Self {
        use std::sync::OnceLock;

        static DATA: OnceLock<RequestsHolder> = OnceLock::new();
        DATA.get_or_init(|| RequestsHolder::new())
    }

    pub fn push(&self, request: RequestData) {
        self.requests.lock().unwrap().push(request);
    }
    pub fn take(&self) -> Vec<RequestData> {
        self.requests.lock().unwrap().drain(..).collect_vec()
    }
}
