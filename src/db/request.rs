
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