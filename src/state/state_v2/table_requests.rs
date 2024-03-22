use std::marker::PhantomData;

use serde::{de::DeserializeOwned, Deserialize};

use crate::tables::{DbTableItem, DbTableNewItem, DbTableUpdateItem};

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
    const INSERT_PATH: &'static str;
}
pub trait TableItemUpdate {
    const UPDATE_PATH: &'static str;
}
pub trait TableItemDelete {
    const DELETE_PATH: &'static str;
}

pub struct TableLoadByIdRequest<T: DbTableItem + TableItemLoadById> {
    _data: PhantomData<T>,
}
pub struct TableLoadAllRequest<T: DbTableItem + TableItemLoadAll> {
    _data: PhantomData<T>,
}
pub struct TableInsertRequest<N: DbTableNewItem + TableItemInsert> {
    _data2: PhantomData<N>,
}
pub struct TableUpdateRequest<N: DbTableUpdateItem + TableItemUpdate> {
    _data2: PhantomData<N>,
}
pub struct TableDeleteRequest<T: DbTableItem + TableItemDelete> {
    _data: PhantomData<T>,
}

impl<Item: DbTableItem + TableItemLoadById + DeserializeOwned> RequestType
    for TableLoadByIdRequest<Item>
where
    State: GetStateTable<Item>,
{
    const URL: &'static str = Item::LOAD_BY_ID_PATH;
    const IS_AUTHORIZED: bool = true;
    const METHOD: reqwest::Method = reqwest::Method::GET;
    type Query = Item::Id;
    type Response = Item;
    type Info = Item::Id;

    fn push_to_state(
        response: Self::Response,
        info: Self::Info,
        state: &mut super::main_state::State,
    ) {
        let table: &mut StateTable<Item> = state.get_table_mut();
        table.get_table_mut().push_one(response);
    }
}

impl<Item: DbTableItem + TableItemLoadAll + DeserializeOwned> RequestType
    for TableLoadAllRequest<Item>
where
    State: GetStateTable<Item>,
{
    const URL: &'static str = Item::LOAD_ALL_PATH;
    const IS_AUTHORIZED: bool = true;
    const METHOD: reqwest::Method = reqwest::Method::GET;
    type Query = ();
    type Response = Vec<Item>;
    type Info = ();

    fn push_to_state(
        response: Self::Response,
        info: Self::Info,
        state: &mut super::main_state::State,
    ) {
        let table: &mut StateTable<Item> = state.get_table_mut();
        table.get_table_mut().replace_all(response);
    }
}
/*
impl<N> RequestType for TableInsertRequest<T, N>
where
    N: DbTableNewItem + TableItemInsert + DeserializeOwned,
    State: GetStateTable<T>,
{
    const URL: &'static str = N::INSERT_PATH;
    const IS_AUTHORIZED: bool = true;
    const METHOD: reqwest::Method = reqwest::Method::POST;
    type Query = ();
    type Body = N;
    type Response = ();
    type Info = ();

    fn push_to_state(
        response: Self::Response,
        info: Self::Info,
        state: &mut super::main_state::State,
    ) {
        let table: &mut StateTable<T> = state.get_table_mut();
        todo!("Load inserted");
    }
}

impl<N> RequestType for TableUpdateRequest<T, N>
where
    N: DbTableUpdateItem + TableItemUpdate + DeserializeOwned,
    State: GetStateTable<T>,
{
    const URL: &'static str = N::UPDATE_PATH;
    const IS_AUTHORIZED: bool = true;
    const METHOD: reqwest::Method = reqwest::Method::PATCH;
    type Query = ();
    type Body = N;
    type Response = ();
    type Info = ();

    fn push_to_state(
        response: Self::Response,
        info: Self::Info,
        state: &mut super::main_state::State,
    ) {
        let table: &mut StateTable<T> = state.get_table_mut();
        todo!("Load updated (or just replace the loaded one)");
    }
}
 */
impl<Item: DbTableItem + TableItemDelete + DeserializeOwned> RequestType
    for TableDeleteRequest<Item>
where
    State: GetStateTable<Item>,
{
    const URL: &'static str = Item::DELETE_PATH;
    const IS_AUTHORIZED: bool = true;
    const METHOD: reqwest::Method = reqwest::Method::DELETE;
    type Query = Item::Id;
    type Response = ();
    type Info = ();

    fn push_to_state(
        response: Self::Response,
        info: Self::Info,
        state: &mut super::main_state::State,
    ) {
        let table: &mut StateTable<Item> = state.get_table_mut();
        todo!("Reload table (or just remove deleted)");
    }
}
