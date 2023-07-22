use calendar_lib::api::events::{self, types::*};

use crate::{
    db::{request::RequestBuilder, table::*},
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

#[derive(Default)]
pub struct Events {
    events: Vec<Event>,
}

impl Events {
    pub fn new() -> Self {
        Self {
            events: Vec::default(),
        }
    }

    pub fn clear(&mut self) {
        self.events.clear();
    }
}

impl From<Vec<Event>> for Events {
    fn from(value: Vec<Event>) -> Self {
        Self { events: value }
    }
}

impl DbTable<Event> for Events {
    fn get(&self) -> &Vec<Event> {
        &self.events
    }

    fn get_mut(&mut self) -> &mut Vec<Event> {
        &mut self.events
    }
}

impl DbTableLoadAll<Event> for Events {
    type Args = events::load_array::Args;

    fn load_all() -> RequestBuilder<Self::Args, ()> {
        use events::load_array::*;

        let parser = make_parser(|r| AppRequestResponse::LoadEvents(r));

        RequestBuilder::new()
            .authorized()
            .with_method(METHOD.clone())
            .with_path(PATH)
            .with_query(Args {})
            .with_parser(parser)
    }
}

impl DbTableLoad<Event> for Events {
    type Args = events::load::Args;

    fn load_by_id(id: i32) -> RequestBuilder<Self::Args, ()> {
        use events::load::*;

        let parser = make_typed_bad_request_parser(
            |r| AppRequestResponse::LoadEvent(r),
            |r| AppRequestResponse::LoadEventError(r),
        );

        RequestBuilder::new()
            .authorized()
            .with_method(METHOD.clone())
            .with_path(PATH)
            .with_query(Args { id })
            .with_info(AppRequestInfo::LoadEvent(id))
            .with_parser(parser)
    }
}

impl DbTableInsert<NewEvent> for Events {
    type Args = events::insert::Args;
    type Body = events::insert::Body;

    fn insert(new_event: NewEvent) -> RequestBuilder<Self::Args, Self::Body> {
        use events::insert::*;

        let parser = make_parser(|r| AppRequestResponse::InsertEvent(r));
        RequestBuilder::new()
            .authorized()
            .with_method(METHOD.clone())
            .with_path(PATH)
            .with_query(Args {})
            .with_body(Body { new_event })
            .with_parser(parser)
    }
}

impl DbTableUpdate<UpdateEvent> for Events {
    type Args = events::update::Args;
    type Body = events::update::Body;

    fn update(upd_event: UpdateEvent) -> RequestBuilder<Self::Args, Self::Body> {
        use events::update::*;

        let id = upd_event.id;
        let parser = make_parser(|r| AppRequestResponse::UpdateEvent(r));
        RequestBuilder::new()
            .authorized()
            .with_method(METHOD.clone())
            .with_path(PATH)
            .with_query(Args {})
            .with_body(Body { upd_event })
            .with_info(AppRequestInfo::UpdateEvent(id))
            .with_parser(parser)
    }
}

impl DbTableDelete<Event> for Events {
    type Args = events::delete::Args;

    fn delete_by_id(id: i32) -> RequestBuilder<Self::Args, ()> {
        use events::delete::*;

        let parser = make_parser(|r| AppRequestResponse::DeleteEvent(r));
        RequestBuilder::new()
            .authorized()
            .with_method(METHOD.clone())
            .with_path(PATH)
            .with_query(Args { id })
            .with_info(AppRequestInfo::DeleteEvent(id))
            .with_parser(parser)
    }
}
