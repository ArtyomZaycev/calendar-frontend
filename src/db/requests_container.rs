use super::{
    request::{RequestDescription, RequestId},
    request_parser::{FromResponse, RequestParser},
};
use bytes::Bytes;
use reqwest::StatusCode;
use std::{cell::Cell, collections::HashMap};

struct RequestData<RequestResponse, RequestInfo, RequestResponseInfo> {
    info: RequestInfo,
    description: RequestDescription,
    parser: Option<RequestParser<RequestResponse>>,
    response: Option<RequestResponse>,
    response_info: Option<RequestResponseInfo>,
}

impl<RequestResponse, RequestInfo, RequestResponseInfo>
    RequestData<RequestResponse, RequestInfo, RequestResponseInfo>
{
    fn new(
        parser: RequestParser<RequestResponse>,
        info: RequestInfo,
        description: RequestDescription,
    ) -> Self {
        Self {
            info,
            description,
            parser: Some(parser),
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

pub struct RequestCounter<RequestResponse, RequestInfo, RequestResponseInfo>
where
    RequestResponse: Clone,
    RequestInfo: Clone,
    RequestResponseInfo: Clone + FromResponse<RequestResponse>,
{
    next_request_id: Cell<RequestId>,
    requests: HashMap<RequestId, RequestData<RequestResponse, RequestInfo, RequestResponseInfo>>,
}

impl<RequestResponse, RequestInfo, RequestResponseInfo>
    RequestCounter<RequestResponse, RequestInfo, RequestResponseInfo>
where
    RequestResponse: Clone,
    RequestInfo: Clone,
    RequestResponseInfo: Clone + FromResponse<RequestResponse>,
{
    pub fn new() -> Self {
        Self {
            next_request_id: Cell::new(0),
            requests: HashMap::new(),
        }
    }

    pub fn reserve_id(&self) -> RequestId {
        let request_id = self.next_request_id.get();
        self.next_request_id.set(request_id + 1);
        request_id
    }
    pub fn push(
        &mut self,
        parser: RequestParser<RequestResponse>,
        info: RequestInfo,
        description: RequestDescription,
    ) -> RequestId {
        let request_id = description.request_id.unwrap_or_else(|| self.reserve_id());
        self.requests
            .insert(request_id, RequestData::new(parser, info, description));
        request_id
    }

    pub fn get_description(&self, id: RequestId) -> Option<RequestDescription> {
        self.requests.get(&id).map(|d| d.description.clone())
    }

    pub fn clone_response(&self, id: RequestId) -> Option<RequestResponse> {
        self.requests.get(&id).and_then(|d| d.response.clone())
    }
    fn take_response(&mut self, id: RequestId) -> Option<RequestResponse> {
        self.requests.get_mut(&id).and_then(|d| d.response.take())
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
        self.requests.get(&id).map(|d| d.info.clone())
    }

    pub fn get_response_info(&self, id: RequestId) -> Option<RequestResponseInfo> {
        self.requests.get(&id).and_then(|d| d.response_info.clone())
    }

    pub fn any_request_in_progress(&self) -> bool {
        self.requests.iter().any(|(_, data)| data.parser.is_some())
    }

    pub fn parse(&mut self, id: RequestId, status_code: StatusCode, bytes: Bytes) {
        if let Some(data) = self.requests.get_mut(&id) {
            data.parse(status_code, bytes);
        }
    }
}
