use super::{
    request::{RequestDescription, RequestId},
    request_parser::{FromResponse, RequestParser},
};
use bytes::Bytes;
use reqwest::StatusCode;
use std::{
    cell::{Cell, RefCell},
    collections::HashMap,
};

struct RequestData<RequestResponse, RequestInfo, RequestResponseInfo, Callback> {
    info: RequestInfo,
    description: RequestDescription,
    parser: Option<RequestParser<RequestResponse>>,
    callback: Option<Callback>,

    response: Option<RequestResponse>,
    response_info: Option<RequestResponseInfo>,
}

impl<RequestResponse, RequestInfo, RequestResponseInfo, Callback>
    RequestData<RequestResponse, RequestInfo, RequestResponseInfo, Callback>
{
    fn new(
        parser: RequestParser<RequestResponse>,
        info: RequestInfo,
        description: RequestDescription,
        callback: Option<Callback>,
    ) -> Self {
        Self {
            info,
            description,
            parser: Some(parser),
            callback,
            response: None,
            response_info: None,
        }
    }

    fn parse(&mut self, status_code: StatusCode, bytes: Bytes)
    where
        RequestResponseInfo: FromResponse<RequestResponse>,
    {
        if let Some(parser) = self.parser.take() {
            let response = parser.parse(status_code, bytes);
            let response_info = RequestResponseInfo::from_response(&response);

            self.response = Some(response);
            self.response_info = Some(response_info);
        }
    }
}

pub struct RequestCounter<RequestResponse, RequestInfo, RequestResponseInfo, Callback>
where
    RequestResponse: Clone,
    RequestInfo: Clone,
    RequestResponseInfo: Clone + FromResponse<RequestResponse>,
{
    next_request_id: Cell<RequestId>,
    requests:
        RefCell<HashMap<RequestId, RequestData<RequestResponse, RequestInfo, RequestResponseInfo, Callback>>>,
}

impl<RequestResponse, RequestInfo, RequestResponseInfo, Callback>
    RequestCounter<RequestResponse, RequestInfo, RequestResponseInfo, Callback>
where
    RequestResponse: Clone,
    RequestInfo: Clone,
    RequestResponseInfo: Clone + FromResponse<RequestResponse>,
{
    pub fn new() -> Self {
        Self {
            next_request_id: Cell::new(0),
            requests: RefCell::new(HashMap::new()),
        }
    }

    pub fn reserve_id(&self) -> RequestId {
        let request_id = self.next_request_id.get();
        self.next_request_id.set(request_id + 1);
        request_id
    }
    pub fn push(
        &self,
        parser: RequestParser<RequestResponse>,
        info: RequestInfo,
        description: RequestDescription,
        callback: Option<Callback>,
    ) -> RequestId {
        let request_id = description.request_id.unwrap_or_else(|| self.reserve_id());
        self.requests.borrow_mut().insert(
            request_id,
            RequestData::new(parser, info, description, callback),
        );
        request_id
    }

    pub fn get_description(&self, id: RequestId) -> Option<RequestDescription> {
        self.requests
            .borrow()
            .get(&id)
            .map(|d| d.description.clone())
    }

    pub fn clone_response(&self, id: RequestId) -> Option<RequestResponse> {
        self.requests
            .borrow()
            .get(&id)
            .and_then(|d| d.response.clone())
    }
    fn take_response(&mut self, id: RequestId) -> Option<RequestResponse> {
        self.requests
            .borrow_mut()
            .get_mut(&id)
            .and_then(|d| d.response.take())
    }
    /// Either take or clone, depending on RequestDescription
    pub fn get_response(&mut self, id: RequestId) -> Option<RequestResponse> {
        let description = self.get_description(id).unwrap_or_default();
        if description.save_results {
            self.clone_response(id)
        } else {
            self.take_response(id)
        }
    }

    pub fn get_info(&self, id: RequestId) -> Option<RequestInfo> {
        self.requests.borrow().get(&id).map(|d| d.info.clone())
    }

    pub fn get_response_info(&self, id: RequestId) -> Option<RequestResponseInfo> {
        self.requests
            .borrow()
            .get(&id)
            .and_then(|d| d.response_info.clone())
    }

    pub fn take_callback(&mut self, id: RequestId) -> Option<Callback> {
        self.requests
            .borrow_mut()
            .get_mut(&id)
            .and_then(|d| d.callback.take())
    }

    pub fn any_request_in_progress(&self) -> bool {
        self.requests
            .borrow()
            .iter()
            .any(|(_, data)| data.response.is_none())
    }

    pub fn parse(&mut self, id: RequestId, status_code: StatusCode, bytes: Bytes) {
        if let Some(data) = self.requests.borrow_mut().get_mut(&id) {
            data.parse(status_code, bytes);
        }
    }
}
