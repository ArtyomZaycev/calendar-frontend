use std::{marker::PhantomData, ops::Deref, sync::atomic::AtomicU16};

use super::main_state::RequestType;

pub type RequestId = u16;
pub type RequestIdAtomic = AtomicU16;

#[derive(Clone)]
pub struct RequestIdentifier<T: RequestType>
where
    T::Info: Clone,
{
    pub(super) id: RequestId,
    pub(super) info: T::Info,
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
