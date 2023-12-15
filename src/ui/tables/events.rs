use calendar_lib::api::{events::types::Event};

use crate::ui::table_view::TableViewItem;

impl TableViewItem for Event {
    fn get_names() -> Vec<String> {
        vec!["Name", "Description", "Start", "End"]
            .into_iter()
            .map(|v| v.to_owned())
            .collect()
    }

    fn get_fields(&self) -> Vec<String> {
        vec![
            self.name.clone(),
            self.description.clone().unwrap_or_default(),
            self.start.to_string(),
            self.end.to_string()
        ]
    }
}