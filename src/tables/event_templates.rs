use calendar_lib::api::event_templates::{self, types::*};

use crate::{
    db::{request::RequestBuilder, table::*},
    requests::*,
    tables::utils::*,
};

impl DbTableItem for EventTemplate {
    type Id = i32;

    fn get_id(&self) -> Self::Id {
        self.id
    }
}

impl DbTableNewItem for NewEventTemplate {}

impl DbTableUpdateItem for UpdateEventTemplate {
    type Id = i32;

    fn get_id(&self) -> Self::Id {
        self.id
    }
}

#[derive(Default)]
pub struct EventTemplates {
    event_templates: Vec<EventTemplate>,
}

impl EventTemplates {
    pub fn new() -> Self {
        Self {
            event_templates: Vec::default(),
        }
    }

    pub fn clear(&mut self) {
        self.event_templates.clear();
    }
}

impl From<Vec<EventTemplate>> for EventTemplates {
    fn from(value: Vec<EventTemplate>) -> Self {
        Self { event_templates: value }
    }
}

impl DbTable<EventTemplate> for EventTemplates {
    fn get(&self) -> &Vec<EventTemplate> {
        &self.event_templates
    }

    fn get_mut(&mut self) -> &mut Vec<EventTemplate> {
        &mut self.event_templates
    }
}

impl DbTableLoadAll<EventTemplate> for EventTemplates {
    type Args = event_templates::load_array::Args;

    fn load_all() -> RequestBuilder<Self::Args, ()> {
        use event_templates::load_array::*;

        let parser = make_parser(|r| AppRequestResponse::LoadEventTemplates(r));

        RequestBuilder::new()
            .authorized()
            .with_method(METHOD.clone())
            .with_path(PATH)
            .with_query(Args {})
            .with_parser(parser)
    }
}

impl DbTableLoad<EventTemplate> for EventTemplates {
    type Args = event_templates::load::Args;

    fn load_by_id(id: i32) -> RequestBuilder<Self::Args, ()> {
        use event_templates::load::*;

        let parser = make_typed_bad_request_parser(
            |r| AppRequestResponse::LoadEventTemplate(r),
            |r| AppRequestResponse::LoadEventTemplateError(r),
        );

        RequestBuilder::new()
            .authorized()
            .with_method(METHOD.clone())
            .with_path(PATH)
            .with_query(Args { id })
            .with_info(AppRequestInfo::LoadEventTemplate(id))
            .with_parser(parser)
    }
}

impl DbTableInsert<NewEventTemplate> for EventTemplates {
    type Args = event_templates::insert::Args;
    type Body = event_templates::insert::Body;

    fn insert(new_event_template: NewEventTemplate) -> RequestBuilder<Self::Args, Self::Body> {
        use event_templates::insert::*;

        let parser = make_parser(|r| AppRequestResponse::InsertEventTemplate(r));
        RequestBuilder::new()
            .authorized()
            .with_method(METHOD.clone())
            .with_path(PATH)
            .with_query(Args {})
            .with_body(Body { new_event_template })
            .with_parser(parser)
    }
}

impl DbTableUpdate<UpdateEventTemplate> for EventTemplates {
    type Args = event_templates::update::Args;
    type Body = event_templates::update::Body;

    fn update(upd_event_template: UpdateEventTemplate) -> RequestBuilder<Self::Args, Self::Body> {
        use event_templates::update::*;

        let id = upd_event_template.id;
        let parser = make_parser(|r| AppRequestResponse::UpdateEventTemplate(r));
        RequestBuilder::new()
            .authorized()
            .with_method(METHOD.clone())
            .with_path(PATH)
            .with_query(Args {})
            .with_body(Body { upd_event_template })
            .with_info(AppRequestInfo::UpdateEventTemplate(id))
            .with_parser(parser)
    }
}

impl DbTableDelete<EventTemplate> for EventTemplates {
    type Args = event_templates::delete::Args;

    fn delete_by_id(id: i32) -> RequestBuilder<Self::Args, ()> {
        use event_templates::delete::*;

        let parser = make_parser(|r| AppRequestResponse::DeleteEventTemplate(r));
        RequestBuilder::new()
            .authorized()
            .with_method(METHOD.clone())
            .with_path(PATH)
            .with_query(Args { id })
            .with_info(AppRequestInfo::DeleteEventTemplate(id))
            .with_parser(parser)
    }
}
