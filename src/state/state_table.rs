use calendar_lib::api::utils::{DeleteByIdQuery, LoadArrayQuery, LoadByIdQuery};

use crate::{
    db::request::RequestIdentifier,
    tables::{table::Table, DbTableItem, DbTableUpdateItem, TableId},
};

use super::{
    request::{make_state_request, RequestType},
    table_requests::{
        StateRequestInfo, TableDeleteRequest, TableInsertRequest, TableItemDelete, TableItemInsert,
        TableItemLoadAll, TableItemLoadById, TableItemUpdate, TableLoadAllRequest,
        TableLoadByIdRequest, TableUpdateRequest,
    },
};

pub struct StateTable<T: DbTableItem> {
    user_id: TableId, // Propagated from UserState
    data: Table<T>,
}

impl<T: DbTableItem> StateTable<T> {
    pub(super) fn new() -> Self {
        Self {
            user_id: -1,
            data: Table::new(),
        }
    }

    pub(super) fn set_user_id(&mut self, user_id: TableId) {
        self.user_id = user_id;
    }

    // Hide?
    pub fn get_table(&self) -> &Table<T> {
        &self.data
    }

    pub fn get_table_mut(&mut self) -> &mut Table<T> {
        &mut self.data
    }
}

impl<T: TableItemLoadById> StateTable<T> {
    pub fn load_by_id(&self, id: TableId) -> RequestIdentifier<TableLoadByIdRequest<T>> {
        make_state_request(StateRequestInfo::new(self.user_id, id), |connector| {
            connector
                .make_request::<TableLoadByIdRequest<T>>()
                .query(&LoadByIdQuery { id })
        })
    }
}

impl<T: TableItemLoadAll> StateTable<T> {
    pub fn load_all(&self) -> RequestIdentifier<TableLoadAllRequest<T>> {
        make_state_request(StateRequestInfo::new_default(self.user_id), |connector| {
            connector
                .make_request::<TableLoadAllRequest<T>>()
                .query(&LoadArrayQuery {
                    user_id: self.user_id,
                })
        })
    }
}

impl<T: TableItemInsert> StateTable<T> {
    pub fn insert(
        &self,
        item: <TableInsertRequest<T> as RequestType>::Body,
    ) -> RequestIdentifier<TableInsertRequest<T>>
    where
        T::Info: Default,
    {
        make_state_request(StateRequestInfo::new_default(self.user_id), |connector| {
            connector
                .make_request::<TableInsertRequest<T>>()
                .json(&item)
        })
    }

    pub fn insert_with_info(
        &self,
        item: <TableInsertRequest<T> as RequestType>::Body,
        info: T::Info,
    ) -> RequestIdentifier<TableInsertRequest<T>> {
        make_state_request(StateRequestInfo::new(self.user_id, info), |connector| {
            connector
                .make_request::<TableInsertRequest<T>>()
                .json(&item)
        })
    }
}

impl<T: TableItemUpdate> StateTable<T> {
    pub fn update(
        &self,
        item: <TableUpdateRequest<T> as RequestType>::Body,
    ) -> RequestIdentifier<TableUpdateRequest<T>>
    where
        T::Info: Default,
    {
        let item_id = item.get_id();
        make_state_request(
            StateRequestInfo::new(self.user_id, (item_id, T::Info::default())),
            |connector| {
                connector
                    .make_request::<TableUpdateRequest<T>>()
                    .json(&item)
            },
        )
    }

    pub fn update_with_info(
        &self,
        item: <TableUpdateRequest<T> as RequestType>::Body,
        info: T::Info,
    ) -> RequestIdentifier<TableUpdateRequest<T>> {
        let item_id = item.get_id();
        make_state_request(
            StateRequestInfo::new(self.user_id, (item_id, info)),
            |connector| {
                connector
                    .make_request::<TableUpdateRequest<T>>()
                    .json(&item)
            },
        )
    }
}

impl<T: TableItemDelete> StateTable<T> {
    pub fn delete(&self, id: TableId) -> RequestIdentifier<TableDeleteRequest<T>> {
        make_state_request(StateRequestInfo::new(self.user_id, id), |connector| {
            connector
                .make_request::<TableDeleteRequest<T>>()
                .query(&DeleteByIdQuery { id })
        })
    }
}
