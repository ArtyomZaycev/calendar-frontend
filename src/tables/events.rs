use calendar_lib::api::events::types::*;

use super::{DbTableItem, DbTableNewItem, DbTableUpdateItem, TableId};

impl DbTableItem for Event {
    fn get_id(&self) -> TableId {
        self.id
    }
}

impl DbTableNewItem for NewEvent {}

impl DbTableUpdateItem for UpdateEvent {
    fn get_id(&self) -> TableId {
        self.id
    }
}
