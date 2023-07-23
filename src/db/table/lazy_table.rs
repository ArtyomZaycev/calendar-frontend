use std::collections::HashMap;
use std::hash::Hash;

use crate::{
    db::{request::RequestBuilder, request_parser::RequestParser},
    requests::*,
    tables::*,
};
use serde::Serialize;

/// 
pub struct LazyTable<T: DbTableItem> where T::Id: Hash {
    /// All ids
    ids: Vec<T::Id>,
    items: HashMap<T::Id, Option<T>>,
}

impl<T: DbTableItem> Default for LazyTable<T> where T::Id: Hash {
    fn default() -> Self {
        Self {
            ids: Default::default(),
            items: Default::default(),
        }
    }
}

impl<T: DbTableItem> LazyTable<T> where T::Id: Hash {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_items(items: Vec<T>) -> Self {
        Self { 
            ids: items.iter().map(|i| i.get_id()).collect(),
            items: HashMap::from_iter(items.into_iter().map(|i| (i.get_id(), Some(i))))
        }
    }

    pub fn clear(&mut self) {
        self.ids.clear();
        self.items.clear();
        self.items.shrink_to_fit();
    }
}

impl<T: DbTableItem> LazyTable<T> where T::Id: Hash {
    pub fn reload_ids(&mut self)

    /// True if this is a new item, false if it was updated
    pub fn push_one(&mut self, new_item: T) -> bool {
        match self
            .items
            .iter_mut()
            .find(|i| i.get_id() == new_item.get_id())
        {
            Some(i) => {
                *i = new_item;
                false
            }
            None => {
                self.items.push(new_item);
                true
            }
        }
    }
    /// Returns removed item (if found)
    pub fn remove_one(&mut self, id: T::Id) -> Option<T> {
        self.items
            .iter()
            .position(|i| i.get_id() == id)
            .map(|ind| self.items.remove(ind))
    }
}

impl<T: DbTableItem> DbTable<T> for Table<T> {
    fn get(&self) -> &Vec<T> {
        &self.items
    }

    fn get_mut(&mut self) -> &mut Vec<T> {
        &mut self.items
    }
}

