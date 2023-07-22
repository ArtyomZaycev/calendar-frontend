use calendar_lib::api::events::{self, types::*};

use super::table::*;
use crate::{
    db::{request_parser::RequestParser, table::*},
    requests::*,
    tables::utils::*,
};

impl DbTableItem for Event {
    type Id = i32;

    fn get_id(&self) -> Self::Id {
        self.id
    }
}

impl DbTableNewItem for NewEvent {}

impl DbTableUpdateItem for UpdateEvent {
    type Id = i32;

    fn get_id(&self) -> Self::Id {
        self.id
    }
}

impl TableLoadById<Event> for Table<Event> {
    type Args = events::load::Args;

    fn get_method() -> reqwest::Method {
        events::load::METHOD.clone()
    }

    fn get_path() -> &'static str {
        events::load::PATH
    }

    fn make_parser() -> RequestParser<AppRequestResponse> {
        make_typed_bad_request_parser(
            |r| AppRequestResponse::LoadEvent(r),
            |r| AppRequestResponse::LoadEventError(r),
        )
    }

    fn make_args(id: i32) -> Self::Args {
        events::load::Args { id }
    }

    fn make_info(id: i32) -> AppRequestInfo {
        AppRequestInfo::LoadEvent(id)
    }
}

impl TableLoadAll<Event> for Table<Event> {
    type Args = events::load_array::Args;

    fn get_method() -> reqwest::Method {
        events::load_array::METHOD.clone()
    }

    fn get_path() -> &'static str {
        events::load_array::PATH
    }

    fn make_parser() -> RequestParser<AppRequestResponse> {
        make_parser(|r| AppRequestResponse::LoadEvents(r))
    }

    fn make_args() -> Self::Args {
        events::load_array::Args {}
    }

    fn make_info() -> AppRequestInfo {
        AppRequestInfo::None
    }
}

impl TableInsert<NewEvent> for Table<Event> {
    type Args = events::insert::Args;
    type Body = events::insert::Body;

    fn get_method() -> reqwest::Method {
        events::insert::METHOD.clone()
    }

    fn get_path() -> &'static str {
        events::insert::PATH
    }

    fn make_parser() -> RequestParser<AppRequestResponse> {
        make_parser(|r| AppRequestResponse::InsertEvent(r))
    }

    fn make_args(_new_item: &NewEvent) -> Self::Args {
        events::insert::Args {}
    }

    fn make_info(_new_item: &NewEvent) -> AppRequestInfo {
        AppRequestInfo::None
    }

    fn make_body(new_item: NewEvent) -> Self::Body {
        events::insert::Body {
            new_event: new_item,
        }
    }
}

impl TableUpdate<UpdateEvent> for Table<Event> {
    type Args = events::update::Args;
    type Body = events::update::Body;

    fn get_method() -> reqwest::Method {
        events::update::METHOD.clone()
    }

    fn get_path() -> &'static str {
        events::update::PATH
    }

    fn make_parser() -> RequestParser<AppRequestResponse> {
        make_parser(|r| AppRequestResponse::UpdateEvent(r))
    }

    fn make_args(_upd_item: &UpdateEvent) -> Self::Args {
        events::update::Args {}
    }

    fn make_info(upd_item: &UpdateEvent) -> AppRequestInfo {
        AppRequestInfo::UpdateEvent(upd_item.id)
    }

    fn make_body(upd_item: UpdateEvent) -> Self::Body {
        events::update::Body {
            upd_event: upd_item,
        }
    }
}

impl TableDeleteById<Event> for Table<Event> {
    type Args = events::delete::Args;

    fn get_method() -> reqwest::Method {
        events::delete::METHOD.clone()
    }

    fn get_path() -> &'static str {
        events::delete::PATH
    }

    fn make_parser() -> RequestParser<AppRequestResponse> {
        make_parser(|r| AppRequestResponse::DeleteEvent(r))
    }

    fn make_args(id: i32) -> Self::Args {
        events::delete::Args { id }
    }

    fn make_info(id: i32) -> AppRequestInfo {
        AppRequestInfo::DeleteEvent(id)
    }
}
