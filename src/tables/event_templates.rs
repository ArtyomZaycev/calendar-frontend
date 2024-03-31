use calendar_lib::api::event_templates::types::*;

use super::{DbTableItem, DbTableNewItem, DbTableUpdateItem, TableId};

impl DbTableItem for EventTemplate {
    fn get_id(&self) -> TableId {
        self.id
    }
}

impl DbTableNewItem for NewEventTemplate {}

impl DbTableUpdateItem for UpdateEventTemplate {
    fn get_id(&self) -> TableId {
        self.id
    }
}
