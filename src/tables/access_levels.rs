use calendar_lib::api::auth::types::AccessLevel;

use super::{DbTableItem, TableId};

impl DbTableItem for AccessLevel {
    fn get_id(&self) -> TableId {
        // To preserve uniqueness
        self.level * 2 + self.edit_rights as i32
    }
}
