use std::{fmt::Debug, marker::PhantomData, sync::atomic::AtomicU16};

use serde::de::DeserializeOwned;

use super::State;

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
    type Info: 'static + Clone + Debug + Send = ();
}

pub trait StateRequestType
where
    Self: RequestType,
{
    fn push_to_state(response: Self::Response, info: Self::Info, state: &mut State);
    #[allow(unused_variables)]
    fn push_bad_to_state(response: Self::BadResponse, info: Self::Info, state: &mut State);
}

#[derive(Clone)]
pub struct RequestIdentifier<T: RequestType> {
    pub(super) id: RequestId,
    pub info: T::Info,
    _data: PhantomData<T>,
}

impl<T: RequestType> RequestIdentifier<T> {
    pub(super) fn new(request_id: RequestId, info: T::Info) -> Self {
        Self {
            id: request_id,
            info,
            _data: PhantomData::default(),
        }
    }
}
