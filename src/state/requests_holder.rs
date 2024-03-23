use std::{cell::RefCell, rc::Rc};

use super::{
    db_connector::{DbConnector, DbConnectorData},
    main_state::{RequestIdentifier, RequestType},
    request::RequestId, State,
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

type RequestChecker = Box<dyn Fn(&DbConnector) -> Option<RequestExecutor>>;
type RequestExecutor = Box<dyn FnOnce(&mut State)>;

// Rename?
/// Holds requests that should be sent at the end of the frame
pub(super) struct RequestsHolder {
    requests: Rc<RefCell<Vec<RequestData>>>,

    // Checker check if request was completed, and returns an Executor
    // Executor populates State with request response
    // Needs to be separated because State needs to be populated 1 frame after response is received
    checkers: Rc<RefCell<Vec<RequestChecker>>>,
    executors: Rc<RefCell<Vec<RequestExecutor>>>,
}

impl RequestsHolder {
    pub fn new() -> Self {
        Self {
            requests: Rc::new(RefCell::new(Vec::new())),
            checkers: Rc::new(RefCell::new(Vec::new())),
            executors: Rc::new(RefCell::new(Vec::new())),
        }
    }

    pub fn push(&self, request: RequestData) {
        self.requests.borrow_mut().push(request);
    }
    pub fn take(&mut self) -> Vec<RequestData> {
        self.requests.replace(Vec::new())
    }

    pub(super) fn make_typical_request<T: 'static + RequestType, F>(
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
        let identifier: RequestIdentifier<T> = RequestIdentifier::new(request_id, info.clone());

        {
            // TODO: Everything about this can be improved
            let checker: RequestChecker = Box::new(move |connector| {
                if connector.is_request_completed(request_id) {
                    let identifier: RequestIdentifier<T> = RequestIdentifier::new(request_id, info.clone());
                    let info = info.clone();
                    let executor: RequestExecutor = Box::new(move |state: &mut State| {
                        let response = state.take_response(identifier);
                        if let Some(response) = response {
                            match response {
                                Ok(response) => T::push_to_state(*response, info, state),
                                Err(response) => T::push_bad_to_state(*response, info, state),
                            }
                        }
                    });
                    Some(executor)
                } else {
                    None
                }
            });
            self.checkers.borrow_mut().push(checker);
        }

        identifier
    }
    
    pub fn update(&mut self, state: &mut State) {
        self.executors.take().into_iter().for_each(|executor: RequestExecutor| {
            executor(state);
        });
    }
}
