use serde::Serialize;

use super::item::*;
use crate::db::request::RequestBuilder;
use crate::requests::{AppRequestInfo, AppRequestResponse};
use crate::state::state_requests::StateCallback;

pub trait DbTableLoadAll<T, RequestResponse = AppRequestResponse, RequestInfo = AppRequestInfo, Callback = StateCallback>
where
    T: DbTableItem,
    RequestResponse: Clone,
    RequestInfo: Clone + Default,
{
    type Args: Serialize;
    fn load_all(&self) -> RequestBuilder<Self::Args, (), Callback, RequestResponse, RequestInfo>;
}

pub trait DbTableLoad<T, RequestResponse = AppRequestResponse, RequestInfo = AppRequestInfo, Callback = StateCallback>
where
    T: DbTableItem,
    RequestResponse: Clone,
    RequestInfo: Clone + Default,
{
    type Args: Serialize;
    fn load_by_id_request(
        &self,
        id: T::Id,
    ) -> RequestBuilder<Self::Args, (), Callback, RequestResponse, RequestInfo>;
}

pub trait DbTableInsert<T, RequestResponse = AppRequestResponse, RequestInfo = AppRequestInfo, Callback = StateCallback>
where
    T: DbTableNewItem,
    RequestResponse: Clone,
    RequestInfo: Clone + Default,
{
    type Args: Serialize;
    type Body: Serialize;
    fn insert_request(
        &self,
        new_item: T,
    ) -> RequestBuilder<Self::Args, Self::Body, Callback, RequestResponse, RequestInfo>;
}

pub trait DbTableUpdate<T, RequestResponse = AppRequestResponse, RequestInfo = AppRequestInfo, Callback = StateCallback>
where
    T: DbTableUpdateItem,
    RequestResponse: Clone,
    RequestInfo: Clone + Default,
{
    type Args: Serialize;
    type Body: Serialize;
    fn update_request(
        &self,
        update_item: T,
    ) -> RequestBuilder<Self::Args, Self::Body, Callback, RequestResponse, RequestInfo>;
}

pub trait DbTableDelete<T, RequestResponse = AppRequestResponse, RequestInfo = AppRequestInfo, Callback = StateCallback>
where
    T: DbTableItem,
    RequestResponse: Clone,
    RequestInfo: Clone + Default,
{
    type Args: Serialize;
    fn delete_by_id_request(
        &self,
        id: T::Id,
    ) -> RequestBuilder<Self::Args, (), Callback, RequestResponse, RequestInfo>;
}
