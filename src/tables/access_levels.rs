use calendar_lib::api::auth::types::AccessLevel;

use super::{DbTableItem, TableId};

impl DbTableItem for AccessLevel {
    fn get_id(&self) -> TableId {
        self.id
    }
}
