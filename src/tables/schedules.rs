use calendar_lib::api::schedules::types::*;

use super::{DbTableItem, DbTableNewItem, DbTableUpdateItem, TableId};

impl DbTableItem for Schedule {
    fn get_id(&self) -> TableId {
        self.id
    }
}

impl DbTableNewItem for NewSchedule {}

impl DbTableUpdateItem for UpdateSchedule {
    fn get_id(&self) -> TableId {
        self.id
    }
}
