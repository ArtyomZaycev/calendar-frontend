pub struct GridLayoutInfo {
    pub num_of_columns: u32,
    pub column_width: f32,
}

impl GridLayoutInfo {
    pub fn from_columns(ui: &egui::Ui, num_of_columns: u32) -> Self {
        Self {
            num_of_columns,
            column_width: (ui.available_width()
            - ui.spacing().item_spacing.x * (num_of_columns - 1) as f32)
            / num_of_columns as f32
        }
    }

    pub fn from_desired_width(ui: &egui::Ui, desired_width: f32) -> Self {
        let num_of_columns = (ui.available_width() / desired_width).round() as u32;
        Self::from_columns(ui, num_of_columns)
    }
}