use calendar_lib::api::utils::User;

use crate::ui::table_view::TableViewItem;

impl TableViewItem for User {
    fn get_names() -> Vec<String> {
        vec!["Name", "Email"]
            .into_iter()
            .map(|v| v.to_owned())
            .collect()
    }

    fn get_fields(&self) -> Vec<String> {
        vec![self.name.clone(), self.email.clone()]
    }
}
