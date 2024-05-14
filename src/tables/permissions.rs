use calendar_lib::api::permissions::types::*;

use super::{DbTableItem, DbTableNewItem, DbTableUpdateItem, TableId};

impl DbTableItem for GrantedPermission {
    fn get_id(&self) -> TableId {
        self.id
    }
}

impl DbTableNewItem for NewGrantedPermission {}

impl DbTableUpdateItem for UpdateGrantedPermission {
    fn get_id(&self) -> TableId {
        self.id
    }
}
