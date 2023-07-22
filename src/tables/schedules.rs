use calendar_lib::api::schedules::{self, types::*};

use crate::{
    db::{table::*, request_parser::RequestParser},
    requests::*,
    tables::utils::*,
};

use super::table::*;

impl DbTableItem for Schedule {
    type Id = i32;

    fn get_id(&self) -> Self::Id {
        self.id
    }
}

impl DbTableNewItem for NewSchedule {}

impl DbTableUpdateItem for UpdateSchedule {
    type Id = i32;

    fn get_id(&self) -> Self::Id {
        self.id
    }
}

impl TableLoadById<Schedule> for Table<Schedule> {
    type Args = schedules::load::Args;

    fn get_method() -> reqwest::Method {
        schedules::load::METHOD.clone()
    }

    fn get_path() -> &'static str {
        schedules::load::PATH
    }

    fn make_parser() -> RequestParser<AppRequestResponse> {
        make_typed_bad_request_parser(
            |r| AppRequestResponse::LoadSchedule(r),
            |r| AppRequestResponse::LoadScheduleError(r),
        )
    }

    fn make_args(id: i32) -> Self::Args {
        schedules::load::Args { id }
    }

    fn make_info(id: i32) -> AppRequestInfo {
        AppRequestInfo::LoadSchedule(id)
    }
}

impl TableLoadAll<Schedule> for Table<Schedule> {
    type Args = schedules::load_array::Args;

    fn get_method() -> reqwest::Method {
        schedules::load_array::METHOD.clone()
    }

    fn get_path() -> &'static str {
        schedules::load_array::PATH
    }

    fn make_parser() -> RequestParser<AppRequestResponse> {
        make_parser(|r| AppRequestResponse::LoadSchedules(r))
    }

    fn make_args() -> Self::Args {
        schedules::load_array::Args {}
    }

    fn make_info() -> AppRequestInfo {
        AppRequestInfo::None
    }
}

impl TableInsert<NewSchedule> for Table<Schedule> {
    type Args = schedules::insert::Args;
    type Body = schedules::insert::Body;

    fn get_method() -> reqwest::Method {
        schedules::insert::METHOD.clone()
    }

    fn get_path() -> &'static str {
        schedules::insert::PATH
    }

    fn make_parser() -> RequestParser<AppRequestResponse> {
        make_parser(|r| AppRequestResponse::InsertSchedule(r))
    }

    fn make_args(_new_item: &NewSchedule) -> Self::Args {
        schedules::insert::Args {}
    }

    fn make_info(_new_item: &NewSchedule) -> AppRequestInfo {
        AppRequestInfo::None
    }

    fn make_body(new_item: NewSchedule) -> Self::Body {
        schedules::insert::Body {
            new_schedule: new_item,
        }
    }
}

impl TableUpdate<UpdateSchedule> for Table<Schedule> {
    type Args = schedules::update::Args;
    type Body = schedules::update::Body;

    fn get_method() -> reqwest::Method {
        schedules::update::METHOD.clone()
    }

    fn get_path() -> &'static str {
        schedules::update::PATH
    }

    fn make_parser() -> RequestParser<AppRequestResponse> {
        make_parser(|r| AppRequestResponse::UpdateSchedule(r))
    }

    fn make_args(_upd_item: &UpdateSchedule) -> Self::Args {
        schedules::update::Args {}
    }

    fn make_info(upd_item: &UpdateSchedule) -> AppRequestInfo {
        AppRequestInfo::UpdateSchedule(upd_item.id)
    }

    fn make_body(upd_item: UpdateSchedule) -> Self::Body {
        schedules::update::Body {
            upd_schedule: upd_item,
        }
    }
}

impl TableDeleteById<Schedule> for Table<Schedule> {
    type Args = schedules::delete::Args;

    fn get_method() -> reqwest::Method {
        schedules::delete::METHOD.clone()
    }

    fn get_path() -> &'static str {
        schedules::delete::PATH
    }

    fn make_parser() -> RequestParser<AppRequestResponse> {
        make_parser(|r| AppRequestResponse::DeleteSchedule(r))
    }

    fn make_args(id: i32) -> Self::Args {
        schedules::delete::Args { id }
    }

    fn make_info(id: i32) -> AppRequestInfo {
        AppRequestInfo::DeleteSchedule(id)
    }
}
