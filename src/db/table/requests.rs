use reqwest::RequestBuilder;

use super::item::*;
use crate::db::{connector::DbConnector, request::RequestId};
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
    fn load_all(
        connector: &mut DbConnector<RequestResponse, RequestInfo, RequestResponseInfo>,
        request: RequestBuilder,
    ) -> RequestId;
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
    fn load_by_id(
        connector: &mut DbConnector<RequestResponse, RequestInfo, RequestResponseInfo>,
        request: RequestBuilder,
        id: T::Id,
    ) -> RequestId;
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
    fn update(
        connector: &mut DbConnector<RequestResponse, RequestInfo, RequestResponseInfo>,
        request: RequestBuilder,
        update_item: T,
    ) -> RequestId;
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
    fn insert(
        connector: &mut DbConnector<RequestResponse, RequestInfo, RequestResponseInfo>,
        request: RequestBuilder,
        new_item: T,
    ) -> RequestId;
}
