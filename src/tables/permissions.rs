use calendar_lib::api::permissions::types::GrantedPermission;

use super::{DbTableItem, TableId};

impl DbTableItem for GrantedPermission {
    fn get_id(&self) -> TableId {
        self.id
    }
}
/*
impl DbTableNewItem for NewEvent {}

impl DbTableUpdateItem for UpdateEvent {
    fn get_id(&self) -> TableId {
        self.id
    }
}
 */
