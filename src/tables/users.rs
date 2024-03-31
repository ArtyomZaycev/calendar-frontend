use calendar_lib::api::{users, utils::User};

use super::table::*;
use crate::{
    db::{request_parser::RequestParser, table::*},
    requests::*,
    tables::utils::*,
};

impl DbTableItem for User {
    fn get_id(&self) -> TableId {
        self.id
    }
}
