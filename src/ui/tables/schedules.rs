use calendar_lib::api::schedules::types::Schedule;

use crate::ui::table_view::TableViewItem;

impl TableViewItem for Schedule {
    fn get_names() -> Vec<String> {
        vec![
            "Name".to_owned(),
            "Description".to_owned(),
            "First Day".to_owned(),
            "Last Day".to_owned(),
        ]
    }

    fn get_fields(&self) -> Vec<String> {
        vec![
            self.name.clone(),
            self.description.clone().unwrap_or_default(),
            self.first_day.to_string(),
            match self.last_day {
                Some(last_day) => last_day.to_string(),
                None => "None".to_owned(),
            },
        ]
    }
}
