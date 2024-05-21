use super::{
    state_updater::{StateChecker, StateExecutor, StateUpdater},
    State,
};

pub use crate::db::request::RequestType;
use crate::db::{
    db_connector::DbConnectorData,
    request::{make_request_custom, RequestIdentifier},
};

pub trait StateRequestType
where
    Self: RequestType,
{
    fn push_to_state(response: Self::Response, info: Self::Info, state: &mut State);
    #[allow(unused_variables)]
    fn push_bad_to_state(response: Self::BadResponse, info: Self::Info, state: &mut State);
}

pub fn make_state_request<T, F>(info: T::Info, make_request: F) -> RequestIdentifier<T>
where
    T: StateRequestType,
    F: FnOnce(&DbConnectorData) -> reqwest::RequestBuilder,
{
    let rinfo = info.clone();
    let make_checker = |request_id| {
        let checker: StateChecker = Box::new(move |state| {
            if state.db_connector.is_request_completed(request_id) {
                state
                    .db_connector
                    .convert_response::<T::Response, T::BadResponse>(request_id);

                let info = info.clone();
                let identifier: RequestIdentifier<T> =
                    RequestIdentifier::new(request_id, info.clone());
                let executor: StateExecutor = Box::new(move |state: &mut State| {
                    let response = state.take_response(&identifier);
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
        checker
    };
    let identifier = make_request_custom(rinfo, make_request);
    StateUpdater::get().push_checker(make_checker(identifier.id));
    identifier
}
