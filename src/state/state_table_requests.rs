use calendar_lib::api::utils::*;

use crate::tables::DbTableItem;

use super::{
    state_table::StateTable,
    table_requests::{TableItemLoadAll, TableItemLoadById},
};

#[allow(unused_variables)]
impl<T: DbTableItem> StateTable<T> {
    pub(super) fn default_push_from_load_by_id(&mut self, id: TableId, item: T) {
        self.get_table_mut().push_one(item);
    }
    pub(super) fn default_push_bad_from_load_by_id(
        &mut self,
        id: TableId,
        response: LoadByIdBadRequestResponse,
    ) {
        match response {
            LoadByIdBadRequestResponse::NotFound => {
                self.get_table_mut().remove_one(id);
            }
        }
    }

    pub(super) fn default_push_from_load_all(&mut self, items: Vec<T>) {
        self.get_table_mut().replace_all(items);
    }
    pub(super) fn default_push_bad_from_load_all(&mut self) {}

    pub(super) fn default_push_from_insert(&mut self)
    where
        T: TableItemLoadAll,
    {
        self.load_all();
    }
    pub(super) fn default_push_bad_from_insert(&mut self)
    where
        T: TableItemLoadAll,
    {
        self.load_all();
    }

    pub(super) fn default_push_from_update(&mut self, id: TableId)
    where
        T: TableItemLoadById,
    {
        self.load_by_id(id);
    }
    pub(super) fn default_push_bad_from_update(
        &mut self,
        id: TableId,
        response: UpdateBadRequestResponse,
    ) where
        T: TableItemLoadById,
    {
        self.load_by_id(id);
    }

    pub(super) fn default_push_from_delete(&mut self, id: TableId) {
        self.get_table_mut().remove_one(id);
    }
    pub(super) fn default_push_bad_from_delete(
        &mut self,
        id: TableId,
        response: DeleteBadRequestResponse,
    ) where
        T: TableItemLoadById,
    {
        self.load_by_id(id);
    }
}
