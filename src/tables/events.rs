use calendar_lib::api::events::{self, types::*};

use super::table::*;
use crate::{
    db::{request_parser::RequestParser, table::*},
    requests::*,
    tables::utils::*,
};

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