/*
pub trait TableLoadById<T, RequestResponse = AppRequestResponse, RequestInfo = AppRequestInfo>
where
    T: DbTableItem,
    RequestResponse: Clone,
    RequestInfo: Clone,
{
    type Args: Serialize;

    //  constants cannot refer to statics
    //const METHOD: reqwest::Method;
    //const PATH: &'static str;

    fn get_method() -> reqwest::Method;
    fn get_path() -> &'static str;
    fn make_parser() -> RequestParser<RequestResponse>;
    fn make_args(id: T::Id) -> Self::Args;
    fn make_info(id: T::Id) -> RequestInfo;
}

impl<T, RequestResponse, RequestInfo> DbTableLoad<T, RequestResponse, RequestInfo> for Table<T>
where
    T: DbTableItem,
    Table<T>: TableLoadById<T, RequestResponse, RequestInfo>,
    RequestResponse: Clone,
    RequestInfo: Clone + Default,
{
    type Args = <Self as TableLoadById<T, RequestResponse, RequestInfo>>::Args;

    fn load_by_id_request(
        &self,
        id: <T as DbTableItem>::Id,
    ) -> RequestBuilder<Self::Args, (), RequestResponse, RequestInfo> {
        RequestBuilder::new()
            .authorized()
            .with_method(Self::get_method())
            .with_path(Self::get_path())
            .with_query(Self::make_args(id))
            .with_info(Self::make_info(id))
            .with_parser(Self::make_parser())
    }
}

pub trait TableLoadAll<T, RequestResponse = AppRequestResponse, RequestInfo = AppRequestInfo>
where
    T: DbTableItem,
    RequestResponse: Clone,
    RequestInfo: Clone,
{
    type Args: Serialize;

    fn get_method() -> reqwest::Method;
    fn get_path() -> &'static str;
    fn make_parser() -> RequestParser<RequestResponse>;
    fn make_args() -> Self::Args;
    fn make_info() -> RequestInfo;
}

impl<T, RequestResponse, RequestInfo> DbTableLoadAll<T, RequestResponse, RequestInfo> for Table<T>
where
    T: DbTableItem,
    Table<T>: TableLoadAll<T, RequestResponse, RequestInfo>,
    RequestResponse: Clone,
    RequestInfo: Clone + Default,
{
    type Args = <Self as TableLoadAll<T, RequestResponse, RequestInfo>>::Args;

    fn load_all(&self) -> RequestBuilder<Self::Args, (), RequestResponse, RequestInfo> {
        RequestBuilder::new()
            .authorized()
            .with_method(Self::get_method())
            .with_path(Self::get_path())
            .with_query(Self::make_args())
            .with_info(Self::make_info())
            .with_parser(Self::make_parser())
    }
}

pub trait TableInsert<N, RequestResponse = AppRequestResponse, RequestInfo = AppRequestInfo>
where
    N: DbTableNewItem,
    RequestResponse: Clone,
    RequestInfo: Clone,
{
    type Args: Serialize;
    type Body: Serialize;

    fn get_method() -> reqwest::Method;
    fn get_path() -> &'static str;
    fn make_parser() -> RequestParser<RequestResponse>;
    fn make_args(new_item: &N) -> Self::Args;
    fn make_info(new_item: &N) -> RequestInfo;
    fn make_body(new_item: N) -> Self::Body;
}

impl<T, N, RequestResponse, RequestInfo> DbTableInsert<N, RequestResponse, RequestInfo> for Table<T>
where
    T: DbTableItem,
    N: DbTableNewItem,
    Table<T>: TableInsert<N, RequestResponse, RequestInfo>,
    RequestResponse: Clone,
    RequestInfo: Clone + Default,
{
    type Args = <Self as TableInsert<N, RequestResponse, RequestInfo>>::Args;
    type Body = <Self as TableInsert<N, RequestResponse, RequestInfo>>::Body;

    fn insert_request(
        &self,
        new_item: N,
    ) -> RequestBuilder<Self::Args, Self::Body, RequestResponse, RequestInfo> {
        RequestBuilder::new()
            .authorized()
            .with_method(Self::get_method())
            .with_path(Self::get_path())
            .with_query(Self::make_args(&new_item))
            .with_info(Self::make_info(&new_item))
            .with_body(Self::make_body(new_item))
            .with_parser(Self::make_parser())
    }
}

pub trait TableUpdate<U, RequestResponse = AppRequestResponse, RequestInfo = AppRequestInfo>
where
    U: DbTableUpdateItem,
    RequestResponse: Clone,
    RequestInfo: Clone,
{
    type Args: Serialize;
    type Body: Serialize;

    fn get_method() -> reqwest::Method;
    fn get_path() -> &'static str;
    fn make_parser() -> RequestParser<RequestResponse>;
    fn make_args(upd_item: &U) -> Self::Args;
    fn make_info(upd_item: &U) -> RequestInfo;
    fn make_body(upd_item: U) -> Self::Body;
}

impl<T, U, RequestResponse, RequestInfo> DbTableUpdate<U, RequestResponse, RequestInfo> for Table<T>
where
    T: DbTableItem,
    U: DbTableUpdateItem,
    Table<T>: TableUpdate<U, RequestResponse, RequestInfo>,
    RequestResponse: Clone,
    RequestInfo: Clone + Default,
{
    type Args = <Self as TableUpdate<U, RequestResponse, RequestInfo>>::Args;
    type Body = <Self as TableUpdate<U, RequestResponse, RequestInfo>>::Body;

    fn update_request(
        &self,
        upd_item: U,
    ) -> RequestBuilder<Self::Args, Self::Body, RequestResponse, RequestInfo> {
        RequestBuilder::new()
            .authorized()
            .with_method(Self::get_method())
            .with_path(Self::get_path())
            .with_query(Self::make_args(&upd_item))
            .with_info(Self::make_info(&upd_item))
            .with_body(Self::make_body(upd_item))
            .with_parser(Self::make_parser())
    }
}

pub trait TableDeleteById<T, RequestResponse = AppRequestResponse, RequestInfo = AppRequestInfo>
where
    T: DbTableItem,
    RequestResponse: Clone,
    RequestInfo: Clone,
{
    type Args: Serialize;

    fn get_method() -> reqwest::Method;
    fn get_path() -> &'static str;
    fn make_parser() -> RequestParser<RequestResponse>;
    fn make_args(id: T::Id) -> Self::Args;
    fn make_info(id: T::Id) -> RequestInfo;
}

impl<T, RequestResponse, RequestInfo> DbTableDelete<T, RequestResponse, RequestInfo> for Table<T>
where
    T: DbTableItem,
    Table<T>: TableDeleteById<T, RequestResponse, RequestInfo>,
    RequestResponse: Clone,
    RequestInfo: Clone + Default,
{
    type Args = <Self as TableDeleteById<T, RequestResponse, RequestInfo>>::Args;

    fn delete_by_id_request(
        &self,
        id: <T as DbTableItem>::Id,
    ) -> RequestBuilder<Self::Args, (), RequestResponse, RequestInfo> {
        RequestBuilder::new()
            .authorized()
            .with_method(Self::get_method())
            .with_path(Self::get_path())
            .with_query(Self::make_args(id))
            .with_info(Self::make_info(id))
            .with_parser(Self::make_parser())
    }
}
 */