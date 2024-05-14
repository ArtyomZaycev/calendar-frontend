use calendar_lib::api::utils::*;
use serde::de::DeserializeOwned;
use std::{fmt::Debug, marker::PhantomData};

use crate::tables::{DbTableItem, DbTableNewItem, DbTableUpdateItem};

use super::{
    main_state::State,
    request::{RequestType, StateRequestType},
};

#[derive(Debug, Clone)]
pub struct StateRequestInfo<T>
where
    T: 'static + Clone + Debug + Send,
{
    pub user_id: TableId,
    pub info: T,
}

impl<T> StateRequestInfo<T>
where
    T: 'static + Clone + Debug + Send,
{
    pub fn new(user_id: TableId, info: T) -> Self {
        Self { user_id, info }
    }
    pub fn new_default(user_id: TableId) -> Self
    where
        T: Default,
    {
        Self {
            user_id,
            info: T::default(),
        }
    }
}

pub trait TableItemLoadById
where
    Self: DbTableItem,
{
    const LOAD_BY_ID_PATH: &'static str;

    fn push_from_load_by_id(state: &mut State, user_id: TableId, id: TableId, item: Self);
    fn push_bad_from_load_by_id(
        state: &mut State,
        user_id: TableId,
        id: TableId,
        response: LoadByIdBadRequestResponse,
    );
}
pub trait TableItemLoadAll
where
    Self: DbTableItem,
{
    const LOAD_ALL_PATH: &'static str;

    fn push_from_load_all(state: &mut State, user_id: TableId, items: Vec<Self>);
    fn push_bad_from_load_all(state: &mut State, user_id: TableId);
}
pub trait TableItemInsert
where
    Self: DbTableItem,
{
    type NewItem: DbTableNewItem;
    const INSERT_PATH: &'static str;

    type BadResponse: 'static + DeserializeOwned = ();

    fn push_from_insert(state: &mut State, user_id: TableId);
    fn push_bad_from_insert(state: &mut State, user_id: TableId, response: Self::BadResponse);
}
pub trait TableItemUpdate
where
    Self: DbTableItem,
{
    type UpdItem: DbTableUpdateItem;
    const UPDATE_PATH: &'static str;

    type BadResponse: 'static + DeserializeOwned = UpdateBadRequestResponse;

    fn push_from_update(state: &mut State, user_id: TableId, id: TableId);
    fn push_bad_from_update(
        state: &mut State,
        user_id: TableId,
        id: TableId,
        response: Self::BadResponse,
    );
}
pub trait TableItemDelete
where
    Self: DbTableItem,
{
    const DELETE_PATH: &'static str;

    fn push_from_delete(state: &mut State, user_id: TableId, id: TableId);
    fn push_bad_from_delete(
        state: &mut State,
        user_id: TableId,
        id: TableId,
        response: DeleteBadRequestResponse,
    );
}

#[derive(Clone, Copy)]
pub struct TableLoadByIdRequest<T: TableItemLoadById> {
    _data: PhantomData<T>,
}
#[derive(Clone, Copy)]
pub struct TableLoadAllRequest<T: TableItemLoadAll> {
    _data: PhantomData<T>,
}
#[derive(Clone, Copy)]
pub struct TableInsertRequest<T: TableItemInsert> {
    _data: PhantomData<T>,
}
#[derive(Clone, Copy)]
pub struct TableUpdateRequest<T: TableItemUpdate> {
    _data: PhantomData<T>,
}
#[derive(Clone, Copy)]
pub struct TableDeleteRequest<T: TableItemDelete> {
    _data: PhantomData<T>,
}

impl<T: TableItemLoadById> RequestType for TableLoadByIdRequest<T> {
    const URL: &'static str = T::LOAD_BY_ID_PATH;
    const IS_AUTHORIZED: bool = true;
    const METHOD: reqwest::Method = reqwest::Method::GET;
    type Query = TableId;
    type Response = T;
    type BadResponse = LoadByIdBadRequestResponse;
    type Info = StateRequestInfo<TableId>;
}
#[allow(unused_variables)]
impl<T: TableItemLoadById> StateRequestType for TableLoadByIdRequest<T> {
    fn push_to_state(response: Self::Response, info: Self::Info, state: &mut State) {
        T::push_from_load_by_id(state, info.user_id, info.info, response);
    }

