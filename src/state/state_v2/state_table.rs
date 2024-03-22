use std::cell::OnceCell;

use serde::de::DeserializeOwned;

use crate::tables::{
    table::{Table, TableDeleteById},
    DbTableItem, DbTableNewItem, DbTableUpdateItem,
};

use super::{
    main_state::{GetStateTable, RequestIdentifier, State},
    requests_holder::RequestsHolder,
    table_requests::{
        TableDeleteRequest, TableInsertRequest, TableItemDelete, TableItemInsert, TableItemLoadAll,
        TableItemLoadById, TableItemUpdate, TableLoadAllRequest, TableLoadByIdRequest,
        TableUpdateRequest,
    },
};

pub struct StateTable<T: DbTableItem> {
    data: Table<T>,
    requests: RequestsHolder,
}

impl<T: DbTableItem> StateTable<T> {
    pub(super) fn new() -> Self {
        Self {
            data: Table::new(),
            requests: RequestsHolder::new(),
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

impl<T: DbTableItem + TableItemLoadById + DeserializeOwned> StateTable<T>
where
    State: GetStateTable<T>,
{
    fn load_by_id(&self, id: T::Id) -> RequestIdentifier<TableLoadByIdRequest<T>> {
        todo!()
    }
}

impl<T: DbTableItem + TableItemLoadAll + DeserializeOwned> StateTable<T>
where
    State: GetStateTable<T>,
{
    fn load_all(&self) -> RequestIdentifier<TableLoadAllRequest<T>> {
        todo!()
    }
}

impl<T: DbTableItem + TableItemInsert> StateTable<T> where State: GetStateTable<T>, T::NewItem: DeserializeOwned {
    fn insert(&self, item: T::NewItem) -> RequestIdentifier<TableInsertRequest<T>> {
        todo!()
    }
}

impl<T: DbTableItem + TableItemUpdate> StateTable<T> where State: GetStateTable<T>, T::UpdItem: DeserializeOwned {
    fn update(&self, item: T) -> RequestIdentifier<TableUpdateRequest<T>> {
        todo!()
    }
}

impl<T: DbTableItem + TableItemDelete + DeserializeOwned> StateTable<T>
where
    State: GetStateTable<T>,
{
    fn delete(&self, id: T::Id) -> RequestIdentifier<TableDeleteRequest<T>> {
        todo!()
    }
}
