pub fn get_width_from_columns(ui: &egui::Ui, num_of_columns: u32) -> f32 {
    divide_from_sections(
        ui.available_width(),
        ui.spacing().item_spacing.x,
        num_of_columns,
    )
}

pub fn get_height_from_rows(ui: &egui::Ui, num_of_rows: u32) -> f32 {
    divide_from_sections(
        ui.available_height(),
        ui.spacing().item_spacing.y,
        num_of_rows,
    )
}

pub fn get_columns_from_width(ui: &egui::Ui, width: f32) -> u32 {
    divide_from_size(ui.available_width(), width).max(1)
}

pub fn divide_from_sections(available: f32, spacing: f32, sections: u32) -> f32 {
    (available - spacing * (sections - 1) as f32) / sections as f32
}

pub fn divide_from_size(available: f32, desired: f32) -> u32 {
    (available / desired).floor() as u32
}
