use std::{cell::RefCell, rc::Rc};

use super::{
    db_connector::DbConnectorData,
    main_state::{RequestIdentifier, RequestType},
    request::RequestId,
};

pub(super) struct RequestData {
    pub id: RequestId,
    pub request: reqwest::Request,
}

impl RequestData {
    pub(super) fn new(id: RequestId, request: reqwest::Request) -> Self {
        Self { id, request }
    }
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

    pub(super) fn make_typical_request<T: RequestType, F>(
        &self,
        info: T::Info,
        make_request: F,
    ) -> RequestIdentifier<T>
    where
        F: FnOnce(&DbConnectorData) -> reqwest::RequestBuilder,
    {
        let connector = DbConnectorData::get();
        let request_id = connector.next_request_id();
        let request = make_request(connector);
        self.push(RequestData::new(request_id, request.build().unwrap()));
        RequestIdentifier::new(request_id, info)
    }
}
