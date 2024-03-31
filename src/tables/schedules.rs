use calendar_lib::api::schedules::{self, types::*};

use crate::{
    db::{request_parser::RequestParser, table::*},
    requests::*,
    tables::utils::*,
};

use super::table::*;

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
