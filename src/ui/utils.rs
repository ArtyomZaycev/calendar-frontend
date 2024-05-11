use egui::{Response, RichText, Sense, Ui};

pub trait UiUtils {
    fn enabled_selectable_rich_text<F: FnOnce()>(
        &mut self,
        text: RichText,
        is_enabled: bool,
        is_selected: bool,
        on_clicked: F,
    ) -> Response;
    fn enabled_selectable_header<F: FnOnce()>(
        &mut self,
        text: &str,
        is_enabled: bool,
        is_selected: bool,
        on_clicked: F,
    ) -> Response;
    fn selectable_header<F: FnOnce()>(
        &mut self,
        text: &str,
        is_selected: bool,
        on_clicked: F,
    ) -> Response;
}

impl UiUtils for Ui {
    fn enabled_selectable_rich_text<F: FnOnce()>(
        &mut self,
        mut text: RichText,
        is_enabled: bool,
        is_selected: bool,
        on_clicked: F,
    ) -> Response {
        if is_selected {
            text = text.underline()
        }

        let response = self.add_enabled(is_enabled, egui::Label::new(text).sense(Sense::click()));

        if response.clicked() {
            on_clicked();
        }

        response
    }

    fn enabled_selectable_header<F: FnOnce()>(
        &mut self,
        text: &str,
        is_enabled: bool,
        is_selected: bool,
        on_clicked: F,
    ) -> Response {
        self.enabled_selectable_rich_text(RichText::new(text).heading(), is_enabled, is_selected, on_clicked)
    }

    fn selectable_header<F: FnOnce()>(
        &mut self,
        text: &str,
        is_selected: bool,
        on_clicked: F,
    ) -> Response {
        self.enabled_selectable_header(text, true, is_selected, on_clicked)
    }
}
