use calendar_lib::api::auth::{self, types::AccessLevel};

use super::table::*;
use crate::{
    db::{request_parser::RequestParser, table::*},
    requests::*,
    tables::utils::*,
};

impl DbTableItem for AccessLevel {
    type Id = i32;

    fn get_id(&self) -> Self::Id {
        // To preserve uniqueness
        self.level * 2 + self.edit_rights as i32
    }
}

impl TableLoadAll<AccessLevel> for Table<AccessLevel> {
    type Args = auth::load_access_levels::Args;

    fn get_method() -> reqwest::Method {
        auth::load_access_levels::METHOD.clone()
    }

    fn get_path() -> &'static str {
        auth::load_access_levels::PATH
    }

    fn make_parser() -> RequestParser<AppRequestResponse> {
        make_parser(|r| AppRequestResponse::LoadAccessLevels(r))
    }

    fn make_args() -> Self::Args {
        auth::load_access_levels::Args {}
    }

    fn make_info() -> AppRequestInfo {
        AppRequestInfo::None
    }
}