    fn push_bad_to_state(response: Self::BadResponse, info: Self::Info, state: &mut State) {
        T::push_bad_from_load_by_id(state, info.user_id, info.info, response);
    }
}

impl<T: TableItemLoadAll> RequestType for TableLoadAllRequest<T> {
    const URL: &'static str = T::LOAD_ALL_PATH;
    const IS_AUTHORIZED: bool = true;
    const METHOD: reqwest::Method = reqwest::Method::GET;
    type Query = LoadArrayQuery;
    type Response = Vec<T>;
    type Info = StateRequestInfo<()>;
}
#[allow(unused_variables)]
impl<T: TableItemLoadAll> StateRequestType for TableLoadAllRequest<T> {
    fn push_to_state(response: Self::Response, info: Self::Info, state: &mut State) {
        T::push_from_load_all(state, info.user_id, response);
    }

    fn push_bad_to_state(response: Self::BadResponse, info: Self::Info, state: &mut State) {
        T::push_bad_from_load_all(state, info.user_id);
    }
}

impl<T: TableItemInsert> RequestType for TableInsertRequest<T> {
    const URL: &'static str = T::INSERT_PATH;
    const IS_AUTHORIZED: bool = true;
    const METHOD: reqwest::Method = reqwest::Method::POST;
    type Query = ();
    type Body = T::NewItem;
    type Response = EmptyResponse;
    type BadResponse = T::BadResponse;
    type Info = StateRequestInfo<()>;
}
#[allow(unused_variables)]
impl<T: TableItemInsert> StateRequestType for TableInsertRequest<T> {
    fn push_to_state(response: Self::Response, info: Self::Info, state: &mut State) {
        T::push_from_insert(state, info.user_id);
    }

    fn push_bad_to_state(response: Self::BadResponse, info: Self::Info, state: &mut State) {
        T::push_bad_from_insert(state, info.user_id, response);
    }
}

#[allow(unused_variables)]
impl<T: TableItemUpdate> RequestType for TableUpdateRequest<T> {
    const URL: &'static str = T::UPDATE_PATH;
    const IS_AUTHORIZED: bool = true;
    const METHOD: reqwest::Method = reqwest::Method::PATCH;
    type Query = ();
    type Body = T::UpdItem;
    type Response = EmptyResponse;
    type BadResponse = T::BadResponse;
    type Info = StateRequestInfo<TableId>;
}
#[allow(unused_variables)]
impl<T: TableItemUpdate> StateRequestType for TableUpdateRequest<T> {
    fn push_to_state(response: Self::Response, info: Self::Info, state: &mut State) {
        T::push_from_update(state, info.user_id, info.info);
    }

    fn push_bad_to_state(response: Self::BadResponse, info: Self::Info, state: &mut State) {
        T::push_bad_from_update(state, info.user_id, info.info, response);
    }
}

#[allow(unused_variables)]
impl<T: TableItemDelete> RequestType for TableDeleteRequest<T> {
    const URL: &'static str = T::DELETE_PATH;
    const IS_AUTHORIZED: bool = true;
    const METHOD: reqwest::Method = reqwest::Method::DELETE;
    type Query = TableId;
    type Response = EmptyResponse;
    type BadResponse = DeleteBadRequestResponse;
    type Info = StateRequestInfo<TableId>;
}
#[allow(unused_variables)]
impl<T: TableItemDelete> StateRequestType for TableDeleteRequest<T> {
    fn push_to_state(response: Self::Response, info: Self::Info, state: &mut State) {
        T::push_from_delete(state, info.user_id, info.info)
    }

    fn push_bad_to_state(response: Self::BadResponse, info: Self::Info, state: &mut State) {
        T::push_bad_from_delete(state, info.user_id, info.info, response);
    }
}
