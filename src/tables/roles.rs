use calendar_lib::api::roles::types::Role;

use super::{DbTableItem, TableId};

impl DbTableItem for Role {
    fn get_id(&self) -> TableId {
        *self as i32
    }
}
