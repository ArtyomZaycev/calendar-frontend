use calendar_lib::api::utils::User;

use super::{DbTableItem, TableId};

impl DbTableItem for User {
    fn get_id(&self) -> TableId {
        self.id
    }
}
