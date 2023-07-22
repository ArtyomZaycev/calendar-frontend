use calendar_lib::api::event_templates::{self, types::*};

use crate::{
    db::{table::*, request_parser::RequestParser},
    requests::*,
    tables::utils::*,
};

use super::table::*;

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

impl TableLoadById<EventTemplate> for Table<EventTemplate> {
    type Args = event_templates::load::Args;

    fn get_method() -> reqwest::Method {
        event_templates::load::METHOD.clone()
    }

    fn get_path() -> &'static str {
        event_templates::load::PATH
    }

    fn make_parser() -> RequestParser<AppRequestResponse> {
        make_typed_bad_request_parser(
            |r| AppRequestResponse::LoadEventTemplate(r),
            |r| AppRequestResponse::LoadEventTemplateError(r),
        )
    }

    fn make_args(id: i32) -> Self::Args {
        event_templates::load::Args { id }
    }

    fn make_info(id: i32) -> AppRequestInfo {
        AppRequestInfo::LoadEventTemplate(id)
    }
}

impl TableLoadAll<EventTemplate> for Table<EventTemplate> {
    type Args = event_templates::load_array::Args;

    fn get_method() -> reqwest::Method {
        event_templates::load_array::METHOD.clone()
    }

    fn get_path() -> &'static str {
        event_templates::load_array::PATH
    }

    fn make_parser() -> RequestParser<AppRequestResponse> {
        make_parser(|r| AppRequestResponse::LoadEventTemplates(r))
    }

    fn make_args() -> Self::Args {
        event_templates::load_array::Args {}
    }

    fn make_info() -> AppRequestInfo {
        AppRequestInfo::None
    }
}

impl TableInsert<NewEventTemplate> for Table<EventTemplate> {
    type Args = event_templates::insert::Args;
    type Body = event_templates::insert::Body;

    fn get_method() -> reqwest::Method {
        event_templates::insert::METHOD.clone()
    }

    fn get_path() -> &'static str {
        event_templates::insert::PATH
    }

    fn make_parser() -> RequestParser<AppRequestResponse> {
        make_parser(|r| AppRequestResponse::InsertEventTemplate(r))
    }

    fn make_args(_new_item: &NewEventTemplate) -> Self::Args {
        event_templates::insert::Args {}
    }

    fn make_info(_new_item: &NewEventTemplate) -> AppRequestInfo {
        AppRequestInfo::None
    }

    fn make_body(new_item: NewEventTemplate) -> Self::Body {
        event_templates::insert::Body {
            new_event_template: new_item,
        }
    }
}

impl TableUpdate<UpdateEventTemplate> for Table<EventTemplate> {
    type Args = event_templates::update::Args;
    type Body = event_templates::update::Body;

    fn get_method() -> reqwest::Method {
        event_templates::update::METHOD.clone()
    }

    fn get_path() -> &'static str {
        event_templates::update::PATH
    }

    fn make_parser() -> RequestParser<AppRequestResponse> {
        make_parser(|r| AppRequestResponse::UpdateEventTemplate(r))
    }

    fn make_args(_upd_item: &UpdateEventTemplate) -> Self::Args {
        event_templates::update::Args {}
    }

    fn make_info(upd_item: &UpdateEventTemplate) -> AppRequestInfo {
        AppRequestInfo::UpdateEventTemplate(upd_item.id)
    }

    fn make_body(upd_item: UpdateEventTemplate) -> Self::Body {
        event_templates::update::Body {
            upd_event_template: upd_item,
        }
    }
}

impl TableDeleteById<EventTemplate> for Table<EventTemplate> {
    type Args = event_templates::delete::Args;

    fn get_method() -> reqwest::Method {
        event_templates::delete::METHOD.clone()
    }

    fn get_path() -> &'static str {
        event_templates::delete::PATH
    }

    fn make_parser() -> RequestParser<AppRequestResponse> {
        make_parser(|r| AppRequestResponse::DeleteEventTemplate(r))
    }

    fn make_args(id: i32) -> Self::Args {
        event_templates::delete::Args { id }
    }

    fn make_info(id: i32) -> AppRequestInfo {
        AppRequestInfo::DeleteEventTemplate(id)
    }
}
