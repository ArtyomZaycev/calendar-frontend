use reqwest::Method;
use serde::Serialize;

use crate::requests::*;

use super::request_parser::RequestParser;

pub type RequestId = u16;

#[derive(Debug, Clone)]
pub struct RequestDescription {
    pub request_id: Option<RequestId>,
    /// Do not delete request response from the connector when polling or getting the result
    pub save_results: bool,
    /// Clone Response for callback
    pub clone_for_callback: bool
}

impl Default for RequestDescription {
    fn default() -> Self {
        Self { request_id: None, save_results: false, clone_for_callback: true }
    }
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

pub struct RequestBuilder<
    Query: Serialize,
    Body: Serialize,
    Callback,
    RequestResponse = AppRequestResponse,
    RequestInfo: Default = AppRequestInfo,
> {
    method: Option<Method>,
    path: String,
    query: Option<Query>,
    body: Option<Body>,
    authorized: bool,
    parser: Option<RequestParser<RequestResponse>>,
    info: RequestInfo,
    callback: Option<Callback>,
}

impl<Query: Serialize, Body: Serialize, Callback, RequestResponse, RequestInfo: Default>
    RequestBuilder<Query, Body, Callback, RequestResponse, RequestInfo>
{
    pub fn new() -> Self {
        Self {
            method: None,
            path: String::default(),
            query: None,
            body: None,
            authorized: false,
            parser: None,
            info: RequestInfo::default(),
            callback: None,
        }
    }

    pub fn with_method(self, method: Method) -> Self {
        Self {
            method: Some(method),
            ..self
        }
    }

    pub fn with_path(self, path: &str) -> Self {
        Self {
            path: path.to_owned(),
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
        Self { info, ..self }
    }

    pub fn with_callback(self, callback: Option<Callback>) -> Self {
        Self {
            callback,
            ..self
        }
    }

    pub fn build(
        self,
        client: reqwest::Client,
        url: &str,
        jwt: &str,
    ) -> Result<
        (
            reqwest::RequestBuilder,
            RequestParser<RequestResponse>,
            RequestInfo,
            Option<Callback>,
        ),
        (),
    > {
        let method = self.method.ok_or(())?;
        let parser = self.parser.ok_or(())?;

        let builder = client.request(method, format!("{url}{}", self.path));
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
        Ok((builder, parser, self.info, self.callback))
    }
}
