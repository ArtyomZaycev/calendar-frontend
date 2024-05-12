use std::{fmt::Debug, marker::PhantomData, sync::atomic::AtomicU16};

use serde::de::DeserializeOwned;

use super::{
    db_connector::DbConnectorData,
    requests_holder::{RequestData, RequestsHolder},
};

pub type RequestId = u16;
pub type RequestIdAtomic = AtomicU16;

// TODO: move to lib
pub trait RequestType
where
    Self: 'static + Send,
{
    const URL: &'static str;
    const IS_AUTHORIZED: bool;
    const METHOD: reqwest::Method;

    type Query;
    type Body = ();
    type Response: 'static + DeserializeOwned;
    type BadResponse: 'static + DeserializeOwned = ();

    /// e.g. update request item.id
    type Info: 'static + Clone + Debug + Send;
}

#[derive(Clone)]
pub struct RequestIdentifier<T: RequestType> {
    pub id: RequestId,
    pub info: T::Info,
    _data: PhantomData<T>,
}

impl<T: RequestType> RequestIdentifier<T> {
    pub fn new(request_id: RequestId, info: T::Info) -> Self {
        Self {
            id: request_id,
            info,
            _data: PhantomData::default(),
        }
    }
}

pub fn make_request_custom<T, F>(
    info: T::Info,
    make_request: F,
) -> RequestIdentifier<T>
where
    T: RequestType,
    F: FnOnce(&DbConnectorData) -> reqwest::RequestBuilder,
{
    let connector = DbConnectorData::get();
    let request_id = connector.next_request_id();
    let request = make_request(connector);
    RequestsHolder::get().push(RequestData::new(request_id, request.build().unwrap()));
    RequestIdentifier::new(request_id, info.clone())
}
