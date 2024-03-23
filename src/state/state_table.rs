use std::{borrow::Borrow, cell::OnceCell};

use serde::{de::DeserializeOwned, Serialize};

use crate::tables::{
    table::{Table, TableDeleteById},
    DbTableItem, DbTableNewItem, DbTableUpdateItem, TableId,
};

use super::{
    db_connector::DbConnectorData,
    main_state::{GetStateTable, RequestIdentifier, RequestType, State},
    requests_holder::{RequestData, RequestsHolder},
    table_requests::{
        TableDeleteRequest, TableInsertRequest, TableItemDelete, TableItemInsert, TableItemLoadAll,
        TableItemLoadById, TableItemUpdate, TableLoadAllRequest, TableLoadByIdRequest,
        TableUpdateRequest,
    },
};

pub struct StateTable<T: DbTableItem> {
    data: Table<T>,
}

impl<T: DbTableItem> StateTable<T> {
    pub(super) fn new() -> Self {
        Self { data: Table::new() }
    }

    pub(super) fn from_vec(items: Vec<T>) -> Self {
        Self {
            data: Table::from_vec(items),
        }
    }

    // Hide?
    pub fn get_table(&self) -> &Table<T> {
        &self.data
    }

    pub fn get_table_mut(&mut self) -> &mut Table<T> {
        &mut self.data
    }
}

impl<T: 'static + Send + DbTableItem + TableItemLoadById + DeserializeOwned> StateTable<T>
where
    State: GetStateTable<T>,
{
    pub fn load_by_id(&self, id: TableId) -> RequestIdentifier<TableLoadByIdRequest<T>> {
        RequestsHolder::get()
            .read()
            .unwrap()
            .make_typical_request(id, |connector| {
                connector.make_request::<TableLoadByIdRequest<T>>()
            })
    }
}

impl<T: 'static + Send + DbTableItem + TableItemLoadAll + DeserializeOwned> StateTable<T>
where
    State: GetStateTable<T>,
{
    pub fn load_all(&self) -> RequestIdentifier<TableLoadAllRequest<T>> {
        RequestsHolder::get()
            .read()
            .unwrap()
            .make_typical_request((), |connector| {
                connector.make_request::<TableLoadAllRequest<T>>()
            })
    }
}

impl<T> StateTable<T>
where
    T: 'static + Send + DbTableItem + TableItemInsert + TableItemLoadAll + DeserializeOwned,
    State: GetStateTable<T>,
    <T as TableItemInsert>::NewItem: Serialize,
{
    pub fn insert(
        &self,
        item: <TableInsertRequest<T> as RequestType>::Body,
    ) -> RequestIdentifier<TableInsertRequest<T>> {
        RequestsHolder::get()
            .read()
            .unwrap()
            .make_typical_request((), |connector| {
                connector
                    .make_request::<TableInsertRequest<T>>()
                    .json(&item)
            })
    }
}

impl<T> StateTable<T>
where
    T: 'static + Send + DbTableItem + TableItemUpdate + TableItemLoadById + DeserializeOwned,
    State: GetStateTable<T>,
    <T as TableItemUpdate>::UpdItem: Serialize,
{
    pub fn update(
        &self,
        item: <TableUpdateRequest<T> as RequestType>::Body,
    ) -> RequestIdentifier<TableUpdateRequest<T>> {
        let item_id = item.get_id();
        RequestsHolder::get()
            .read()
            .unwrap()
            .make_typical_request(item_id, |connector| {
                connector
                    .make_request::<TableUpdateRequest<T>>()
                    .json(&item)
            })
    }
}

impl<T> StateTable<T>
where
    T: 'static + Send + DbTableItem + TableItemDelete + DeserializeOwned + TableItemLoadAll,
    State: GetStateTable<T>,
{
    fn delete(&self, id: TableId) -> RequestIdentifier<TableDeleteRequest<T>> {
        RequestsHolder::get()
            .read()
            .unwrap()
            .make_typical_request(id, |connector| {
                connector.make_request::<TableDeleteRequest<T>>().query(&id)
            })
    }
}
