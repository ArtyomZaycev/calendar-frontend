use std::marker::PhantomData;

use calendar_lib::api::utils::EmptyResponse;
use serde::de::DeserializeOwned;

use crate::tables::{DbTableItem, DbTableNewItem, DbTableUpdateItem, TableId};

use super::{
    main_state::{GetStateTable, RequestType, State},
    state_table::StateTable,
};

// Should be moved to lib

pub trait TableItemLoadById {
    const LOAD_BY_ID_PATH: &'static str;
}
pub trait TableItemLoadAll {
    const LOAD_ALL_PATH: &'static str;
}
pub trait TableItemInsert {
    type NewItem: DbTableNewItem + DeserializeOwned;
    const INSERT_PATH: &'static str;
}
pub trait TableItemUpdate {
    type UpdItem: DbTableUpdateItem + DeserializeOwned;
    const UPDATE_PATH: &'static str;
}
pub trait TableItemDelete {
    const DELETE_PATH: &'static str;
}

#[derive(Clone, Copy)]
pub struct TableLoadByIdRequest<T: DbTableItem + TableItemLoadById> {
    _data: PhantomData<T>,
}
#[derive(Clone, Copy)]
pub struct TableLoadAllRequest<T: DbTableItem + TableItemLoadAll> {
    _data: PhantomData<T>,
}
#[derive(Clone, Copy)]
pub struct TableInsertRequest<T: DbTableItem + TableItemInsert> {
    _data: PhantomData<T>,
}
#[derive(Clone, Copy)]
pub struct TableUpdateRequest<T: DbTableItem + TableItemUpdate> {
    _data: PhantomData<T>,
}
#[derive(Clone, Copy)]
pub struct TableDeleteRequest<T: DbTableItem + TableItemDelete> {
    _data: PhantomData<T>,
}

impl<T> RequestType for TableLoadByIdRequest<T>
where
    T: 'static + DbTableItem + TableItemLoadById + DeserializeOwned,
    State: GetStateTable<T>,
{
    const URL: &'static str = T::LOAD_BY_ID_PATH;
    const IS_AUTHORIZED: bool = true;
    const METHOD: reqwest::Method = reqwest::Method::GET;
    type Query = TableId;
    type Response = T;
    type Info = TableId;

    fn push_to_state(response: Self::Response, info: Self::Info, state: &mut State) {
        let table: &mut StateTable<T> = state.get_table_mut();
        table.get_table_mut().push_one(response);
    }
}

impl<T> RequestType for TableLoadAllRequest<T>
where
    T: 'static + DbTableItem + TableItemLoadAll + DeserializeOwned,
    State: GetStateTable<T>,
{
    const URL: &'static str = T::LOAD_ALL_PATH;
    const IS_AUTHORIZED: bool = true;
    const METHOD: reqwest::Method = reqwest::Method::GET;
    type Query = ();
    type Response = Vec<T>;
    type Info = ();

    fn push_to_state(response: Self::Response, info: Self::Info, state: &mut State) {
        let table: &mut StateTable<T> = state.get_table_mut();
        table.get_table_mut().replace_all(response);
    }
}

impl<T> RequestType for TableInsertRequest<T>
where
    T: 'static + Send + DbTableItem + TableItemInsert + TableItemLoadAll + DeserializeOwned,
    T::NewItem: DeserializeOwned,
    State: GetStateTable<T>,
{
    const URL: &'static str = T::INSERT_PATH;
    const IS_AUTHORIZED: bool = true;
    const METHOD: reqwest::Method = reqwest::Method::POST;
    type Query = ();
    type Body = T::NewItem;
    type Response = EmptyResponse;
    type Info = ();

    fn push_to_state(response: Self::Response, info: Self::Info, state: &mut State) {
        let table: &mut StateTable<T> = state.get_table_mut();
        table.load_all();
    }
}

impl<T> RequestType for TableUpdateRequest<T>
where
    T: 'static + Send + DbTableItem + TableItemUpdate + TableItemLoadById + DeserializeOwned,
    T::UpdItem: DeserializeOwned,
    State: GetStateTable<T>,
{
    const URL: &'static str = T::UPDATE_PATH;
    const IS_AUTHORIZED: bool = true;
    const METHOD: reqwest::Method = reqwest::Method::PATCH;
    type Query = ();
    type Body = T::UpdItem;
    type Response = EmptyResponse;
    type Info = TableId;

    fn push_to_state(response: Self::Response, info: Self::Info, state: &mut State) {
        let table: &mut StateTable<T> = state.get_table_mut();
        table.load_by_id(info);
    }
}

impl<T> RequestType for TableDeleteRequest<T>
where
    T: 'static + Send + DbTableItem + TableItemDelete + DeserializeOwned + TableItemLoadAll,
    State: GetStateTable<T>,
{
    const URL: &'static str = T::DELETE_PATH;
    const IS_AUTHORIZED: bool = true;
    const METHOD: reqwest::Method = reqwest::Method::DELETE;
    type Query = TableId;
    type Response = EmptyResponse;
    type Info = TableId;

    fn push_to_state(response: Self::Response, info: Self::Info, state: &mut State) {
        let table: &mut StateTable<T> = state.get_table_mut();
        table.load_all();
    }
}
