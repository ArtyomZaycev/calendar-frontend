use serde::Serialize;

use super::item::*;
use crate::db::request::RequestBuilder;
use crate::{
    db::request_parser::FromResponse,
    requests::{AppRequestInfo, AppRequestResponse, AppRequestResponseInfo},
};

pub trait DbTableLoadAll<
    T: DbTableItem,
    RequestResponse = AppRequestResponse,
    RequestInfo = AppRequestInfo,
    RequestResponseInfo = AppRequestResponseInfo,
> where
    RequestResponse: Clone,
    RequestInfo: Clone,
    RequestResponseInfo: Clone + FromResponse<RequestResponse>,
{
    type Args: Serialize;
    fn load_all() -> RequestBuilder<Self::Args, ()>;
}

pub trait DbTableLoad<
    T: DbTableItem,
    RequestResponse = AppRequestResponse,
    RequestInfo = AppRequestInfo,
    RequestResponseInfo = AppRequestResponseInfo,
> where
    RequestResponse: Clone,
    RequestInfo: Clone,
    RequestResponseInfo: Clone + FromResponse<RequestResponse>,
{
    type Args: Serialize;
    fn load_by_id(id: T::Id) -> RequestBuilder<Self::Args, ()>;
}

pub trait DbTableInsert<
    T: DbTableNewItem,
    RequestResponse = AppRequestResponse,
    RequestInfo = AppRequestInfo,
    RequestResponseInfo = AppRequestResponseInfo,
> where
    RequestResponse: Clone,
    RequestInfo: Clone,
    RequestResponseInfo: Clone + FromResponse<RequestResponse>,
{
    type Args: Serialize;
    type Body: Serialize;
    fn insert(new_item: T,
    ) -> RequestBuilder<Self::Args, Self::Body>;
}

pub trait DbTableUpdate<
    T: DbTableUpdateItem,
    RequestResponse = AppRequestResponse,
    RequestInfo = AppRequestInfo,
    RequestResponseInfo = AppRequestResponseInfo,
> where
    RequestResponse: Clone,
    RequestInfo: Clone,
    RequestResponseInfo: Clone + FromResponse<RequestResponse>,
{
    type Args: Serialize;
    type Body: Serialize;
    fn update(update_item: T) -> RequestBuilder<Self::Args, Self::Body>;
}

pub trait DbTableDelete<
    T: DbTableItem,
    RequestResponse = AppRequestResponse,
    RequestInfo = AppRequestInfo,
    RequestResponseInfo = AppRequestResponseInfo,
> where
    RequestResponse: Clone,
    RequestInfo: Clone,
    RequestResponseInfo: Clone + FromResponse<RequestResponse>,
{
    type Args: Serialize;
    fn delete_by_id(id: T::Id,) -> RequestBuilder<Self::Args, ()>;
}
