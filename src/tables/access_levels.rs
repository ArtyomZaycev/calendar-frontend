use calendar_lib::api::auth::{self, types::AccessLevel};

use super::table::*;
use crate::{
    db::{request_parser::RequestParser, table::*},
    requests::*,
    tables::utils::*,
};

impl DbTableItem for AccessLevel {
    fn get_id(&self) -> TableId {
        // To preserve uniqueness
        self.level * 2 + self.edit_rights as i32
    }
}
