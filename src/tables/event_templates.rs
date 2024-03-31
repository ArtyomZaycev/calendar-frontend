use calendar_lib::api::event_templates::{self, types::*};

use crate::{
    db::{request_parser::RequestParser, table::*},
    requests::*,
    tables::utils::*,
};

use super::table::*;

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
