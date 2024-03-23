use std::{
    cell::RefCell,
    rc::Rc,
    sync::{Arc, Mutex, RwLock},
};

use itertools::Itertools;

use super::{
    db_connector::{DbConnector, DbConnectorData},
    main_state::{RequestIdentifier, RequestType},
    request::RequestId,
    State,
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

type RequestChecker = Box<dyn Fn(&DbConnector) -> Option<RequestExecutor> + Send>;
type RequestExecutor = Box<dyn FnOnce(&mut State) + Send>;

/// Keeps count of requests that need to be executed
/// And how to populate State with the response
pub(super) struct RequestsHolder {
    requests: Arc<Mutex<Vec<RequestData>>>,

    // Checker check if request was completed, and returns an Executor
    // Executor populates State with request response
    // Needs to be separated because State needs to be populated 1 frame after response is received
    checkers: Arc<Mutex<Vec<RequestChecker>>>,
    executors: Arc<Mutex<Vec<RequestExecutor>>>,
}

impl RequestsHolder {
    fn new() -> Self {
        Self {
            requests: Arc::new(Mutex::new(Vec::new())),
            checkers: Arc::new(Mutex::new(Vec::new())),
            executors: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn get() -> &'static RwLock<Self> {
        use std::sync::OnceLock;

        static DATA: OnceLock<RwLock<RequestsHolder>> = OnceLock::new();
        DATA.get_or_init(|| RwLock::new(RequestsHolder::new()))
    }

    pub fn push(&self, request: RequestData) {
        self.requests.lock().unwrap().push(request);
    }
    pub fn take(&mut self) -> Vec<RequestData> {
        self.requests.lock().unwrap().drain(..).collect_vec()
    }

    pub(super) fn make_typical_request<T: 'static + RequestType + Send, F>(
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
                    let identifier: RequestIdentifier<T> =
                        RequestIdentifier::new(request_id, info.clone());
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
            self.checkers.lock().unwrap().push(checker);
        }

        identifier
    }

    pub fn update(&self, state: &mut State) {
        self.executors
            .lock()
            .unwrap()
            .drain(..)
            .for_each(|executor: RequestExecutor| {
                executor(state);
            });
        let mut new_executors = vec![];
        let checkers = { self.checkers.lock().unwrap().drain(..).collect_vec() };
        let mut remained_checkers = checkers
            .into_iter()
            .filter_map(|checker: RequestChecker| {
                if let Some(executor) = checker(&state.db_connector) {
                    new_executors.push(executor);
                    None
                } else {
                    Some(checker)
                }
            })
            .collect_vec();
        self.checkers.lock().unwrap().append(&mut remained_checkers);
        self.executors.lock().unwrap().append(&mut new_executors);
    }
}
