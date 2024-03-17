use crate::tables::{table::Table, DbTableItem};

use super::requests_holder::RequestsHolder;

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
}
