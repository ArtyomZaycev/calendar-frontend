use calendar_lib::api::{users, utils::User};

use super::{DbTableItem, DbTableNewItem, DbTableUpdateItem, TableId};

impl DbTableItem for User {
    fn get_id(&self) -> TableId {
        self.id
    }
}
