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

impl TableLoadAll<User> for Table<User> {
    type Args = users::load_array::Args;

    fn get_method() -> reqwest::Method {
        users::load_array::METHOD.clone()
    }

    fn get_path() -> &'static str {
        users::load_array::PATH
    }

    fn make_parser() -> RequestParser<AppRequestResponse> {
        make_parser(|r| AppRequestResponse::LoadUsers(r))
    }

    fn make_args() -> Self::Args {
        users::load_array::Args {}
    }

    fn make_info() -> AppRequestInfo {
        AppRequestInfo::None
    }
}
