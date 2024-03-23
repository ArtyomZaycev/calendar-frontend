use std::cell::OnceCell;

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
    pub(super) requests: RequestsHolder,
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
    pub fn load_by_id(&self, id: TableId) -> RequestIdentifier<TableLoadByIdRequest<T>> {
        let connector = DbConnectorData::get();
        let request_id = connector.next_request_id();
        let request = connector
            .make_request(
                TableLoadByIdRequest::<T>::METHOD,
                TableLoadByIdRequest::<T>::URL,
                TableLoadByIdRequest::<T>::IS_AUTHORIZED,
            )
            .query(&id);
        self.requests
            .push(RequestData::new(request_id, request.build().unwrap()));
        RequestIdentifier::new(request_id, id)
    }
}

impl<T: DbTableItem + TableItemLoadAll + DeserializeOwned> StateTable<T>
where
    State: GetStateTable<T>,
{
    pub fn load_all(&self) -> RequestIdentifier<TableLoadAllRequest<T>> {
        let connector = DbConnectorData::get();
        let request_id = connector.next_request_id();
        let request = connector.make_request(
            TableLoadAllRequest::<T>::METHOD,
            TableLoadAllRequest::<T>::URL,
            TableLoadAllRequest::<T>::IS_AUTHORIZED,
        );
        self.requests
            .push(RequestData::new(request_id, request.build().unwrap()));
        RequestIdentifier::new(request_id, ())
    }
}

impl<T> StateTable<T>
where
    T: DbTableItem + TableItemInsert + TableItemLoadAll + DeserializeOwned,
    State: GetStateTable<T>,
    reqwest::Body: From<<TableInsertRequest<T> as RequestType>::Body>,
{
    pub fn insert(
        &self,
        item: <TableInsertRequest<T> as RequestType>::Body,
    ) -> RequestIdentifier<TableInsertRequest<T>> {
        let connector = DbConnectorData::get();
        let request_id = connector.next_request_id();
        let request = connector
            .make_request(
                TableInsertRequest::<T>::METHOD,
                TableInsertRequest::<T>::URL,
                TableInsertRequest::<T>::IS_AUTHORIZED,
            )
            .body(item);
        self.requests
            .push(RequestData::new(request_id, request.build().unwrap()));
        RequestIdentifier::new(request_id, ())
    }
}

impl<T> StateTable<T>
where
    T: DbTableItem + TableItemUpdate + TableItemLoadById + DeserializeOwned,
    State: GetStateTable<T>,
    reqwest::Body: From<<TableUpdateRequest<T> as RequestType>::Body>,
{
    pub fn update(
        &self,
        item: <TableUpdateRequest<T> as RequestType>::Body,
    ) -> RequestIdentifier<TableUpdateRequest<T>> {
        let item_id = item.get_id();
        let connector = DbConnectorData::get();
        let request_id = connector.next_request_id();
        let request = connector
            .make_request(
                TableUpdateRequest::<T>::METHOD,
                TableUpdateRequest::<T>::URL,
                TableUpdateRequest::<T>::IS_AUTHORIZED,
            )
            .body(item);
        self.requests
            .push(RequestData::new(request_id, request.build().unwrap()));
        RequestIdentifier::new(request_id, item_id)
    }
}

impl<T> StateTable<T>
where
    T: DbTableItem + TableItemDelete + DeserializeOwned + TableItemLoadAll,
    State: GetStateTable<T>,
{
    fn delete(&self, id: TableId) -> RequestIdentifier<TableDeleteRequest<T>> {
        let connector = DbConnectorData::get();
        let request_id = connector.next_request_id();
        let request = connector
            .make_request(
                TableDeleteRequest::<T>::METHOD,
                TableDeleteRequest::<T>::URL,
                TableDeleteRequest::<T>::IS_AUTHORIZED,
            )
            .query(&id);
        self.requests
            .push(RequestData::new(request_id, request.build().unwrap()));
        RequestIdentifier::new(request_id, id)
    }
}
