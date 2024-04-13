use std::marker::PhantomData;

use calendar_lib::api::utils::{EmptyResponse, LoadByIdBadRequestResponse};

use crate::tables::{DbTableItem, DbTableNewItem, DbTableUpdateItem, TableId};

use super::{
    main_state::State,
    request::{RequestType, StateRequestType},
};

pub trait TableItemLoadById
where
    Self: DbTableItem,
{
    const LOAD_BY_ID_PATH: &'static str;

    fn push_from_load_by_id(state: &mut State, id: TableId, item: Self);
    fn push_bad_from_load_by_id(
        state: &mut State,
        id: TableId,
        response: LoadByIdBadRequestResponse,
    );
}
pub trait TableItemLoadAll
where
    Self: DbTableItem,
{
    const LOAD_ALL_PATH: &'static str;

    fn push_from_load_all(state: &mut State, items: Vec<Self>);
    fn push_bad_from_load_all(state: &mut State);
}
pub trait TableItemInsert
where
    Self: DbTableItem,
{
    type NewItem: DbTableNewItem;
    const INSERT_PATH: &'static str;

    fn push_from_insert(state: &mut State);
    fn push_bad_from_insert(state: &mut State);
}
pub trait TableItemUpdate
where
    Self: DbTableItem,
{
    type UpdItem: DbTableUpdateItem;
    const UPDATE_PATH: &'static str;

    fn push_from_update(state: &mut State, id: TableId);
    fn push_bad_from_update(state: &mut State, id: TableId);
}
pub trait TableItemDelete
where
    Self: DbTableItem,
{
    const DELETE_PATH: &'static str;

    fn push_from_delete(state: &mut State, id: TableId);
    fn push_bad_from_delete(state: &mut State, id: TableId);
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
    type Info = TableId;
}
#[allow(unused_variables)]
impl<T: TableItemLoadById> StateRequestType for TableLoadByIdRequest<T> {
    fn push_to_state(response: Self::Response, info: Self::Info, state: &mut State) {
        T::push_from_load_by_id(state, info, response);
    }

    fn push_bad_to_state(response: Self::BadResponse, info: Self::Info, state: &mut State) {
        T::push_bad_from_load_by_id(state, info, response);
    }
}

impl<T: TableItemLoadAll> RequestType for TableLoadAllRequest<T> {
    const URL: &'static str = T::LOAD_ALL_PATH;
    const IS_AUTHORIZED: bool = true;
    const METHOD: reqwest::Method = reqwest::Method::GET;
    type Query = ();
    type Response = Vec<T>;
    type Info = ();
}
#[allow(unused_variables)]
impl<T: TableItemLoadAll> StateRequestType for TableLoadAllRequest<T> {
    fn push_to_state(response: Self::Response, info: Self::Info, state: &mut State) {
        T::push_from_load_all(state, response);
    }

    fn push_bad_to_state(response: Self::BadResponse, info: Self::Info, state: &mut State) {
        T::push_bad_from_load_all(state);
    }
}

#[allow(unused_variables)]
impl<T: TableItemInsert> RequestType for TableInsertRequest<T> {
    const URL: &'static str = T::INSERT_PATH;
    const IS_AUTHORIZED: bool = true;
    const METHOD: reqwest::Method = reqwest::Method::POST;
    type Query = ();
    type Body = T::NewItem;
    type Response = EmptyResponse;
    type Info = ();
}
#[allow(unused_variables)]
impl<T: TableItemInsert> StateRequestType for TableInsertRequest<T> {
    fn push_to_state(response: Self::Response, info: Self::Info, state: &mut State) {
        T::push_from_insert(state);
    }

    fn push_bad_to_state(response: Self::BadResponse, info: Self::Info, state: &mut State) {
        T::push_bad_from_insert(state);
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
    type Info = TableId;
}
#[allow(unused_variables)]
impl<T: TableItemUpdate> StateRequestType for TableUpdateRequest<T> {
    fn push_to_state(response: Self::Response, info: Self::Info, state: &mut State) {
        T::push_from_update(state, info);
    }

    fn push_bad_to_state(_response: Self::BadResponse, info: Self::Info, state: &mut State) {
        T::push_bad_from_update(state, info);
    }
}

#[allow(unused_variables)]
impl<T: TableItemDelete> RequestType for TableDeleteRequest<T> {
    const URL: &'static str = T::DELETE_PATH;
    const IS_AUTHORIZED: bool = true;
    const METHOD: reqwest::Method = reqwest::Method::DELETE;
    type Query = TableId;
    type Response = EmptyResponse;
    type Info = TableId;
}
#[allow(unused_variables)]
impl<T: TableItemDelete> StateRequestType for TableDeleteRequest<T> {
    fn push_to_state(response: Self::Response, info: Self::Info, state: &mut State) {
        T::push_from_delete(state, info)
    }

    fn push_bad_to_state(response: Self::BadResponse, info: Self::Info, state: &mut State) {
        T::push_bad_from_delete(state, info);
    }
}
