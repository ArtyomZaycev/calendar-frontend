use crate::{
    db::{request::RequestBuilder, request_parser::RequestParser},
    requests::*,
    tables::*,
};
use serde::Serialize;

pub struct Table<T: DbTableItem> {
    items: Vec<T>,
}

impl<T: DbTableItem> Default for Table<T> {
    fn default() -> Self {
        Self {
            items: Default::default(),
        }
    }
}

impl<T: DbTableItem> Table<T> {
    pub fn new() -> Self {
        Self {
            items: Vec::default(),
        }
    }

    pub fn from_vec(items: Vec<T>) -> Self {
        Self { items }
    }

    pub fn clear(&mut self) {
        self.items.clear()
    }
}

impl<T: DbTableItem> Table<T> {
    pub fn find_item(&self, id: TableId) -> Option<&T> {
        self.items.iter().find(|i| i.get_id() == id)
    }

    pub fn find_item_mut(&mut self, id: TableId) -> Option<&mut T> {
        self.items.iter_mut().find(|i| i.get_id() == id)
    }

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
    pub fn remove_one(&mut self, id: TableId) -> Option<T> {
        self.items
            .iter()
            .position(|i| i.get_id() == id)
            .map(|ind| self.items.remove(ind))
    }
    pub fn replace_all(&mut self, new_data: Vec<T>) {
        self.items = new_data;
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
    fn make_args(id: TableId) -> Self::Args;
    fn make_info(id: TableId) -> RequestInfo;
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
        id: TableId,
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
    fn make_args(id: TableId) -> Self::Args;
    fn make_info(id: TableId) -> RequestInfo;
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
        id: TableId,
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
