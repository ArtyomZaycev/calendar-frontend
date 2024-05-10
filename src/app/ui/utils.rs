use super::super::{utils::AppView, CalendarApp};

impl CalendarApp {
    pub(super) fn set_view(&mut self, view: impl Into<AppView>) {
        self.view = view.into();
    }

    pub fn configure_styles(ctx: &egui::Context) {
        /*
           Default:
           {
               Small: FontId { size: 9.0, family: Proportional },
               Body: FontId { size: 12.5, family: Proportional },
               Monospace: FontId { size: 12.0, family: Monospace },
               Button: FontId { size: 12.5, family: Proportional },
               Heading: FontId { size: 18.0, family: Proportional }
           }
        */

        let mut style = (*ctx.style()).clone();
        style.text_styles = [
            (
                egui::TextStyle::Small,
                egui::FontId::new(11.0, egui::FontFamily::Proportional),
            ),
            (
                egui::TextStyle::Body,
                egui::FontId::new(14.0, egui::FontFamily::Proportional),
            ),
            (
                egui::TextStyle::Monospace,
                egui::FontId::new(12.0, egui::FontFamily::Proportional),
            ),
            (
                egui::TextStyle::Button,
                egui::FontId::new(14.0, egui::FontFamily::Proportional),
            ),
            (
                egui::TextStyle::Heading,
                egui::FontId::new(22.0, egui::FontFamily::Proportional),
            ),
        ]
        .into();
        style.interaction.selectable_labels = false;
        ctx.set_style(style);
    }
}
