use calendar_lib::api::utils::{DeleteByIdQuery, LoadByIdQuery};
use serde::{de::DeserializeOwned, Serialize};

use crate::tables::{table::Table, DbTableItem, DbTableUpdateItem, TableId};

use super::{
    main_state::{GetStateTable, RequestIdentifier, RequestType, State},
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
        State::make_request(id, |connector| {
            connector
                .make_request::<TableLoadByIdRequest<T>>()
                .query(&LoadByIdQuery { id })
        })
    }
}

impl<T: 'static + Send + DbTableItem + TableItemLoadAll + DeserializeOwned> StateTable<T>
where
    State: GetStateTable<T>,
{
    pub fn load_all(&self) -> RequestIdentifier<TableLoadAllRequest<T>> {
        State::make_request((), |connector| {
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
        State::make_request((), |connector| {
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
        State::make_request(item_id, |connector| {
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
    pub fn delete(&self, id: TableId) -> RequestIdentifier<TableDeleteRequest<T>> {
        State::make_request(id, |connector| {
            connector
                .make_request::<TableDeleteRequest<T>>()
                .query(&DeleteByIdQuery { id })
        })
    }
}
