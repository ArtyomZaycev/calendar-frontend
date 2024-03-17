use std::{cell::RefCell, rc::Rc};

use super::request::RequestId;

pub(super) struct RequestData {
    pub id: RequestId,
    pub request: reqwest::Request,
}

// Rename?
/// Holds requests that should be sent at the end of the frame
pub(super) struct RequestsHolder {
    requests: Rc<RefCell<Vec<RequestData>>>,
}

impl RequestsHolder {
    pub fn new() -> Self {
        Self {
            requests: Rc::new(RefCell::new(Vec::new())),
        }
    }

    pub fn push(&self, request: RequestData) {
        self.requests.borrow_mut().push(request);
    }
    pub fn take(&mut self) -> Vec<RequestData> {
        self.requests.replace(Vec::new())
    }
}
