use calendar_lib::api::{events::types::Event, event_templates::types::EventTemplate};

use crate::ui::table_view::TableViewItem;


impl TableViewItem for EventTemplate {
    fn get_names() -> Vec<String> {
        vec![
            "Name".to_owned(),
            "Event Name".to_owned(),
            "Event Description".to_owned(),
            "Duration".to_owned(),
        ]
    }

    fn get_fields(&self) -> Vec<String> {
        vec![
            self.name.clone(),
            self.event_name.clone(),
            self.event_description.clone().unwrap_or_default(),
            format!("{} minutes", (self.duration.as_secs() / 60)),
        ]
    }
}
