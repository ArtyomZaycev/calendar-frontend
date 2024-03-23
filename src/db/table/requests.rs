use serde::Serialize;

use super::item::*;
use crate::db::request::RequestBuilder;
use crate::requests::{AppRequestInfo, AppRequestResponse};

pub trait DbTableLoadAll<T, RequestResponse = AppRequestResponse, RequestInfo = AppRequestInfo>
where
    T: DbTableItem,
    RequestResponse: Clone,
    RequestInfo: Clone + Default,
{
    type Args: Serialize;
    fn load_all(&self) -> RequestBuilder<Self::Args, (), RequestResponse, RequestInfo>;
}

pub trait DbTableLoad<T, RequestResponse = AppRequestResponse, RequestInfo = AppRequestInfo>
where
    T: DbTableItem,
    RequestResponse: Clone,
    RequestInfo: Clone + Default,
{
    type Args: Serialize;
    fn load_by_id_request(
        &self,
        id: TableId,
    ) -> RequestBuilder<Self::Args, (), RequestResponse, RequestInfo>;
}

pub trait DbTableInsert<T, RequestResponse = AppRequestResponse, RequestInfo = AppRequestInfo>
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
    ) -> RequestBuilder<Self::Args, Self::Body, RequestResponse, RequestInfo>;
}

pub trait DbTableUpdate<T, RequestResponse = AppRequestResponse, RequestInfo = AppRequestInfo>
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
    ) -> RequestBuilder<Self::Args, Self::Body, RequestResponse, RequestInfo>;
}

pub trait DbTableDelete<T, RequestResponse = AppRequestResponse, RequestInfo = AppRequestInfo>
where
    T: DbTableItem,
    RequestResponse: Clone,
    RequestInfo: Clone + Default,
{
    type Args: Serialize;
    fn delete_by_id_request(
        &self,
        id: TableId,
    ) -> RequestBuilder<Self::Args, (), RequestResponse, RequestInfo>;
}
