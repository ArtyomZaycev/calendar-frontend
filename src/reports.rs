use calendar_lib::api::{utils::User, other::types::UserMemoryUsageData};
use genpdf::{Alignment, elements::{Paragraph, Break}};

pub fn create_memory_usage_report(
    user: &User,
    data: UserMemoryUsageData
) {
    // Load a font from the file system
    let font_family = genpdf::fonts::from_files("./assets", "CourierPrime", None)
    .expect("Failed to load font family");
    // Create a document and set the default font family
    let mut doc = genpdf::Document::new(font_family);
    // Change the default settings
    doc.set_title("User Memory Usage Report");
    // Customize the pages
    let mut decorator = genpdf::SimplePageDecorator::new();
    decorator.set_margins(10);
    doc.set_page_decorator(decorator);
    // Add one or more elements
    doc.push(Paragraph::new(format!("Memory Usage Report for user {}", user.name)).aligned(Alignment::Center));
    doc.push(Break::new(1.5));
    doc.push(Paragraph::new(format!("Amount of Events: {}", data.events_count)));
    doc.push(Paragraph::new(format!("Amount of Passwords: {}", data.passwords_count)));
    doc.push(Paragraph::new(format!("Amount of Schedules: {}", data.schedules_count)));
    doc.push(Paragraph::new(format!("Amount of Event Plans: {}", data.event_plans_count)));
    doc.push(Paragraph::new(format!("Amount of Event Templates: {}", data.event_templates_count)));
    doc.push(Paragraph::new(format!("Predicted storage usage: {} bytes", data.bytes)));
    // Render the document and write it to a file
    doc.render_to_file(format!("./reports/user_memory_usage_report_{}.pdf", user.name)).expect("Failed to write PDF file");
}