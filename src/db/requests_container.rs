use super::{
    request::{RequestDescription, RequestId},
    request_parser::{FromResponse, RequestParser},
};
use bytes::Bytes;
use reqwest::StatusCode;
use std::{cell::Cell, collections::HashMap};

pub struct RequestCounter<RequestResponse, RequestInfo, RequestResponseInfo>
where
    RequestResponse: Clone,
    RequestInfo: Clone,
    RequestResponseInfo: Clone + FromResponse<RequestResponse>,
{
    next_request_id: Cell<RequestId>,
    descriptions: HashMap<RequestId, RequestDescription>,
    parsers: HashMap<RequestId, RequestParser<RequestResponse>>,
    infos: HashMap<RequestId, RequestInfo>,
    responses: HashMap<RequestId, RequestResponse>,
    response_infos: HashMap<RequestId, RequestResponseInfo>,
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
            descriptions: HashMap::new(),
            parsers: HashMap::new(),
            infos: HashMap::new(),
            responses: HashMap::new(),
            response_infos: HashMap::new(),
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
        let request_id = self.reserve_id();

        self.parsers.insert(request_id, parser);
        self.infos.insert(request_id, info);
        self.descriptions.insert(request_id, description);

        request_id
    }

    pub fn get_description(&self, id: RequestId) -> Option<RequestDescription> {
        self.descriptions.get(&id).cloned()
    }

    pub fn clone_response(&self, id: RequestId) -> Option<RequestResponse> {
        self.responses.get(&id).cloned()
    }
    fn take_response(&mut self, id: RequestId) -> Option<RequestResponse> {
        self.responses.remove(&id)
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
        self.infos.get(&id).cloned()
    }

    pub fn get_response_info(&self, id: RequestId) -> Option<RequestResponseInfo> {
        self.response_infos.get(&id).cloned()
    }

    pub fn parse(&mut self, id: RequestId, status_code: StatusCode, bytes: Bytes) {
        if let Some(parser) = self.parsers.remove(&id) {
            let response = parser.parse(status_code, bytes);
            let response_info = RequestResponseInfo::from_response(&response);
            self.responses.insert(id, response);
            self.response_infos.insert(id, response_info);
        }
    }
}
