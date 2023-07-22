use calendar_lib::api::schedules::{self, types::*};

use crate::{
    db::{request::RequestBuilder, table::*},
    requests::*,
    tables::utils::*,
};

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

#[derive(Default)]
pub struct Schedules {
    schedules: Vec<Schedule>,
}

impl Schedules {
    pub fn new() -> Self {
        Self {
            schedules: Vec::default(),
        }
    }

    pub fn clear(&mut self) {
        self.schedules.clear();
    }
}

impl From<Vec<Schedule>> for Schedules {
    fn from(value: Vec<Schedule>) -> Self {
        Self { schedules: value }
    }
}

impl DbTable<Schedule> for Schedules {
    fn get(&self) -> &Vec<Schedule> {
        &self.schedules
    }

    fn get_mut(&mut self) -> &mut Vec<Schedule> {
        &mut self.schedules
    }
}

impl DbTableLoadAll<Schedule> for Schedules {
    type Args = schedules::load_array::Args;

    fn load_all() -> RequestBuilder<Self::Args, ()> {
        use schedules::load_array::*;

        let parser = make_parser(|r| AppRequestResponse::LoadSchedules(r));

        RequestBuilder::new()
            .authorized()
            .with_method(METHOD.clone())
            .with_path(PATH)
            .with_query(Args {})
            .with_parser(parser)
    }
}

impl DbTableLoad<Schedule> for Schedules {
    type Args = schedules::load::Args;

    fn load_by_id(id: i32) -> RequestBuilder<Self::Args, ()> {
        use schedules::load::*;

        let parser = make_typed_bad_request_parser(
            |r| AppRequestResponse::LoadSchedule(r),
            |r| AppRequestResponse::LoadScheduleError(r),
        );

        RequestBuilder::new()
            .authorized()
            .with_method(METHOD.clone())
            .with_path(PATH)
            .with_query(Args { id })
            .with_info(AppRequestInfo::LoadSchedule(id))
            .with_parser(parser)
    }
}

impl DbTableInsert<NewSchedule> for Schedules {
    type Args = schedules::insert::Args;
    type Body = schedules::insert::Body;

    fn insert(new_schedule: NewSchedule) -> RequestBuilder<Self::Args, Self::Body> {
        use schedules::insert::*;

        let parser = make_parser(|r| AppRequestResponse::InsertSchedule(r));
        RequestBuilder::new()
            .authorized()
            .with_method(METHOD.clone())
            .with_path(PATH)
            .with_query(Args {})
            .with_body(Body { new_schedule })
            .with_parser(parser)
    }
}

impl DbTableUpdate<UpdateSchedule> for Schedules {
    type Args = schedules::update::Args;
    type Body = schedules::update::Body;

    fn update(upd_schedule: UpdateSchedule) -> RequestBuilder<Self::Args, Self::Body> {
        use schedules::update::*;

        let id = upd_schedule.id;
        let parser = make_parser(|r| AppRequestResponse::UpdateSchedule(r));
        RequestBuilder::new()
            .authorized()
            .with_method(METHOD.clone())
            .with_path(PATH)
            .with_query(Args {})
            .with_body(Body { upd_schedule })
            .with_info(AppRequestInfo::UpdateSchedule(id))
            .with_parser(parser)
    }
}

impl DbTableDelete<Schedule> for Schedules {
    type Args = schedules::delete::Args;

    fn delete_by_id(id: i32) -> RequestBuilder<Self::Args, ()> {
        use schedules::delete::*;

        let parser = make_parser(|r| AppRequestResponse::DeleteSchedule(r));
        RequestBuilder::new()
            .authorized()
            .with_method(METHOD.clone())
            .with_path(PATH)
            .with_query(Args { id })
            .with_info(AppRequestInfo::DeleteSchedule(id))
            .with_parser(parser)
    }
}
