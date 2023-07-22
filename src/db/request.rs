use reqwest::Method;
use serde::Serialize;

use crate::requests::*;

use super::request_parser::RequestParser;

pub type RequestId = u16;

#[derive(Debug, Clone, Default)]
pub struct RequestDescription {
    pub request_id: Option<RequestId>,
    /// Do not delete request response from the connector when polling or getting the result
    pub save_results: bool,
}

impl RequestDescription {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_request_id(self, request_id: RequestId) -> Self {
        Self {
            request_id: Some(request_id),
            ..self
        }
    }
    pub fn save_results(self) -> Self {
        Self {
            save_results: true,
            ..self
        }
    }
}

pub struct RequestBuilder<Query: Serialize, Body: Serialize, RequestResponse = AppRequestResponse, RequestInfo: Default = AppRequestInfo> {
    method: Option<Method>,
    query: Option<Query>,
    body: Option<Body>,
    authorized: bool,
    parser: Option<RequestParser<RequestResponse>>,
    info: RequestInfo,
}

impl<Query: Serialize, Body: Serialize, RequestResponse, RequestInfo: Default> RequestBuilder<Query, Body, RequestResponse, RequestInfo> {
    pub fn new() -> Self {
        Self {
            method: None,
            query: None,
            body: None,
            authorized: false,
            parser: None,
            info: RequestInfo::default(),
        }
    }

    pub fn with_method(self, method: Method) -> Self {
        Self {
            method: Some(method),
            ..self
        }
    }

    pub fn with_query(self, query: Query) -> Self {
        Self {
            query: Some(query),
            ..self
        }
    }

    pub fn with_body(self, body: Body) -> Self {
        Self {
            body: Some(body),
            ..self
        }
    }

    pub fn authorized(self) -> Self {
        Self {
            authorized: true,
            ..self
        }
    }

    pub fn not_authorized(self) -> Self {
        Self {
            authorized: false,
            ..self
        }
    }

    pub fn with_parser(self, parser: RequestParser<RequestResponse>) -> Self {
        Self {
            parser: Some(parser),
            ..self
        }
    }

    pub fn with_info(self, info: RequestInfo) -> Self {
        Self {
            info,
            ..self
        }
    }

    pub fn build(self, client: reqwest::Client, url: &str, jwt: &str) -> Result<(reqwest::RequestBuilder, RequestParser<RequestResponse>, RequestInfo), ()> {
        let method = self.method.ok_or(())?;
        let parser = self.parser.ok_or(())?;

        let builder = client.request(method, url);
        let builder = if self.authorized {
            builder.bearer_auth(jwt)
        } else {
            builder
        };
        let builder = if let Some(query) = self.query {
            builder.query(&query)
        } else {
            builder
        };
        let builder = if let Some(body) = self.body {
            builder.json(&body)
        } else {
            builder
        };
        Ok((
            builder,
            parser,
            self.info
        ))
    }
}